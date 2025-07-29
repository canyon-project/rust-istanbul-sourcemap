# 安装指南

## 🚀 快速安装

### 步骤1: 安装Go模块

```bash
go get github.com/canyon-project/rust-istanbul-sourcemap
```

### 步骤2: 下载原生库

在你的项目目录中运行：

```bash
go run github.com/canyon-project/rust-istanbul-sourcemap/install.go
```

或者使用下载脚本：

```bash
go run github.com/canyon-project/rust-istanbul-sourcemap/download_lib.go
```

### 步骤3: 使用

```go
package main

import (
    "fmt"
    "log"
    
    istanbul "github.com/canyon-project/rust-istanbul-sourcemap"
)

func main() {
    ism := istanbul.New()
    fmt.Printf("Version: %s\n", ism.GetVersion())
    
    // 你的覆盖率数据
    result, err := ism.TransformCoverage(coverageData)
    if err != nil {
        log.Fatal(err)
    }
    
    fmt.Println(result)
}
```

## 🔧 详细说明

### 为什么需要两步安装？

这个库使用CGO调用Rust编写的原生库，需要：

1. **Go代码**：通过`go get`获取
2. **原生库**：平台特定的动态库文件（.so/.dylib/.dll）

### 支持的平台

- **Linux**: x86_64, ARM64
- **macOS**: x86_64 (Intel), ARM64 (Apple Silicon)
- **Windows**: x86_64

### 库文件位置

安装脚本会将库文件下载到：
- `lib/libistanbul_sourcemap.so` (Linux/macOS)
- `lib/istanbul_sourcemap.dll` (Windows)

## 🛠️ 故障排除

### 1. CGO编译错误

确保安装了C编译器：

**macOS:**
```bash
xcode-select --install
```

**Linux:**
```bash
sudo apt-get install build-essential
```

**Windows:**
安装MinGW或Visual Studio

### 2. 库下载失败

如果自动下载失败，可以手动下载：

1. 访问 [GitHub Releases](https://github.com/canyon-project/rust-istanbul-sourcemap/releases)
2. 下载对应平台的库文件
3. 放到项目的`lib/`目录中
4. 重命名为标准名称：
   - Linux/macOS: `libistanbul_sourcemap.so`
   - Windows: `istanbul_sourcemap.dll`

### 3. 权限问题

在Unix系统上，确保库文件有执行权限：

```bash
chmod +x lib/libistanbul_sourcemap.so
```

### 4. 环境变量

如果仍有链接问题，设置环境变量：

```bash
# Linux
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:./lib

# macOS
export DYLD_LIBRARY_PATH=$DYLD_LIBRARY_PATH:./lib

# Windows
set PATH=%PATH%;.\lib
```

## 📋 完整示例

### 项目结构

```
your-project/
├── go.mod
├── main.go
└── lib/
    └── libistanbul_sourcemap.so  # 安装脚本下载的库文件
```

### go.mod

```go
module your-project

go 1.22

require github.com/canyon-project/rust-istanbul-sourcemap v0.1.5
```

### main.go

```go
package main

import (
	"fmt"
	"log"

	istanbul "github.com/canyon-project/rust-istanbul-sourcemap"
)

func main() {
	// 创建实例
	ism := istanbul.New()
	
	// 获取版本信息
	fmt.Printf("Library version: %s\n", ism.GetVersion())
	fmt.Printf("Platform: %s\n", ism.GetPlatform())

	// Istanbul覆盖率数据
	coverageData := `{
		"dist/app.js": {
			"path": "dist/app.js",
			"statementMap": {
				"0": {"start": {"line": 1, "column": 0}, "end": {"line": 1, "column": 25}}
			},
			"fnMap": {},
			"branchMap": {},
			"s": {"0": 1},
			"f": {},
			"b": {},
			"inputSourceMap": {
				"version": 3,
				"sources": ["src/app.ts"],
				"names": [],
				"mappings": "AAAA,SAASA"
			}
		}
	}`

	// 转换覆盖率数据
	result, err := ism.TransformCoverage(coverageData)
	if err != nil {
		log.Fatalf("Error: %v", err)
	}

	fmt.Printf("Transformed coverage:\n%s\n", result)
}
```

### 运行

```bash
# 安装依赖
go mod tidy

# 下载原生库
go run github.com/canyon-project/rust-istanbul-sourcemap/install.go

# 运行程序
go run main.go
```

## 🎯 最佳实践

1. **在CI/CD中**：
   ```bash
   # 在构建脚本中添加
   go run github.com/canyon-project/rust-istanbul-sourcemap/install.go
   ```

2. **Docker中**：
   ```dockerfile
   RUN go run github.com/canyon-project/rust-istanbul-sourcemap/install.go
   ```

3. **Makefile中**：
   ```makefile
   install-deps:
   	go mod tidy
   	go run github.com/canyon-project/rust-istanbul-sourcemap/install.go
   ```

这样用户就有了清晰的安装步骤，避免了CGO编译时找不到库文件的问题！