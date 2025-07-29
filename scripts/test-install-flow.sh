#!/bin/bash

set -e

echo "🧪 测试新的安装流程..."

# 创建临时目录
TEMP_DIR=$(mktemp -d)
echo "📁 临时测试目录: $TEMP_DIR"

cd "$TEMP_DIR"

# 初始化Go模块
echo "📦 步骤1: 初始化Go模块..."
go mod init test-install-flow

# 添加依赖
echo "⬇️  步骤2: 添加依赖..."
go get github.com/canyon-project/rust-istanbul-sourcemap@latest

# 运行安装脚本
echo "🔧 步骤3: 运行安装脚本..."
go run github.com/canyon-project/rust-istanbul-sourcemap/install.go

# 检查lib目录
echo "📚 步骤4: 检查库文件..."
if [ -d "lib" ]; then
    echo "✅ lib目录存在"
    ls -la lib/
else
    echo "❌ lib目录不存在"
    exit 1
fi

# 创建测试文件
echo "📝 步骤5: 创建测试文件..."
cat > main.go << 'EOF'
package main

import (
	"fmt"
	"log"

	istanbul "github.com/canyon-project/rust-istanbul-sourcemap"
)

func main() {
	fmt.Println("🧪 测试新的安装流程...")

	// 创建实例
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
	
	fmt.Println("🎉 新安装流程测试成功!")
}
EOF

# 运行测试
echo "🚀 步骤6: 运行测试..."
go run main.go

# 清理
echo "🧹 清理临时文件..."
cd /
rm -rf "$TEMP_DIR"

echo ""
echo "✅ 新安装流程测试完成!"
echo ""
echo "用户现在需要两步："
echo "1. go get github.com/canyon-project/rust-istanbul-sourcemap"
echo "2. go run github.com/canyon-project/rust-istanbul-sourcemap/install.go"