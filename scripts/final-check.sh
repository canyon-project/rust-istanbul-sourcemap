#!/bin/bash

set -e

echo "🎯 最终发布前检查..."

# 清理构建
echo "🧹 清理构建..."
cargo clean

# 完整构建测试
echo "🔨 完整构建测试..."
cargo build --release

# 运行所有测试
echo "🧪 运行所有测试..."
cargo test --release

# 检查动态库构建
echo "📚 检查动态库构建..."
if [ -f "target/release/libistanbul_sourcemap.so" ] || [ -f "target/release/libistanbul_sourcemap.dylib" ] || [ -f "target/release/istanbul_sourcemap.dll" ]; then
    echo "   ✓ 动态库构建成功"
else
    echo "   ❌ 动态库构建失败"
    exit 1
fi

# 检查二进制文件
echo "🔧 检查二进制文件..."
if [ -f "target/release/istanbul-sourcemap" ] || [ -f "target/release/istanbul-sourcemap.exe" ]; then
    echo "   ✓ CLI工具构建成功"
else
    echo "   ❌ CLI工具构建失败"
    exit 1
fi

# 运行示例
echo "🚀 运行示例..."
cargo run --bin example --release

# 检查Go模块
echo "🐹 检查Go模块..."
go mod tidy
go mod verify

echo ""
echo "🎉 所有检查通过！项目已准备好发布！"
echo ""
echo "发布步骤："
echo "1. git add ."
echo "2. git commit -m 'feat: 升级GitHub Actions并修复所有警告'"
echo "3. git push origin main"
echo "4. ./scripts/publish.sh v0.1.0"
echo ""
echo "用户安装方式："
echo "go get github.com/canyon-project/rust-istanbul-sourcemap"