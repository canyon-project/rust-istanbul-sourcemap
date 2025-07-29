use crate::{
    sourcemap::get_mapping, BranchMeta, CoverageMap, FileCoverage, FunctionMeta, Location,
    SourceMap,
};
use anyhow::Result;
use std::collections::HashMap;

/// Source map store for managing transformations
pub struct SourceMapStore {
    base_dir: Option<String>,
    data: HashMap<String, SourceMap>,
}

impl SourceMapStore {
    pub fn new() -> Self {
        Self {
            base_dir: None,
            data: HashMap::new(),
        }
    }

    pub fn with_base_dir(base_dir: String) -> Self {
        Self {
            base_dir: Some(base_dir),
            data: HashMap::new(),
        }
    }

    /// Transform coverage map using source maps
    pub fn transform_coverage(&self, coverage_map: CoverageMap) -> Result<CoverageMap> {
        let has_input_source_maps = coverage_map
            .values()
            .any(|fc| fc.input_source_map.is_some());

        if !has_input_source_maps && self.data.is_empty() {
            return Ok(coverage_map);
        }

        let transformer = SourceMapTransformer::new();
        let mut unique_files: HashMap<String, MappedCoverage> = HashMap::new();

        // 使用不同的方法来避免生命周期问题

        for (file_path, fc) in coverage_map {
            if let Some(source_map) = &fc.input_source_map {
                let changed = transformer.process_file(&fc, source_map, &mut unique_files);
                if !changed {
                    println!("File [{}] ignored, nothing could be mapped", file_path);
                }
            } else {
                let key = get_unique_key(&file_path);
                unique_files.insert(key, MappedCoverage::from_file_coverage(fc));
            }
        }

        let mut result = HashMap::new();
        for (_, mc) in unique_files {
            result.insert(mc.file_coverage.path.clone(), mc.file_coverage);
        }

        Ok(result)
    }
}

impl Default for SourceMapStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Source map transformer
pub struct SourceMapTransformer;

impl SourceMapTransformer {
    pub fn new() -> Self {
        Self
    }

    /// Process a single file's coverage data
    pub fn process_file(&self, fc: &FileCoverage, source_map: &SourceMap, unique_files: &mut HashMap<String, MappedCoverage>) -> bool {
        let mut changes = 0;

        // Process statements
        for (s, loc) in &fc.statement_map {
            let hits = fc.s.get(s).copied().unwrap_or(0);
            if let Some(mapping) = get_mapping(source_map, loc, &fc.path) {
                changes += 1;
                let key = get_unique_key(&mapping.source);
                if !unique_files.contains_key(&key) {
                    unique_files.insert(key.clone(), MappedCoverage::new(mapping.source.clone()));
                }
                unique_files.get_mut(&key).unwrap().add_statement(mapping.loc, hits);
            }
        }

        // Process functions
        for (f, fn_meta) in &fc.fn_map {
            let hits = fc.f.get(f).copied().unwrap_or(0);
            let mapping = get_mapping(source_map, &fn_meta.decl, &fc.path);
            let span_mapping = get_mapping(source_map, &fn_meta.loc, &fc.path);

            if let (Some(mapping), Some(span_mapping)) = (mapping, span_mapping) {
                if mapping.source == span_mapping.source {
                    changes += 1;
                    let key = get_unique_key(&mapping.source);
                    if !unique_files.contains_key(&key) {
                        unique_files.insert(key.clone(), MappedCoverage::new(mapping.source.clone()));
                    }
                    unique_files.get_mut(&key).unwrap().add_function(
                        fn_meta.name.clone(),
                        mapping.loc,
                        span_mapping.loc,
                        hits,
                    );
                }
            }
        }

        // Process branches
        for (b, branch_meta) in &fc.branch_map {
            let hits = fc.b.get(b).cloned().unwrap_or_default();
            let mut locs = Vec::new();
            let mut mapped_hits = Vec::new();
            let mut source = None;
            let mut skip = false;

            for (i, loc) in branch_meta.locations.iter().enumerate() {
                if let Some(mapping) = get_mapping(source_map, loc, &fc.path) {
                    if source.is_none() {
                        source = Some(mapping.source.clone());
                    }
                    if source.as_ref() != Some(&mapping.source) {
                        skip = true;
                    }
                    locs.push(mapping.loc);
                    if i < hits.len() {
                        mapped_hits.push(hits[i]);
                    }
                }
            }

            let loc_mapping = if branch_meta.loc.start.line != 0 || branch_meta.loc.start.column != 0 {
                get_mapping(source_map, &branch_meta.loc, &fc.path)
            } else {
                None
            };

            if !skip && !locs.is_empty() {
                if let Some(source) = source {
                    changes += 1;
                    let key = get_unique_key(&source);
                    if !unique_files.contains_key(&key) {
                        unique_files.insert(key.clone(), MappedCoverage::new(source.clone()));
                    }
                    let branch_loc = loc_mapping
                        .map(|m| m.loc)
                        .unwrap_or_else(|| locs[0].clone());
                    unique_files.get_mut(&key).unwrap().add_branch(
                        branch_meta.branch_type.clone(),
                        branch_loc,
                        locs,
                        mapped_hits,
                    );
                }
            }
        }

        changes > 0
    }
}

impl Default for SourceMapTransformer {
    fn default() -> Self {
        Self::new()
    }
}

/// Mapped coverage data
pub struct MappedCoverage {
    pub file_coverage: FileCoverage,
    meta: MappedCoverageMeta,
}

struct MappedCoverageMeta {
    last: LastIndices,
    seen: HashMap<String, usize>,
}

struct LastIndices {
    s: usize,
    f: usize,
    b: usize,
}

impl MappedCoverage {
    pub fn new(path: String) -> Self {
        Self {
            file_coverage: FileCoverage {
                path,
                statement_map: HashMap::new(),
                fn_map: HashMap::new(),
                branch_map: HashMap::new(),
                s: HashMap::new(),
                f: HashMap::new(),
                b: HashMap::new(),
                input_source_map: None,
            },
            meta: MappedCoverageMeta {
                last: LastIndices { s: 0, f: 0, b: 0 },
                seen: HashMap::new(),
            },
        }
    }

    pub fn from_file_coverage(fc: FileCoverage) -> Self {
        Self {
            file_coverage: fc,
            meta: MappedCoverageMeta {
                last: LastIndices { s: 0, f: 0, b: 0 },
                seen: HashMap::new(),
            },
        }
    }

    /// Add statement to mapped coverage
    pub fn add_statement(&mut self, loc: Location, hits: u32) -> usize {
        let key = format!("s:{}", loc_string(&loc));
        
        if let Some(&index) = self.meta.seen.get(&key) {
            let index_str = index.to_string();
            *self.file_coverage.s.entry(index_str).or_insert(0) += hits;
            index
        } else {
            let index = self.meta.last.s;
            self.meta.last.s += 1;
            self.meta.seen.insert(key, index);
            
            let index_str = index.to_string();
            self.file_coverage.statement_map.insert(index_str.clone(), loc);
            self.file_coverage.s.insert(index_str, hits);
            index
        }
    }

    /// Add function to mapped coverage
    pub fn add_function(&mut self, name: String, decl: Location, loc: Location, hits: u32) -> usize {
        let key = format!("f:{}", loc_string(&decl));
        
        if let Some(&index) = self.meta.seen.get(&key) {
            let index_str = index.to_string();
            *self.file_coverage.f.entry(index_str).or_insert(0) += hits;
            index
        } else {
            let index = self.meta.last.f;
            self.meta.last.f += 1;
            self.meta.seen.insert(key, index);
            
            let index_str = index.to_string();
            let fn_name = if name.is_empty() {
                format!("(unknown_{})", index)
            } else {
                name
            };
            
            self.file_coverage.fn_map.insert(
                index_str.clone(),
                FunctionMeta {
                    name: fn_name,
                    decl,
                    loc,
                },
            );
            self.file_coverage.f.insert(index_str, hits);
            index
        }
    }

    /// Add branch to mapped coverage
    pub fn add_branch(
        &mut self,
        branch_type: String,
        loc: Location,
        branch_locations: Vec<Location>,
        hits: Vec<u32>,
    ) -> usize {
        let mut key_parts = vec!["b".to_string()];
        for l in &branch_locations {
            key_parts.push(loc_string(l));
        }
        let key = key_parts.join(":");

        if let Some(&index) = self.meta.seen.get(&key) {
            let index_str = index.to_string();
            if let Some(existing_hits) = self.file_coverage.b.get_mut(&index_str) {
                for (i, hit) in hits.iter().enumerate() {
                    if i < existing_hits.len() {
                        existing_hits[i] += hit;
                    }
                }
            }
            index
        } else {
            let index = self.meta.last.b;
            self.meta.last.b += 1;
            self.meta.seen.insert(key, index);
            
            let index_str = index.to_string();
            self.file_coverage.branch_map.insert(
                index_str.clone(),
                BranchMeta {
                    branch_type,
                    loc,
                    locations: branch_locations,
                },
            );
            self.file_coverage.b.insert(index_str, hits);
            index
        }
    }
}

/// Helper functions
fn loc_string(loc: &Location) -> String {
    format!(
        "{}:{}:{}:{}",
        loc.start.line, loc.start.column, loc.end.line, loc.end.column
    )
}

fn get_unique_key(pathname: &str) -> String {
    pathname.replace(['/', '\\'], "_")
}