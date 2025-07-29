#!/bin/bash

set -e

echo "🔧 修复CGO路径配置..."

CURRENT_DIR=$(pwd)
TEST_PROJECT="/Users/travzhang/test-go"

# 确保我们在正确的目录
if [ ! -f "Cargo.toml" ]; then
    echo "❌ 请在rust-istanbul-sourcemap项目根目录运行此脚本"
    exit 1
fi

echo "📁 当前目录: $CURRENT_DIR"

# 构建动态库
echo "🏗️  构建Rust动态库..."
cargo build --release

# 检查动态库是否存在
LIB_FILE="target/release/libistanbul_sourcemap.dylib"
if [ ! -f "$LIB_FILE" ]; then
    echo "❌ 动态库构建失败: $LIB_FILE"
    exit 1
fi

echo "✅ 动态库构建成功: $LIB_FILE"

# 创建lib目录的符号链接（为了兼容性）
echo "🔗 创建lib目录符号链接..."
rm -rf lib
ln -sf target/release lib

echo "✅ 符号链接创建完成: lib -> target/release"

# 如果测试项目存在，也修复它
if [ -d "$TEST_PROJECT" ]; then
    echo "🔧 修复测试项目..."
    cd "$TEST_PROJECT"
    
    # 创建本地lib目录
    mkdir -p lib
    cp "$CURRENT_DIR/$LIB_FILE" lib/
    
    # 确保go.mod使用本地路径
    cat > go.mod << EOF
module test-go

go 1.22

require github.com/canyon-project/rust-istanbul-sourcemap v0.1.1

replace github.com/canyon-project/rust-istanbul-sourcemap => $CURRENT_DIR
EOF
    
    # 创建正确的main.go
    cat > main.go << 'EOF'
package main

import (
	"fmt"
	"log"

	istanbul "github.com/canyon-project/rust-istanbul-sourcemap"
)

func main() {
	fmt.Println("🧪 测试Istanbul Sourcemap Go绑定...")

	// 正确的Istanbul覆盖率数据格式 (string, 不是 []byte)
	coverageData := `{
		"dist/app.js": {
			"path": "dist/app.js",
			"statementMap": {
				"0": {"start": {"line": 1, "column": 0}, "end": {"line": 1, "column": 25}}
			},
			"fnMap": {
				"0": {
					"name": "myFunction",
					"decl": {"start": {"line": 1, "column": 9}, "end": {"line": 1, "column": 19}},
					"loc": {"start": {"line": 1, "column": 0}, "end": {"line": 3, "column": 1}}
				}
			},
			"branchMap": {},
			"s": {"0": 1},
			"f": {"0": 1},
			"b": {},
			"inputSourceMap": {
				"version": 3,
				"sources": ["src/app.ts"],
				"names": ["myFunction"],
				"mappings": "AAAA,SAASA",
				"file": "app.js"
			}
		}
	}`

	// 初始化转换器
	ism := istanbul.New()
	
	// 获取版本信息
	fmt.Printf("📚 Library version: %s\n", ism.GetVersion())
	fmt.Printf("🖥️  Platform: %s\n", ism.GetPlatform())

	// 转换覆盖率数据
	result, err := ism.TransformCoverage(coverageData)
	if err != nil {
		log.Fatalf("❌ Error transforming coverage: %v", err)
	}

	fmt.Printf("✅ Transformed coverage data:\n%s\n", result)
}
EOF
    
    # 运行go mod tidy
    echo "🔄 运行go mod tidy..."
    go mod tidy
    
    echo "✅ 测试项目修复完成"
    
    cd "$CURRENT_DIR"
fi

echo ""
echo "🎉 CGO路径修复完成！"
echo ""
echo "现在可以测试："
echo "cd $TEST_PROJECT"
echo "go run main.go"
echo ""
echo "如果仍有问题，尝试设置环境变量："
echo "DYLD_LIBRARY_PATH=./lib:$CURRENT_DIR/target/release go run main.go"