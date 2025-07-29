# Istanbul Sourcemap Go

Go语言绑定，用于调用高性能的Rust实现的Istanbul覆盖率数据source map转换器。

## 安装

### 方法1: 使用go get (推荐)

```bash
go get github.com/canyon-project/rust-istanbul-sourcemap
```

安装后运行安装脚本下载对应平台的动态库：

```bash
# Linux/macOS
./install.sh

# Windows
install.bat
```

### 方法2: 手动安装

1. 克隆仓库：
```bash
git clone https://github.com/canyon-project/rust-istanbul-sourcemap.git
cd rust-istanbul-sourcemap
```

2. 下载动态库：
```bash
go run download_lib.go
```

## 使用方法

### 基本用法

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
    
    // 获取版本
    fmt.Printf("Library version: %s\n", ism.GetVersion())
    
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
    
    fmt.Printf("Transformed coverage:\n%s\n", result)
}
```

### 使用便捷函数

```go
package main

import (
    "fmt"
    "log"
    
    istanbul "github.com/canyon-project/rust-istanbul-sourcemap"
)

func main() {
    // 直接使用包级别函数
    version := istanbul.GetVersion()
    fmt.Printf("Version: %s\n", version)
    
    result, err := istanbul.TransformCoverage(coverageData)
    if err != nil {
        log.Fatal(err)
    }
    
    fmt.Println(result)
}
```

## API 文档

### 类型

#### IstanbulSourceMap

主要的结构体，提供所有转换功能。

```go
type IstanbulSourceMap struct{}
```

### 方法

#### New() *IstanbulSourceMap

创建新的IstanbulSourceMap实例。

```go
ism := istanbul.New()
```

#### (*IstanbulSourceMap) TransformCoverage(input string) (string, error)

转换Istanbul覆盖率数据，应用source map映射。

**参数:**
- `input`: JSON格式的Istanbul覆盖率数据

**返回:**
- `string`: 转换后的覆盖率数据（JSON格式）
- `error`: 错误信息（如果有）

#### (*IstanbulSourceMap) GetVersion() string

获取底层Rust库的版本号。

#### (*IstanbulSourceMap) GetPlatform() string

获取当前平台信息（OS/架构）。

### 包级别函数

#### TransformCoverage(input string) (string, error)

便捷函数，等同于创建实例后调用TransformCoverage方法。

#### GetVersion() string

便捷函数，获取库版本。

## 支持的平台

- **Linux**: x86_64, ARM64
- **macOS**: x86_64 (Intel), ARM64 (Apple Silicon)
- **Windows**: x86_64

## 性能特点

- **高性能**: 基于Rust实现，比纯Go实现快数倍
- **内存安全**: Rust的内存安全保证
- **零拷贝**: 最小化数据复制
- **并发安全**: 可以在多个goroutine中安全使用

## 错误处理

所有可能失败的操作都返回error。常见错误：

- 无效的JSON输入
- Source map解析失败
- 内存分配失败
- 动态库加载失败

```go
result, err := ism.TransformCoverage(invalidJSON)
if err != nil {
    // 处理错误
    log.Printf("Transform failed: %v", err)
    return
}
```

## 测试

运行测试：

```bash
go test -v
```

注意：测试需要先下载动态库：

```bash
go run download_lib.go
go test -v
```

## 故障排除

### 动态库加载失败

如果遇到动态库加载问题：

1. 确保已运行安装脚本
2. 检查`lib/`目录是否存在对应的库文件
3. 在Linux上可能需要设置`LD_LIBRARY_PATH`
4. 在macOS上可能需要设置`DYLD_LIBRARY_PATH`

### CGO编译问题

确保系统安装了C编译器：

- **Linux**: `gcc`
- **macOS**: Xcode Command Line Tools
- **Windows**: MinGW或Visual Studio

## 与其他语言的对比

| 特性 | Go + Rust | 纯Go | Node.js |
|------|-----------|------|---------|
| 性能 | 🟢 优秀 | 🟡 良好 | 🟡 良好 |
| 内存使用 | 🟢 低 | 🟡 中等 | 🔴 高 |
| 部署复杂度 | 🟡 中等 | 🟢 简单 | 🟢 简单 |
| 类型安全 | 🟢 强 | 🟢 强 | 🟡 弱 |

## 许可证

MIT License

## 贡献

欢迎提交Issue和Pull Request！

## 更新日志

### v0.1.0
- 初始版本
- 支持基本的source map转换
- 多平台支持
- 完整的测试套件