# 使用指南

## 🚀 快速开始

### 安装

```bash
go get github.com/canyon-project/rust-istanbul-sourcemap
```

### 基本使用

```go
package main

import (
    "fmt"
    "log"
    
    istanbul "github.com/canyon-project/rust-istanbul-sourcemap"
)

func main() {
    // 创建实例（会自动下载动态库）
    ism := istanbul.New()
    
    // 获取版本信息
    fmt.Printf("Version: %s\n", ism.GetVersion())
    
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
        log.Fatal(err)
    }
    
    fmt.Printf("Result: %s\n", result)
}
```

## 📦 自动库管理

库会在首次使用时自动下载：

1. **自动检测**：检查是否存在动态库
2. **自动下载**：如果不存在，从GitHub Releases下载
3. **平台适配**：自动选择适合的平台版本

支持的平台：
- Linux (x86_64, ARM64)
- macOS (x86_64, ARM64)
- Windows (x86_64)

## 🔧 手动管理

如果自动下载失败，可以手动下载：

```bash
# 方法1: 运行下载脚本
go run github.com/canyon-project/rust-istanbul-sourcemap/download_lib.go

# 方法2: 手动下载
# 从 https://github.com/canyon-project/rust-istanbul-sourcemap/releases
# 下载对应平台的动态库到 lib/ 目录
```

## 🛠️ 故障排除

### CGO编译错误

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

### 动态库加载失败

1. **检查库文件**：
   ```bash
   ls lib/
   ```

2. **设置环境变量**：
   ```bash
   # Linux/macOS
   export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:./lib
   export DYLD_LIBRARY_PATH=$DYLD_LIBRARY_PATH:./lib
   
   # Windows
   set PATH=%PATH%;.\lib
   ```

3. **手动下载**：
   ```bash
   go run github.com/canyon-project/rust-istanbul-sourcemap/download_lib.go
   ```

### 网络问题

如果无法从GitHub下载，可以：

1. **使用代理**：
   ```bash
   export HTTPS_PROXY=your-proxy
   go run download_lib.go
   ```

2. **手动下载**：
   从GitHub Releases页面手动下载并放到`lib/`目录

## 📋 API参考

### IstanbulSourceMap

#### New() *IstanbulSourceMap
创建新实例，会自动确保动态库可用。

#### TransformCoverage(input string) (string, error)
转换Istanbul覆盖率数据。

**参数:**
- `input`: JSON格式的Istanbul覆盖率数据

**返回:**
- `string`: 转换后的覆盖率数据
- `error`: 错误信息

#### GetVersion() string
获取库版本号。

#### GetPlatform() string
获取当前平台信息。

### 包级别函数

#### TransformCoverage(input string) (string, error)
便捷函数，等同于 `New().TransformCoverage(input)`

#### GetVersion() string
便捷函数，获取库版本。

## 🎯 最佳实践

1. **错误处理**：
   ```go
   result, err := ism.TransformCoverage(data)
   if err != nil {
       log.Printf("Transform failed: %v", err)
       return
   }
   ```

2. **重用实例**：
   ```go
   ism := istanbul.New() // 创建一次
   // 多次使用
   result1, _ := ism.TransformCoverage(data1)
   result2, _ := ism.TransformCoverage(data2)
   ```

3. **并发安全**：
   ```go
   // 可以在多个goroutine中安全使用
   var ism = istanbul.New()
   
   go func() {
       result, _ := ism.TransformCoverage(data1)
   }()
   
   go func() {
       result, _ := ism.TransformCoverage(data2)
   }()
   ```

## 🔄 更新

更新到最新版本：

```bash
go get -u github.com/canyon-project/rust-istanbul-sourcemap
```

库会自动检测版本并下载对应的动态库。