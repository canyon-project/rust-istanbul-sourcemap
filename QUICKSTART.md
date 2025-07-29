# 快速开始指南

## 🚀 发布你的第一个版本

### 1. 提交所有更改

```bash
git add .
git commit -m "feat: 添加Go语言绑定和多平台发布支持"
git push origin main
```

### 2. 发布版本

```bash
./scripts/publish.sh v0.1.0
```

这会自动：
- 创建Git tag
- 触发GitHub Actions
- 构建多平台二进制文件和动态库
- 创建GitHub Release

### 3. 验证发布

等待GitHub Actions完成后，检查：
- [GitHub Releases](https://github.com/canyon-project/rust-istanbul-sourcemap/releases) 页面
- 确保所有平台的文件都已上传

## 📦 用户如何安装

### Go开发者

```bash
# 安装Go包
go get github.com/canyon-project/rust-istanbul-sourcemap

# 在项目中使用
cd your-go-project
go run github.com/canyon-project/rust-istanbul-sourcemap/download_lib.go
```

### 命令行用户

```bash
# 下载对应平台的二进制文件
wget https://github.com/canyon-project/rust-istanbul-sourcemap/releases/latest/download/istanbul-sourcemap-go_linux_amd64.tar.gz
tar -xzf istanbul-sourcemap-go_linux_amd64.tar.gz
sudo mv istanbul-sourcemap-go /usr/local/bin/

# 使用
istanbul-sourcemap-go -input coverage.json -output transformed.json
```

## 🧪 测试安装

创建一个测试项目来验证：

```bash
mkdir test-installation
cd test-installation
go mod init test
go get github.com/canyon-project/rust-istanbul-sourcemap@v0.1.0
```

创建测试文件 `main.go`：

```go
package main

import (
    "fmt"
    "log"
    
    istanbul "github.com/canyon-project/rust-istanbul-sourcemap"
)

func main() {
    ism := istanbul.New()
    fmt.Printf("Library version: %s\n", ism.GetVersion())
    fmt.Printf("Platform: %s\n", ism.GetPlatform())
}
```

运行测试：

```bash
go run github.com/canyon-project/rust-istanbul-sourcemap/download_lib.go
go run main.go
```

## 📋 发布检查清单

发布前确保：

- [ ] 所有测试通过
- [ ] 文档已更新
- [ ] 版本号符合语义化版本规范
- [ ] GitHub Actions配置正确
- [ ] FFI接口完整
- [ ] 示例代码可运行

## 🔧 故障排除

### GitHub Actions失败

1. 检查Actions日志
2. 确保所有依赖正确安装
3. 验证Rust和Go代码编译通过

### 动态库加载失败

1. 确保`download_lib.go`正确下载了库文件
2. 检查库文件权限
3. 验证CGO配置

### 版本发布问题

1. 确保tag格式正确 (`v0.1.0`)
2. 检查GitHub token权限
3. 验证仓库设置

## 📈 后续步骤

发布成功后可以考虑：

1. **添加更多平台支持**
   - Windows ARM64
   - 更多Linux发行版

2. **改进文档**
   - 添加更多使用示例
   - 创建API文档
   - 录制演示视频

3. **社区推广**
   - 发布到Reddit、HackerNews
   - 写技术博客
   - 参与相关社区讨论

4. **功能增强**
   - 添加更多source map功能
   - 性能优化
   - 错误处理改进

## 🎯 成功指标

发布成功的标志：

- ✅ GitHub Release页面显示所有平台的文件
- ✅ 用户可以通过`go get`安装
- ✅ CLI工具可以正常运行
- ✅ 动态库可以被Go代码调用
- ✅ 所有平台的测试通过

恭喜！你现在有了一个完整的多平台发布系统！🎉