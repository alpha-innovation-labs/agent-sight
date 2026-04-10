#!/usr/bin/env node

import { readFileSync, writeFileSync } from 'fs';
import { join } from 'path';

const projectRoot = process.cwd();
const packageJsonPath = join(projectRoot, 'package.json');
const cargoTomlPath = join(projectRoot, 'cli', 'Cargo.toml');
const cargoLockPath = join(projectRoot, 'cli', 'Cargo.lock');

const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf8'));
const version = packageJson.version;

const cargoToml = readFileSync(cargoTomlPath, 'utf8');
const nextCargoToml = cargoToml.replace(/version = "[^"]+"/, `version = "${version}"`);

if (nextCargoToml !== cargoToml) {
  writeFileSync(cargoTomlPath, nextCargoToml);
}

try {
  const cargoLock = readFileSync(cargoLockPath, 'utf8');
  const nextCargoLock = cargoLock.replace(
    /name = "promsight_rs"\nversion = "[^"]+"/,
    `name = "promsight_rs"\nversion = "${version}"`,
  );

  if (nextCargoLock !== cargoLock) {
    writeFileSync(cargoLockPath, nextCargoLock);
  }
} catch {
  // Cargo.lock may not exist yet on fresh checkouts.
}

console.log(`Synced Rust crate version to ${version}`);
