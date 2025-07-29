use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod sourcemap;
pub mod transformer;
pub mod ffi;

pub use sourcemap::*;
pub use transformer::*;

/// Position in source code (line, column)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

/// Location range in source code
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub start: Position,
    pub end: Position,
}

/// Function metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionMeta {
    pub name: String,
    pub decl: Location,
    pub loc: Location,
}

/// Branch metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchMeta {
    #[serde(rename = "type")]
    pub branch_type: String,
    pub loc: Location,
    pub locations: Vec<Location>,
}

/// Source map structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMap {
    pub version: u32,
    pub sources: Vec<String>,
    pub names: Vec<String>,
    pub mappings: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(rename = "sourceRoot", skip_serializing_if = "Option::is_none")]
    pub source_root: Option<String>,
    #[serde(rename = "sourcesContent", skip_serializing_if = "Option::is_none")]
    pub sources_content: Option<Vec<String>>,
}

/// File coverage data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCoverage {
    pub path: String,
    #[serde(rename = "statementMap")]
    pub statement_map: HashMap<String, Location>,
    #[serde(rename = "fnMap")]
    pub fn_map: HashMap<String, FunctionMeta>,
    #[serde(rename = "branchMap")]
    pub branch_map: HashMap<String, BranchMeta>,
    pub s: HashMap<String, u32>, // statement hits
    pub f: HashMap<String, u32>, // function hits
    pub b: HashMap<String, Vec<u32>>, // branch hits
    #[serde(rename = "inputSourceMap", skip_serializing_if = "Option::is_none")]
    pub input_source_map: Option<SourceMap>,
}

/// Coverage map (file path -> file coverage)
pub type CoverageMap = HashMap<String, FileCoverage>;

/// Mapping result from source map
#[derive(Debug, Clone)]
pub struct Mapping {
    pub source: String,
    pub loc: Location,
}

/// Create a new source map store
pub fn create_source_map_store() -> SourceMapStore {
    SourceMapStore::new()
}

/// Transform Istanbul coverage data with source maps
pub fn transform_coverage(coverage_map: CoverageMap) -> Result<CoverageMap> {
    let store = create_source_map_store();
    store.transform_coverage(coverage_map)
}

/// Convenience function to transform JSON coverage data
pub fn transform_istanbul_coverage(json_data: &str) -> Result<String> {
    let coverage_map: CoverageMap = serde_json::from_str(json_data)?;
    let transformed = transform_coverage(coverage_map)?;
    Ok(serde_json::to_string_pretty(&transformed)?)
}