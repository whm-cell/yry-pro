#!/bin/bash

# 检查是否提供版本号参数
if [ -z "$1" ]; then
  echo "错误: 请提供版本号"
  echo "用法: ./scripts/release.sh <版本号>"
  echo "示例: ./scripts/release.sh 0.1.6"
  exit 1
fi

VERSION=$1
BRANCH=$(git rev-parse --abbrev-ref HEAD)

# 确认版本发布
echo "准备发布版本 v${VERSION}，当前分支为 ${BRANCH}"
read -p "请确认是否继续 (y/n): " CONFIRM
if [ "$CONFIRM" != "y" ]; then
  echo "已取消发布"
  exit 0
fi

# 同步版本号
echo "正在同步版本号..."
pnpm run sync-version ${VERSION}

# 提交版本更改
echo "正在提交版本更改..."
git add package.json src-tauri/Cargo.toml
git commit -m "chore: 更新版本号至 ${VERSION}"

# 推送更改
echo "正在推送版本更改到远程仓库..."
git push origin ${BRANCH}

# 创建并推送标签
echo "正在创建标签 v${VERSION}..."
git tag v${VERSION}
git push origin v${VERSION}

echo "✅ 版本 v${VERSION} 发布流程已完成"
echo "请前往 GitHub 仓库页面检查 Actions 是否已自动触发构建流程" 