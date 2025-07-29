#!/bin/bash

set -e

echo "🔧 修复动态库链接问题..."

# 检查参数
if [ -z "$1" ]; then
    echo "使用方法: $0 <你的测试项目路径>"
    echo "示例: $0 /Users/travzhang/Desktop/test-go"
    exit 1
fi

TEST_PROJECT="$1"
CURRENT_DIR=$(pwd)

if [ ! -d "$TEST_PROJECT" ]; then
    echo "❌ 测试项目目录不存在: $TEST_PROJECT"
    exit 1
fi

echo "📁 测试项目: $TEST_PROJECT"
echo "📁 源项目: $CURRENT_DIR"

# 构建动态库
echo "🏗️  构建Rust动态库..."
cd "$CURRENT_DIR"
cargo build --release

# 检查动态库是否存在
if [[ "$OSTYPE" == "darwin"* ]]; then
    LIB_FILE="target/release/libistanbul_sourcemap.dylib"
    if [ ! -f "$LIB_FILE" ]; then
        echo "❌ 动态库不存在: $LIB_FILE"
        exit 1
    fi
    echo "✅ 找到动态库: $LIB_FILE"
else
    echo "❌ 当前脚本主要支持macOS，请根据你的系统调整"
    exit 1
fi

# 进入测试项目
cd "$TEST_PROJECT"

# 创建lib目录
mkdir -p lib

# 复制动态库
echo "📚 复制动态库到测试项目..."
cp "$CURRENT_DIR/$LIB_FILE" lib/libistanbul_sourcemap.dylib
cp "$CURRENT_DIR/$LIB_FILE" lib/libistanbul_sourcemap.so

echo "✅ 动态库复制完成"

# 检查Go模块缓存中的库
GO_MOD_CACHE=$(go env GOMODCACHE)
MODULE_PATH="$GO_MOD_CACHE/github.com/canyon-project/rust-istanbul-sourcemap@v0.1.1"

if [ -d "$MODULE_PATH" ]; then
    echo "📦 找到Go模块缓存: $MODULE_PATH"
    
    # 创建lib目录并复制库
    mkdir -p "$MODULE_PATH/lib"
    cp "$CURRENT_DIR/$LIB_FILE" "$MODULE_PATH/lib/libistanbul_sourcemap.dylib"
    cp "$CURRENT_DIR/$LIB_FILE" "$MODULE_PATH/lib/libistanbul_sourcemap.so"
    
    echo "✅ 动态库已复制到Go模块缓存"
else
    echo "⚠️  Go模块缓存目录不存在，将使用本地方法"
fi

# 修改go.mod使用本地路径（更可靠的方法）
echo "🔧 修改go.mod使用本地路径..."

cat > go.mod << EOF
module test-go

go 1.22

require github.com/canyon-project/rust-istanbul-sourcemap v0.1.1

replace github.com/canyon-project/rust-istanbul-sourcemap => $CURRENT_DIR
EOF

echo "✅ go.mod已更新为使用本地路径"

# 运行go mod tidy
echo "🔄 运行go mod tidy..."
go mod tidy

# 设置环境变量
echo "🌍 设置环境变量..."
export CGO_LDFLAGS="-L./lib -L$CURRENT_DIR/target/release"
export DYLD_LIBRARY_PATH="./lib:$CURRENT_DIR/target/release:$DYLD_LIBRARY_PATH"
export LD_LIBRARY_PATH="./lib:$CURRENT_DIR/target/release:$LD_LIBRARY_PATH"

echo "✅ 环境变量已设置"

echo ""
echo "🎉 修复完成！"
echo ""
echo "现在尝试运行："
echo "cd $TEST_PROJECT"
echo "export DYLD_LIBRARY_PATH=\"./lib:$CURRENT_DIR/target/release:\$DYLD_LIBRARY_PATH\""
echo "go run main.go"
echo ""
echo "如果仍有问题，尝试："
echo "CGO_LDFLAGS=\"-L./lib -L$CURRENT_DIR/target/release\" go run main.go"