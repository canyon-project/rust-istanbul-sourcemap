use criterion::{black_box, criterion_group, criterion_main, Criterion};
use istanbul_sourcemap::{transform_istanbul_coverage, CoverageMap, SourceMapStore};
use serde_json;

fn benchmark_transform_coverage(c: &mut Criterion) {
    let test_data = r#"{
        "dist/app.js": {
            "path": "dist/app.js",
            "statementMap": {
                "0": {"start": {"line": 1, "column": 0}, "end": {"line": 1, "column": 25}},
                "1": {"start": {"line": 2, "column": 0}, "end": {"line": 2, "column": 20}},
                "2": {"start": {"line": 3, "column": 0}, "end": {"line": 3, "column": 15}},
                "3": {"start": {"line": 4, "column": 0}, "end": {"line": 4, "column": 30}},
                "4": {"start": {"line": 5, "column": 0}, "end": {"line": 5, "column": 25}}
            },
            "fnMap": {
                "0": {
                    "name": "function1",
                    "decl": {"start": {"line": 1, "column": 9}, "end": {"line": 1, "column": 18}},
                    "loc": {"start": {"line": 1, "column": 0}, "end": {"line": 3, "column": 1}}
                },
                "1": {
                    "name": "function2",
                    "decl": {"start": {"line": 4, "column": 9}, "end": {"line": 4, "column": 18}},
                    "loc": {"start": {"line": 4, "column": 0}, "end": {"line": 5, "column": 1}}
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
                },
                "1": {
                    "type": "switch",
                    "loc": {"start": {"line": 5, "column": 0}, "end": {"line": 5, "column": 25}},
                    "locations": [
                        {"start": {"line": 5, "column": 0}, "end": {"line": 5, "column": 8}},
                        {"start": {"line": 5, "column": 8}, "end": {"line": 5, "column": 16}},
                        {"start": {"line": 5, "column": 16}, "end": {"line": 5, "column": 25}}
                    ]
                }
            },
            "s": {"0": 10, "1": 8, "2": 5, "3": 12, "4": 3},
            "f": {"0": 5, "1": 7},
            "b": {"0": [8, 2], "1": [4, 3, 5]},
            "inputSourceMap": {
                "version": 3,
                "sources": ["src/main.ts", "src/utils.ts"],
                "names": ["function1", "function2", "console", "log"],
                "mappings": "AAAA,SAASA,aACP,OAAOC,QAAQC,IAAI,SAAUC,aAC7B,OAAOD,QAAQC,IAAI",
                "file": "app.js",
                "sourceRoot": "",
                "sourcesContent": [
                    "function function1() {\n  if (condition) {\n    return true;\n  }\n}",
                    "function function2() {\n  switch (value) {\n    case 1: return 'one';\n    case 2: return 'two';\n    default: return 'other';\n  }\n}"
                ]
            }
        }
    }"#;

    c.bench_function("transform_istanbul_coverage", |b| {
        b.iter(|| {
            transform_istanbul_coverage(black_box(test_data)).unwrap()
        })
    });

    let coverage_map: CoverageMap = serde_json::from_str(test_data).unwrap();
    let store = SourceMapStore::new();

    c.bench_function("source_map_store_transform", |b| {
        b.iter(|| {
            store.transform_coverage(black_box(coverage_map.clone())).unwrap()
        })
    });
}

criterion_group!(benches, benchmark_transform_coverage);
criterion_main!(benches);