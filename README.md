# Rust Istanbul Source Map Transformer

这是一个用Rust语言实现的Istanbul覆盖率数据source map转换器，提供与JavaScript版本的`istanbul-lib-source-maps`库类似的功能。

## 特性

- 🦀 **类型安全**: 使用Rust的强类型系统确保数据安全
- ⚡ **高性能**: Rust的零成本抽象和内存安全
- 🔄 **Source Map转换**: 将生成代码的覆盖率数据转换为原始源代码的覆盖率数据
- 📊 **完整支持**: 支持语句(statements)、函数(functions)和分支(branches)覆盖率
- 🛠️ **易于使用**: 提供简洁的API接口

## 安装

将以下内容添加到你的 `Cargo.toml`:

```toml
[dependencies]
istanbul-sourcemap = "0.1.0"
```

## 使用方法

### 基本用法

```rust
use istanbul_sourcemap::{transform_istanbul_coverage, SourceMapStore};
use serde_json;

fn main() -> anyhow::Result<()> {
    // Istanbul覆盖率JSON数据（带有inputSourceMap）
    let istanbul_data = r#"{
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
    }"#;

    // 方法1: 使用便捷函数
    let result = transform_istanbul_coverage(istanbul_data)?;
    println!("{}", result);

    // 方法2: 使用SourceMapStore
    let coverage_map = serde_json::from_str(istanbul_data)?;
    let store = SourceMapStore::new();
    let transformed_map = store.transform_coverage(coverage_map)?;
    
    let json_result = serde_json::to_string_pretty(&transformed_map)?;
    println!("{}", json_result);

    Ok(())
}
```

### 高级用法

```rust
use istanbul_sourcemap::{SourceMapStore, CoverageMap};

fn advanced_usage() -> anyhow::Result<()> {
    // 创建带有基础目录的store
    let store = SourceMapStore::with_base_dir("./src".to_string());
    
    // 从文件读取覆盖率数据
    let coverage_data = std::fs::read_to_string("coverage.json")?;
    let coverage_map: CoverageMap = serde_json::from_str(&coverage_data)?;
    
    // 转换覆盖率数据
    let transformed = store.transform_coverage(coverage_map)?;
    
    // 保存结果
    let output = serde_json::to_string_pretty(&transformed)?;
    std::fs::write("transformed_coverage.json", output)?;
    
    Ok(())
}
```

## 数据结构

### 主要类型

```rust
// 覆盖率映射
pub type CoverageMap = HashMap<String, FileCoverage>;

// 文件覆盖率数据
pub struct FileCoverage {
    pub path: String,
    pub statement_map: HashMap<String, Location>,
    pub fn_map: HashMap<String, FunctionMeta>,
    pub branch_map: HashMap<String, BranchMeta>,
    pub s: HashMap<String, u32>,              // 语句命中次数
    pub f: HashMap<String, u32>,              // 函数命中次数
    pub b: HashMap<String, Vec<u32>>,         // 分支命中次数
    pub input_source_map: Option<SourceMap>,
}

// 位置信息
pub struct Location {
    pub start: Position,
    pub end: Position,
}

pub struct Position {
    pub line: u32,
    pub column: u32,
}
```

## API 参考

### 函数

- `create_source_map_store() -> SourceMapStore` - 创建新的source map store
- `transform_coverage(coverage_map: CoverageMap) -> Result<CoverageMap>` - 转换覆盖率数据
- `transform_istanbul_coverage(json_data: &str) -> Result<String>` - 便捷的JSON转换函数

### SourceMapStore 方法

- `new() -> Self` - 创建新实例
- `with_base_dir(base_dir: String) -> Self` - 创建带基础目录的实例
- `transform_coverage(&self, coverage_map: CoverageMap) -> Result<CoverageMap>` - 转换覆盖率数据

## 运行示例

```bash
# 运行主示例
cargo run

# 运行详细示例
cargo run --bin example

# 运行测试
cargo test

# 运行测试并显示输出
cargo test -- --nocapture
```

## 测试

项目包含完整的测试套件：

```bash
# 运行所有测试
cargo test

# 运行集成测试
cargo test --test integration_tests

# 运行特定测试
cargo test test_transform_coverage_with_source_map
```

## 性能特点

- **零拷贝**: 尽可能避免不必要的数据复制
- **内存安全**: Rust的所有权系统防止内存泄漏
- **并发安全**: 所有类型都是Send + Sync（在适当的情况下）
- **错误处理**: 使用Result类型进行优雅的错误处理

## 与JavaScript版本的对比

| 特性 | JavaScript版本 | Rust版本 |
|------|---------------|----------|
| 类型安全 | ❌ | ✅ |
| 内存安全 | ❌ | ✅ |
| 性能 | 中等 | 高 |
| 并发 | 有限 | 优秀 |
| 生态系统 | 丰富 | 成长中 |
| 学习曲线 | 低 | 中等 |

## 注意事项

1. **Source Map解码**: 当前实现使用了简化的VLQ解码。在生产环境中，建议使用更完整的source map解析库。

2. **错误处理**: 使用`anyhow`和`thiserror`进行错误处理，提供详细的错误信息。

3. **序列化**: 使用`serde`进行JSON序列化/反序列化，支持自定义字段名。

## 扩展功能

如果需要更完整的功能，可以考虑以下扩展：

1. 完整的VLQ解码实现
2. 异步I/O支持
3. 更高级的source map缓存
4. 插件系统
5. CLI工具

## 贡献

欢迎提交Issue和Pull Request！

## 许可证

MIT License

## 更新日志

### v0.1.0
- 初始版本
- 基本的source map转换功能
- 完整的测试套件
- 文档和示例