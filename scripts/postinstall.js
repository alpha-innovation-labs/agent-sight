#!/usr/bin/env node

import {
  chmodSync,
  createWriteStream,
  existsSync,
  lstatSync,
  mkdirSync,
  readFileSync,
  symlinkSync,
  unlinkSync,
  writeFileSync,
} from 'fs';
import { execSync } from 'child_process';
import { get } from 'https';
import { arch, platform } from 'os';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';

const CLI_NAME = 'agent-sight';
const __dirname = dirname(fileURLToPath(import.meta.url));
const projectRoot = join(__dirname, '..');
const binDir = join(projectRoot, 'bin');
const packageJson = JSON.parse(readFileSync(join(projectRoot, 'package.json'), 'utf8'));

function isMusl() {
  if (platform() !== 'linux') {
    return false;
  }

  try {
    const output = execSync('ldd --version 2>&1 || true', { encoding: 'utf8' });
    return output.toLowerCase().includes('musl');
  } catch {
    return existsSync('/lib/ld-musl-x86_64.so.1') || existsSync('/lib/ld-musl-aarch64.so.1');
  }
}

function getBinaryName() {
  let osKey;
  switch (platform()) {
    case 'darwin':
      osKey = 'darwin';
      break;
    case 'linux':
      osKey = isMusl() ? 'linux-musl' : 'linux';
      break;
    case 'win32':
      osKey = 'win32';
      break;
    default:
      return null;
  }

  let archKey;
  switch (arch()) {
    case 'x64':
    case 'x86_64':
      archKey = 'x64';
      break;
    case 'arm64':
    case 'aarch64':
      archKey = 'arm64';
      break;
    default:
      return null;
  }

  return `${CLI_NAME}-${osKey}-${archKey}${platform() === 'win32' ? '.exe' : ''}`;
}

function getRepositorySlug() {
  const repository = typeof packageJson.repository === 'string'
    ? packageJson.repository
    : packageJson.repository?.url;

  if (!repository) {
    return null;
  }

  const match = repository.match(/github\.com[:/](.+?)\/?(?:\.git)?$/);
  return match?.[1] ?? null;
}

function writeInstallMethod() {
  const userAgent = process.env.npm_config_user_agent || '';
  let method = '';

  if (userAgent.startsWith('pnpm/')) {
    method = 'pnpm';
  } else if (userAgent.startsWith('yarn/')) {
    method = 'yarn';
  } else if (userAgent.startsWith('bun/')) {
    method = 'bun';
  } else if (userAgent.startsWith('npm/')) {
    method = 'npm';
  }

  if (method) {
    try {
      writeFileSync(join(binDir, '.install-method'), method);
    } catch {
      // Best effort only.
    }
  }
}

function downloadFile(url, destinationPath) {
  return new Promise((resolve, reject) => {
    const file = createWriteStream(destinationPath);

    const request = (currentUrl) => {
      get(currentUrl, (response) => {
        if (response.statusCode === 301 || response.statusCode === 302) {
          request(response.headers.location);
          return;
        }

        if (response.statusCode !== 200) {
          reject(new Error(`Failed to download: HTTP ${response.statusCode}`));
          return;
        }

        response.pipe(file);
        file.on('finish', () => {
          file.close();
          resolve();
        });
      }).on('error', (error) => {
        try {
          unlinkSync(destinationPath);
        } catch {
          // Ignore cleanup failures.
        }
        reject(error);
      });
    };

    request(url);
  });
}

async function fixUnixSymlink(binaryPath) {
  let npmBinDir;
  try {
    const prefix = execSync('npm prefix -g', { encoding: 'utf8' }).trim();
    npmBinDir = join(prefix, 'bin');
  } catch {
    return;
  }

  const symlinkPath = join(npmBinDir, CLI_NAME);

  try {
    if (!lstatSync(symlinkPath).isSymbolicLink()) {
      return;
    }
  } catch {
    return;
  }

  try {
    unlinkSync(symlinkPath);
    symlinkSync(binaryPath, symlinkPath);
    console.log('Optimized global symlink to point at the native binary');
  } catch (error) {
    console.log(`Could not optimize symlink: ${error.message}`);
  }
}

async function fixWindowsShims(binaryName) {
  let npmBinDir;
  try {
    npmBinDir = execSync('npm prefix -g', { encoding: 'utf8' }).trim();
  } catch {
    return;
  }

  const cmdShim = join(npmBinDir, `${CLI_NAME}.cmd`);
  const ps1Shim = join(npmBinDir, `${CLI_NAME}.ps1`);
  if (!existsSync(cmdShim)) {
    return;
  }

  const relativeBinaryPath = `node_modules\\${packageJson.name}\\bin\\${binaryName}`;
  const absoluteBinaryPath = join(npmBinDir, relativeBinaryPath);
  if (!existsSync(absoluteBinaryPath)) {
    return;
  }

  try {
    writeFileSync(cmdShim, `@ECHO off\r\n"%~dp0${relativeBinaryPath}" %*\r\n`);
    writeFileSync(
      ps1Shim,
      `#!/usr/bin/env pwsh\r\n$basedir = Split-Path $MyInvocation.MyCommand.Definition -Parent\r\n& "$basedir\\${relativeBinaryPath}" $args\r\nexit $LASTEXITCODE\r\n`,
    );
    console.log('Optimized Windows shims to point at the native binary');
  } catch (error) {
    console.log(`Could not optimize shims: ${error.message}`);
  }
}

async function fixGlobalInstallBin(binaryPath, binaryName) {
  if (!existsSync(binaryPath)) {
    return;
  }

  if (platform() === 'win32') {
    await fixWindowsShims(binaryName);
    return;
  }

  await fixUnixSymlink(binaryPath);
}

async function main() {
  const binaryName = getBinaryName();
  if (!binaryName) {
    console.log(`Skipping native install for unsupported platform ${platform()}-${arch()}`);
    return;
  }

  const binaryPath = join(binDir, binaryName);

  if (existsSync(binaryPath)) {
    if (platform() !== 'win32') {
      chmodSync(binaryPath, 0o755);
    }
    writeInstallMethod();
    await fixGlobalInstallBin(binaryPath, binaryName);
    console.log(`Native binary ready: ${binaryName}`);
    return;
  }

  mkdirSync(binDir, { recursive: true });

  const slug = getRepositorySlug();
  if (!slug) {
    console.log('No GitHub repository configured, skipping native binary download.');
    return;
  }

  const downloadUrl = `https://github.com/${slug}/releases/download/v${packageJson.version}/${binaryName}`;
  console.log(`Downloading native binary for ${platform()}-${arch()} from ${downloadUrl}`);

  try {
    await downloadFile(downloadUrl, binaryPath);
    if (platform() !== 'win32') {
      chmodSync(binaryPath, 0o755);
    }
    console.log(`Downloaded native binary: ${binaryName}`);
  } catch (error) {
    console.log(`Could not download native binary: ${error.message}`);
    console.log('Build it locally with `npm run build:native`.');
  }

  writeInstallMethod();
  await fixGlobalInstallBin(binaryPath, binaryName);
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
