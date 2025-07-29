#!/bin/bash

set -e

echo "🧪 测试go get安装流程..."

# 创建临时目录
TEMP_DIR=$(mktemp -d)
echo "📁 临时测试目录: $TEMP_DIR"

cd "$TEMP_DIR"

# 初始化Go模块
echo "📦 初始化Go模块..."
go mod init test-go-get

# 添加依赖
echo "⬇️  添加依赖..."
go get github.com/canyon-project/rust-istanbul-sourcemap@latest

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
	fmt.Println("🧪 测试go get安装的模块...")

	// 创建实例（应该自动下载库）
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
	
	fmt.Println("🎉 go get测试成功!")
}
EOF

echo "🚀 运行测试..."
go run main.go

# 检查lib目录
echo "📚 检查下载的库文件..."
if [ -d "lib" ]; then
    echo "✅ lib目录存在"
    ls -la lib/
else
    echo "❌ lib目录不存在"
fi

# 清理
echo "🧹 清理临时文件..."
cd /
rm -rf "$TEMP_DIR"

echo ""
echo "✅ go get测试完成!"