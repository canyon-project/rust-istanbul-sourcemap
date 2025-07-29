#!/bin/bash

set -e

echo "🔧 设置GitHub仓库..."

# 检查是否已经有远程仓库
if git remote get-url origin >/dev/null 2>&1; then
    echo "✅ 远程仓库已配置"
    git remote -v
else
    echo "❌ 未配置远程仓库"
    echo "请先在GitHub上创建仓库 canyon-project/rust-istanbul-sourcemap"
    echo "然后运行："
    echo "git remote add origin https://github.com/canyon-project/rust-istanbul-sourcemap.git"
    exit 1
fi

# 检查当前分支
CURRENT_BRANCH=$(git branch --show-current)
echo "当前分支: $CURRENT_BRANCH"

# 检查是否有未提交的更改
if [ -n "$(git status --porcelain)" ]; then
    echo "📝 发现未提交的更改，正在提交..."
    git add .
    git commit -m "feat: 完整的Go语言绑定和多平台发布支持

- 添加Go语言FFI绑定
- 支持多平台动态库构建
- 完整的GitHub Actions CI/CD
- 升级所有Actions到最新版本
- 添加CLI工具和示例
- 完善的文档和测试"
else
    echo "✅ 工作目录干净"
fi

# 推送到远程仓库
echo "🚀 推送到远程仓库..."
git push -u origin $CURRENT_BRANCH

echo ""
echo "🎉 仓库设置完成！"
echo ""
echo "现在用户可以安装Go模块："
echo "go get github.com/canyon-project/rust-istanbul-sourcemap"
echo ""
echo "如果要发布版本，运行："
echo "./scripts/publish.sh v0.1.0"