#!/bin/bash

set -e

echo "⚡ 快速修复动态库问题..."

# 默认测试项目路径
TEST_PROJECT="/Users/travzhang/Desktop/test-go"
CURRENT_DIR=$(pwd)

if [ ! -d "$TEST_PROJECT" ]; then
    echo "❌ 测试项目目录不存在: $TEST_PROJECT"
    echo "请提供正确的路径作为参数"
    exit 1
fi

echo "🏗️  构建动态库..."
cargo build --release

echo "📁 进入测试项目: $TEST_PROJECT"
cd "$TEST_PROJECT"

# 创建lib目录
mkdir -p lib

# 复制动态库（macOS）
echo "📚 复制动态库..."
cp "$CURRENT_DIR/target/release/libistanbul_sourcemap.dylib" lib/
ln -sf libistanbul_sourcemap.dylib lib/libistanbul_sourcemap.so

# 修改go.mod使用本地路径
echo "🔧 修改go.mod..."
cat > go.mod << EOF
module test-go

go 1.22

require github.com/canyon-project/rust-istanbul-sourcemap v0.1.1

replace github.com/canyon-project/rust-istanbul-sourcemap => $CURRENT_DIR
EOF

# 创建正确的main.go
echo "📝 创建正确的main.go..."
cat > main.go << 'EOF'
package main

import (
	"fmt"
	"log"

	istanbul "github.com/canyon-project/rust-istanbul-sourcemap"
)

func main() {
	fmt.Println("🧪 测试Istanbul Sourcemap Go绑定...")

	// 正确的Istanbul覆盖率数据格式
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

echo ""
echo "🎉 修复完成！"
echo ""
echo "现在运行测试："
echo "cd $TEST_PROJECT"
echo "DYLD_LIBRARY_PATH=./lib go run main.go"