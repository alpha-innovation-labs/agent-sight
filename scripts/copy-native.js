#!/usr/bin/env node

import { execSync } from 'child_process';
import { chmodSync, copyFileSync, existsSync, mkdirSync } from 'fs';
import { arch, platform } from 'os';
import { join } from 'path';

const CLI_NAME = 'agent-sight';
const projectRoot = process.cwd();
const binDir = join(projectRoot, 'bin');
const ext = platform() === 'win32' ? '.exe' : '';

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
      throw new Error(`Unsupported platform: ${platform()}`);
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
      throw new Error(`Unsupported architecture: ${arch()}`);
  }

  return `${CLI_NAME}-${osKey}-${archKey}${ext}`;
}

const sourcePath = join(projectRoot, 'cli', 'target', 'release', `${CLI_NAME}${ext}`);
if (!existsSync(sourcePath)) {
  throw new Error(`Native build output not found: ${sourcePath}`);
}

mkdirSync(binDir, { recursive: true });

const destinationPath = join(binDir, getBinaryName());
copyFileSync(sourcePath, destinationPath);

if (platform() !== 'win32') {
  chmodSync(destinationPath, 0o755);
}

console.log(`Copied ${sourcePath} -> ${destinationPath}`);
