#!/bin/bash

set -e

echo "🧪 最终测试脚本..."

CURRENT_DIR=$(pwd)
TEST_PROJECT="/Users/travzhang/test-go"

# 确保动态库存在
echo "🏗️  确保动态库存在..."
cargo build --release

# 检查lib符号链接
if [ ! -L "lib" ]; then
    echo "🔗 创建lib符号链接..."
    ln -sf target/release lib
fi

echo "✅ 动态库准备完成"

# 进入测试项目
if [ ! -d "$TEST_PROJECT" ]; then
    echo "📁 创建测试项目目录..."
    mkdir -p "$TEST_PROJECT"
fi

cd "$TEST_PROJECT"

echo "📝 创建测试项目文件..."

# 创建go.mod
cat > go.mod << EOF
module test-go

go 1.22

require github.com/canyon-project/rust-istanbul-sourcemap v0.1.1

replace github.com/canyon-project/rust-istanbul-sourcemap => $CURRENT_DIR
EOF

# 创建main.go
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

echo "🚀 运行测试..."
go run main.go

echo ""
echo "🎉 测试完成！"