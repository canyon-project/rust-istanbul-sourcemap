#!/bin/bash

set -e

echo "🔍 验证Go模块安装..."

# 创建临时测试目录
TEMP_DIR=$(mktemp -d)
echo "📁 临时测试目录: $TEMP_DIR"

cd "$TEMP_DIR"

# 初始化Go模块
echo "📦 初始化Go模块..."
go mod init test-installation

# 添加依赖
echo "⬇️  添加依赖..."
go get github.com/canyon-project/rust-istanbul-sourcemap

# 创建测试文件
echo "📝 创建测试文件..."
cat > main.go << 'EOF'
package main

import (
	"fmt"
	"log"

	istanbul "github.com/canyon-project/rust-istanbul-sourcemap"
)

func main() {
	fmt.Println("🧪 验证Istanbul Sourcemap安装...")

	// 创建实例（会自动下载库）
	ism := istanbul.New()

	// 获取版本信息
	fmt.Printf("📚 Library version: %s\n", ism.GetVersion())
	fmt.Printf("🖥️  Platform: %s\n", ism.GetPlatform())

	// 测试数据
	coverageData := `{
		"test.js": {
			"path": "test.js",
			"statementMap": {
				"0": {"start": {"line": 1, "column": 0}, "end": {"line": 1, "column": 10}}
			},
			"fnMap": {},
			"branchMap": {},
			"s": {"0": 1},
			"f": {},
			"b": {}
		}
	}`

	// 转换测试
	result, err := ism.TransformCoverage(coverageData)
	if err != nil {
		log.Fatalf("❌ 转换失败: %v", err)
	}

	fmt.Printf("✅ 转换成功!\n")
	fmt.Printf("📊 结果长度: %d 字符\n", len(result))
	
	fmt.Println("🎉 安装验证成功!")
}
EOF

# 运行测试
echo "🚀 运行测试..."
go run main.go

# 清理
echo "🧹 清理临时文件..."
cd /
rm -rf "$TEMP_DIR"

echo ""
echo "✅ Go模块安装验证完成!"
echo ""
echo "用户现在可以使用:"
echo "go get github.com/canyon-project/rust-istanbul-sourcemap"