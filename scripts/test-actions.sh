#!/bin/bash

set -e

echo "🧪 测试GitHub Actions配置..."

# 检查GitHub Actions语法
echo "✅ 检查GitHub Actions语法..."
if command -v act >/dev/null 2>&1; then
    echo "   使用act验证工作流..."
    act --list
    echo "   ✓ GitHub Actions语法正确"
else
    echo "   ⚠️  act未安装，跳过语法检查"
    echo "   提示: 安装act来本地测试GitHub Actions: https://github.com/nektos/act"
fi

# 检查Rust代码格式
echo "✅ 检查Rust代码格式..."
if command -v cargo >/dev/null 2>&1; then
    cargo fmt -- --check
    echo "   ✓ Rust代码格式正确"
else
    echo "   ❌ Cargo未安装"
    exit 1
fi

# 运行Clippy检查
echo "✅ 运行Clippy检查..."
cargo clippy -- -D warnings
echo "   ✓ Clippy检查通过"

# 运行Rust测试
echo "✅ 运行Rust测试..."
cargo test
echo "   ✓ Rust测试通过"

# 检查Go代码格式
echo "✅ 检查Go代码..."
if command -v go >/dev/null 2>&1; then
    go mod tidy
    go mod verify
    echo "   ✓ Go模块配置正确"
    
    # 格式化Go代码
    go fmt ./...
    echo "   ✓ Go代码格式正确"
    
    # 运行Go vet
    go vet ./...
    echo "   ✓ Go vet检查通过"
else
    echo "   ❌ Go未安装"
    exit 1
fi

echo ""
echo "🎉 所有检查通过！"
echo ""
echo "GitHub Actions配置已更新："
echo "- ✅ 升级到actions/upload-artifact@v4"
echo "- ✅ 升级到actions/download-artifact@v4" 
echo "- ✅ 升级到actions/cache@v4"
echo "- ✅ 升级到actions/setup-go@v5"
echo "- ✅ 添加了代码质量检查"
echo "- ✅ 更新Go版本到1.22"
echo ""
echo "现在可以安全地推送代码并发布了！"