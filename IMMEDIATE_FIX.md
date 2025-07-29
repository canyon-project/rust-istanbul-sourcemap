# 立即修复动态库问题

## 🚨 问题分析
错误显示Go找到了模块，但链接器找不到 `libistanbul_sourcemap` 动态库。

## ⚡ 立即解决方案

### 步骤1: 构建动态库
在 `rust-istanbul-sourcemap` 目录中运行：
```bash
cargo build --release
```

### 步骤2: 修复你的测试项目
在你的 `test-go` 目录中：

1. **创建lib目录并复制动态库**：
```bash
mkdir -p lib
cp /Users/travzhang/Desktop/rust-istanbul-sourcemap/target/release/libistanbul_sourcemap.dylib lib/
ln -sf libistanbul_sourcemap.dylib lib/libistanbul_sourcemap.so
```

2. **修改go.mod使用本地路径**：
```go
module test-go

go 1.22

require github.com/canyon-project/rust-istanbul-sourcemap v0.1.1

replace github.com/canyon-project/rust-istanbul-sourcemap => /Users/travzhang/Desktop/rust-istanbul-sourcemap
```

3. **修正main.go**（你的代码有个小错误）：
```go
package main

import (
	"fmt"
	"log"

	istanbul "github.com/canyon-project/rust-istanbul-sourcemap"
)

func main() {
	fmt.Println("🧪 测试Istanbul Sourcemap Go绑定...")

	// 注意：使用string，不是[]byte
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

	ism := istanbul.New()
	fmt.Printf("📚 Library version: %s\n", ism.GetVersion())
	fmt.Printf("🖥️  Platform: %s\n", ism.GetPlatform())

	// 注意：TransformCoverage接受string，不是[]byte
	result, err := ism.TransformCoverage(coverageData)
	if err != nil {
		log.Fatalf("❌ Error: %v", err)
	}

	fmt.Printf("✅ Result:\n%s\n", result)
}
```

4. **运行go mod tidy**：
```bash
go mod tidy
```

5. **设置环境变量并运行**：
```bash
DYLD_LIBRARY_PATH=./lib go run main.go
```

## 🔧 一键修复脚本

或者运行我准备的修复脚本：
```bash
cd /Users/travzhang/Desktop/rust-istanbul-sourcemap
./scripts/quick-library-fix.sh
```

## 🎯 关键点

1. **动态库位置**：必须在 `lib/` 目录中
2. **go.mod配置**：使用本地路径替换
3. **环境变量**：`DYLD_LIBRARY_PATH=./lib`
4. **数据类型**：`TransformCoverage` 接受 `string`，不是 `[]byte`

## 🚀 验证成功

如果一切正常，你应该看到类似输出：
```
🧪 测试Istanbul Sourcemap Go绑定...
📚 Library version: 0.1.0
🖥️  Platform: darwin/arm64
✅ Result:
{
  "src/app.ts": {
    ...
  }
}
```