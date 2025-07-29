use istanbul_sourcemap::{transform_istanbul_coverage, CoverageMap, SourceMapStore};

fn main() -> anyhow::Result<()> {
    example_usage()?;
    Ok(())
}

fn example_usage() -> anyhow::Result<()> {
    // 模拟一个带有inputSourceMap的Istanbul覆盖率数据
    let istanbul_data = r#"{
        "dist/bundle.js": {
            "path": "dist/bundle.js",
            "statementMap": {
                "0": {"start": {"line": 1, "column": 0}, "end": {"line": 1, "column": 25}},
                "1": {"start": {"line": 2, "column": 0}, "end": {"line": 2, "column": 20}},
                "2": {"start": {"line": 3, "column": 0}, "end": {"line": 3, "column": 15}}
            },
            "fnMap": {
                "0": {
                    "name": "testFunction",
                    "decl": {"start": {"line": 1, "column": 9}, "end": {"line": 1, "column": 21}},
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
            "s": {"0": 5, "1": 3, "2": 0},
            "f": {"0": 2},
            "b": {"0": [3, 1]},
            "inputSourceMap": {
                "version": 3,
                "sources": ["src/main.ts", "src/utils.ts"],
                "names": ["testFunction", "console", "log", "helper"],
                "mappings": "AAAA,SAASA,aACP,OAAOC,QAAQC,IAAI,SAAUC",
                "file": "bundle.js",
                "sourceRoot": "",
                "sourcesContent": [
                    "function testFunction() {\n  if (condition) {\n    return console.log('Hello');\n  }\n}",
                    "export function helper() {\n  return 'helper';\n}"
                ]
            }
        }
    }"#;

    println!("=== Istanbul Source Map 转换示例 ===\n");

    // 解析原始数据
    let coverage_map: CoverageMap = serde_json::from_str(istanbul_data)
        .map_err(|e| anyhow::anyhow!("解析覆盖率数据失败: {}", e))?;

    println!("原始覆盖率数据:");
    print_coverage_summary(&coverage_map);

    // 方法1: 使用便捷函数
    println!("\n--- 方法1: 使用便捷函数 ---");
    let result = transform_istanbul_coverage(istanbul_data)
        .map_err(|e| anyhow::anyhow!("转换失败: {}", e))?;

    let transformed_map: CoverageMap = serde_json::from_str(&result)?;
    print_coverage_summary(&transformed_map);

    // 方法2: 使用SourceMapStore
    println!("\n--- 方法2: 使用SourceMapStore ---");
    let store = SourceMapStore::new();
    let transformed_map2 = store
        .transform_coverage(coverage_map)
        .map_err(|e| anyhow::anyhow!("转换失败: {}", e))?;

    print_coverage_summary(&transformed_map2);

    // 输出完整的JSON结果
    println!("\n--- 完整JSON输出 ---");
    let json_output = serde_json::to_string_pretty(&transformed_map2)?;
    println!("{json_output}");

    // 统计信息
    println!("\n--- 转换统计 ---");
    print_transformation_stats(&transformed_map2);

    Ok(())
}

fn print_coverage_summary(coverage_map: &CoverageMap) {
    for (file_path, fc) in coverage_map {
        println!("📁 文件: {file_path}");

        // 语句覆盖统计
        let total_statements = fc.statement_map.len();
        let covered_statements = fc.s.values().filter(|&&hits| hits > 0).count();
        println!(
            "  📊 语句覆盖: {}/{} ({:.1}%)",
            covered_statements,
            total_statements,
            if total_statements > 0 {
                (covered_statements as f64 / total_statements as f64) * 100.0
            } else {
                0.0
            }
        );

        // 函数覆盖统计
        let total_functions = fc.fn_map.len();
        let covered_functions = fc.f.values().filter(|&&hits| hits > 0).count();
        println!(
            "  🔧 函数覆盖: {}/{} ({:.1}%)",
            covered_functions,
            total_functions,
            if total_functions > 0 {
                (covered_functions as f64 / total_functions as f64) * 100.0
            } else {
                0.0
            }
        );

        // 分支覆盖统计
        let total_branches: usize = fc.b.values().map(|branches| branches.len()).sum();
        let covered_branches: usize =
            fc.b.values()
                .flat_map(|branches| branches.iter())
                .filter(|&&hits| hits > 0)
                .count();
        println!(
            "  🌿 分支覆盖: {}/{} ({:.1}%)",
            covered_branches,
            total_branches,
            if total_branches > 0 {
                (covered_branches as f64 / total_branches as f64) * 100.0
            } else {
                0.0
            }
        );

        // 源映射信息
        if let Some(source_map) = &fc.input_source_map {
            println!("  🗺️  源映射: {} -> {:?}", fc.path, source_map.sources);
        }
        println!();
    }
}

fn print_transformation_stats(coverage_map: &CoverageMap) {
    let total_files = coverage_map.len();
    let original_sources: std::collections::HashSet<_> = coverage_map.keys().collect();

    println!("总文件数: {total_files}");
    println!("转换后的源文件:");
    for source in original_sources {
        println!("  - {source}");
    }

    let total_statements: usize = coverage_map.values().map(|fc| fc.statement_map.len()).sum();
    let total_functions: usize = coverage_map.values().map(|fc| fc.fn_map.len()).sum();
    let total_branches: usize = coverage_map.values().map(|fc| fc.branch_map.len()).sum();

    println!("总计:");
    println!("  语句: {total_statements}");
    println!("  函数: {total_functions}");
    println!("  分支: {total_branches}");
}
