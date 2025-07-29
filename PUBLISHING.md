# 发布指南

这个项目支持发布到多个包管理平台，让用户可以通过不同方式安装和使用。

## 支持的安装方式

### 1. Go模块 (主要方式)

用户可以直接使用Go包管理器安装：

```bash
go get github.com/canyon-project/rust-istanbul-sourcemap
```

**优点:**
- 最简单的安装方式
- 自动处理依赖
- 支持版本管理
- 与Go生态系统完美集成

### 2. 命令行工具

#### GitHub Releases
- 自动构建多平台二进制文件
- 支持 Linux (x86_64, ARM64)
- 支持 macOS (x86_64, ARM64)  
- 支持 Windows (x86_64)

#### Homebrew (计划中)
```bash
brew install canyon-project/tap/istanbul-sourcemap-go
```

#### Arch Linux AUR (计划中)
```bash
yay -S istanbul-sourcemap-go
```

### 3. 动态库

供其他语言调用的动态库：
- Linux: `.so` 文件
- macOS: `.dylib` 文件
- Windows: `.dll` 文件

## 发布流程

### 自动发布 (推荐)

1. 确保代码已提交到main分支
2. 运行发布脚本：
   ```bash
   ./scripts/publish.sh v0.1.0
   ```
3. GitHub Actions会自动：
   - 构建Rust动态库
   - 构建Go CLI工具
   - 创建GitHub Release
   - 上传所有构建产物

### 手动发布

1. 创建并推送tag：
   ```bash
   git tag -a v0.1.0 -m "Release v0.1.0"
   git push origin v0.1.0
   ```

2. GitHub Actions会自动处理其余步骤

## 用户安装步骤

### Go开发者

```bash
# 安装Go包
go get github.com/canyon-project/rust-istanbul-sourcemap

# 下载动态库 (自动)
cd your-project
go run github.com/canyon-project/rust-istanbul-sourcemap/download_lib.go

# 或使用安装脚本
curl -sSL https://raw.githubusercontent.com/canyon-project/rust-istanbul-sourcemap/main/install.sh | bash
```

### 命令行用户

```bash
# 下载二进制文件
wget https://github.com/canyon-project/rust-istanbul-sourcemap/releases/latest/download/istanbul-sourcemap-go_linux_amd64.tar.gz
tar -xzf istanbul-sourcemap-go_linux_amd64.tar.gz
sudo mv istanbul-sourcemap-go /usr/local/bin/

# 使用
istanbul-sourcemap-go -input coverage.json -output transformed.json
```

### 其他语言开发者

下载对应平台的动态库文件，参考 `examples/go/` 中的集成示例。

## 版本管理

- 使用语义化版本 (Semantic Versioning)
- 格式: `v<major>.<minor>.<patch>`
- 例如: `v0.1.0`, `v1.0.0`, `v1.2.3`

## 平台支持矩阵

| 平台 | Go模块 | CLI工具 | 动态库 | 状态 |
|------|--------|---------|--------|------|
| Linux x86_64 | ✅ | ✅ | ✅ | 完全支持 |
| Linux ARM64 | ✅ | ✅ | ✅ | 完全支持 |
| macOS x86_64 | ✅ | ✅ | ✅ | 完全支持 |
| macOS ARM64 | ✅ | ✅ | ✅ | 完全支持 |
| Windows x86_64 | ✅ | ✅ | ✅ | 完全支持 |
| Windows ARM64 | ❌ | ❌ | ❌ | 计划中 |

## 故障排除

### 动态库下载失败

如果自动下载失败，用户可以：

1. 手动从GitHub Releases下载
2. 设置代理或镜像
3. 使用本地构建

### CGO编译问题

确保系统有C编译器：
- Linux: `gcc`
- macOS: Xcode Command Line Tools
- Windows: MinGW或Visual Studio

## 未来计划

- [ ] 发布到Homebrew官方tap
- [ ] 发布到Arch Linux AUR
- [ ] 支持更多Linux发行版的包管理器
- [ ] 添加Windows包管理器支持 (Chocolatey, Scoop)
- [ ] 提供Docker镜像
- [ ] 支持更多CPU架构