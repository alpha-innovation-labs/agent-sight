#!/usr/bin/env node

import { spawn, execSync } from 'child_process';
import { accessSync, chmodSync, constants, existsSync } from 'fs';
import { arch, platform } from 'os';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';

const CLI_NAME = 'agent-sight';
const __dirname = dirname(fileURLToPath(import.meta.url));

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
    case 'arm64':
    case 'aarch64':
      archKey = 'arm64';
      break;
    case 'x64':
    case 'x86_64':
      archKey = 'x64';
      break;
    default:
      return null;
  }

  return `${CLI_NAME}-${osKey}-${archKey}${platform() === 'win32' ? '.exe' : ''}`;
}

const binaryName = getBinaryName();

if (!binaryName) {
  console.error(`Error: Unsupported platform: ${platform()}-${arch()}`);
  process.exit(1);
}

const binaryPath = join(__dirname, binaryName);

if (!existsSync(binaryPath)) {
  console.error(`Error: No binary found for ${platform()}-${arch()}`);
  console.error(`Expected: ${binaryPath}`);
  console.error('');
  console.error('Run "npm run build:native" to build the local native binary,');
  console.error('or reinstall the package to trigger the GitHub release download.');
  process.exit(1);
}

if (platform() !== 'win32') {
  try {
    accessSync(binaryPath, constants.X_OK);
  } catch {
    try {
      chmodSync(binaryPath, 0o755);
    } catch (error) {
      console.error(`Error: Cannot make binary executable: ${error.message}`);
      process.exit(1);
    }
  }
}

const child = spawn(binaryPath, process.argv.slice(2), {
  stdio: 'inherit',
  windowsHide: false,
});

child.on('error', (error) => {
  console.error(`Error executing binary: ${error.message}`);
  process.exit(1);
});

child.on('close', (code) => {
  process.exit(code ?? 0);
});
