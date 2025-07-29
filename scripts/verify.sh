#!/bin/bash

set -e

echo "🔍 验证项目配置..."

# 检查Go模块配置
echo "✅ 检查go.mod..."
if grep -q "github.com/canyon-project/rust-istanbul-sourcemap" go.mod; then
    echo "   ✓ go.mod 配置正确"
else
    echo "   ❌ go.mod 配置错误"
    exit 1
fi

# 检查GitHub Actions配置
echo "✅ 检查GitHub Actions..."
if [ -f ".github/workflows/build.yml" ]; then
    echo "   ✓ GitHub Actions 配置存在"
else
    echo "   ❌ GitHub Actions 配置缺失"
    exit 1
fi

# 检查CLI工具
echo "✅ 检查CLI工具..."
if [ -f "cmd/cli/main.go" ]; then
    echo "   ✓ CLI工具存在"
else
    echo "   ❌ CLI工具缺失"
    exit 1
fi

# 检查FFI接口
echo "✅ 检查FFI接口..."
if [ -f "src/ffi.rs" ]; then
    echo "   ✓ FFI接口存在"
else
    echo "   ❌ FFI接口缺失"
    exit 1
fi

# 检查Cargo.toml配置
echo "✅ 检查Cargo.toml..."
if grep -q 'crate-type = \["cdylib"\]' Cargo.toml; then
    echo "   ✓ Cargo.toml 配置正确"
else
    echo "   ❌ Cargo.toml 配置错误"
    exit 1
fi

# 检查包管理器配置
echo "✅ 检查包管理器配置..."
if [ -f "pkg/homebrew/istanbul-sourcemap-go.rb" ]; then
    echo "   ✓ Homebrew 配置存在"
else
    echo "   ❌ Homebrew 配置缺失"
fi

if [ -f "pkg/arch/PKGBUILD" ]; then
    echo "   ✓ Arch Linux 配置存在"
else
    echo "   ❌ Arch Linux 配置缺失"
fi

echo ""
echo "🎉 所有配置验证完成！"
echo ""
echo "下一步："
echo "1. 提交所有更改到Git"
echo "2. 运行 ./scripts/publish.sh v0.1.0 发布第一个版本"
echo "3. 用户就可以通过 go get github.com/canyon-project/rust-istanbul-sourcemap 安装了"