#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// 从命令行参数或环境变量获取版本号
const newVersion = process.argv[2] || process.env.APP_VERSION;

if (!newVersion) {
  console.error('请提供版本号作为参数或设置 APP_VERSION 环境变量');
  process.exit(1);
}

// 验证版本号格式
if (!/^\d+\.\d+\.\d+(-\w+(\.\d+)?)?$/.test(newVersion)) {
  console.error('版本号格式无效，应为 x.y.z 或 x.y.z-alpha.n');
  process.exit(1);
}

const rootDir = path.resolve(__dirname, '..');

// 更新 package.json 中的版本
const packageJsonPath = path.join(rootDir, 'package.json');
const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
const oldPackageVersion = packageJson.version;
packageJson.version = newVersion;
fs.writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2) + '\n');

// 更新 Cargo.toml 中的版本
const cargoTomlPath = path.join(rootDir, 'src-tauri', 'Cargo.toml');
const cargoToml = fs.readFileSync(cargoTomlPath, 'utf8');
const updatedCargoToml = cargoToml.replace(
  /^version = "(.*?)"/m,
  `version = "${newVersion}"`
);
fs.writeFileSync(cargoTomlPath, updatedCargoToml);

// 更新 tauri.conf.json 中的版本
const tauriConfPath = path.join(rootDir, 'src-tauri', 'tauri.conf.json');
const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, 'utf8'));
tauriConf.version = newVersion;
fs.writeFileSync(tauriConfPath, JSON.stringify(tauriConf, null, 2) + '\n');

console.log(`版本已从 ${oldPackageVersion} 更新到 ${newVersion}`);
console.log('- package.json 已更新');
console.log('- src-tauri/Cargo.toml 已更新');
console.log('- src-tauri/tauri.conf.json 已更新'); 