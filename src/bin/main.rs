use istanbul_sourcemap::{transform_istanbul_coverage, CoverageMap, SourceMapStore};

fn main() -> anyhow::Result<()> {
    // 示例Istanbul覆盖率数据
    let istanbul_data = r#"{
        "dist/app.js": {
            "path": "dist/app.js",
            "statementMap": {
                "0": {"start": {"line": 1, "column": 0}, "end": {"line": 1, "column": 25}},
                "1": {"start": {"line": 2, "column": 0}, "end": {"line": 2, "column": 20}},
                "2": {"start": {"line": 3, "column": 0}, "end": {"line": 3, "column": 15}}
            },
            "fnMap": {
                "0": {
                    "name": "myFunction",
                    "decl": {"start": {"line": 1, "column": 9}, "end": {"line": 1, "column": 19}},
                    "loc": {"start": {"line": 1, "column": 0}, "end": {"line": 3, "column": 1}}
                }
            },
            "branchMap": {
                "0": {
                    "type": "if",
                    "loc": {"start": {"line": 2, "column": 0}, "end": {"line": 2, "column": 20}},
                    "locations": [
                        {"start": {"line": 2, "column": 0}, "end": {"line": 2, "column": 10}},
                        {"start": {"line": 2, "column": 10}, "end": {"line": 2, "column": 20}}
                    ]
                }
            },
            "s": {"0": 1, "1": 1, "2": 0},
            "f": {"0": 1},
            "b": {"0": [1, 0]},
            "inputSourceMap": {
                "version": 3,
                "sources": ["src/app.ts"],
                "names": ["myFunction", "console", "log"],
                "mappings": "AAAA,SAASA,WACP,OAAOC,QAAQC,IAAI",
                "file": "app.js",
                "sourceRoot": "",
                "sourcesContent": ["function myFunction() {\n  return console.log('Hello');\n}"]
            }
        }
    }"#;

    println!("原始覆盖率数据:");
    let coverage_map: CoverageMap = serde_json::from_str(istanbul_data)?;
    print_coverage_map(&coverage_map);

    // 使用便捷函数转换
    println!("\n使用便捷函数转换:");
    let result = transform_istanbul_coverage(istanbul_data)?;
    println!("{result}");

    // 使用SourceMapStore转换
    println!("\n使用SourceMapStore转换:");
    let store = SourceMapStore::new();
    let transformed_map = store.transform_coverage(coverage_map)?;
    print_coverage_map(&transformed_map);

    let json_result = serde_json::to_string_pretty(&transformed_map)?;
    println!("\nJSON格式输出:");
    println!("{json_result}");

    Ok(())
}

fn print_coverage_map(coverage_map: &CoverageMap) {
    for (file_path, fc) in coverage_map {
        println!("文件: {file_path}");
        println!("  语句覆盖: {:?}", fc.s);
        println!("  函数覆盖: {:?}", fc.f);
        println!("  分支覆盖: {:?}", fc.b);
        if let Some(source_map) = &fc.input_source_map {
            println!("  源映射: {:?}", source_map.sources);
        }
        println!();
    }
}
