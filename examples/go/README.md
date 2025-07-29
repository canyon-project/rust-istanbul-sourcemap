# Go语言调用示例

这个示例展示了如何在Go语言中调用Rust构建的动态库。

## 构建步骤

1. 首先构建Rust动态库：
```bash
cargo build --release
```

2. 复制动态库到Go项目目录：
```bash
# Linux
cp ../../target/release/libistanbul_sourcemap.so .

# macOS
cp ../../target/release/libistanbul_sourcemap.dylib ./libistanbul_sourcemap.so

# Windows
cp ../../target/release/istanbul_sourcemap.dll .
```

3. 运行Go程序：
```bash
go run main.go
```

## 使用GitHub Action构建的产物

如果你使用GitHub Action构建的产物，下载对应平台的动态库文件并重命名：

- Linux AMD64: `libistanbul_sourcemap-linux-amd64.so` → `libistanbul_sourcemap.so`
- Linux ARM64: `libistanbul_sourcemap-linux-arm64.so` → `libistanbul_sourcemap.so`
- macOS AMD64: `libistanbul_sourcemap-darwin-amd64.dylib` → `libistanbul_sourcemap.so`
- macOS ARM64: `libistanbul_sourcemap-darwin-arm64.dylib` → `libistanbul_sourcemap.so`
- Windows: `istanbul_sourcemap-windows-amd64.dll` → `istanbul_sourcemap.dll`

## API说明

### TransformCoverage(input string) (string, error)
转换Istanbul覆盖率数据，应用源映射。

### GetVersion() string
获取库版本号。

## 注意事项

1. 确保动态库文件在Go程序的工作目录中
2. 在Linux/macOS上可能需要设置`LD_LIBRARY_PATH`或`DYLD_LIBRARY_PATH`
3. CGO需要C编译器支持