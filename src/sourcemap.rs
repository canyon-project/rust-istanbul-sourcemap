use crate::{Location, Mapping, Position, SourceMap};

/// Source map decoder for handling VLQ mappings
pub struct SourceMapDecoder {
    source_map: SourceMap,
}

impl SourceMapDecoder {
    pub fn new(source_map: SourceMap) -> Self {
        Self { source_map }
    }

    /// Get original position for a generated position
    pub fn get_original_position(&self, line: u32, column: u32) -> Option<OriginalPosition> {
        // Simplified implementation - in production you'd need full VLQ decoding
        if self.source_map.sources.is_empty() {
            return None;
        }

        // Simple mapping to first source
        Some(OriginalPosition {
            source: self.source_map.sources[0].clone(),
            line,
            column,
            name: None,
        })
    }

    /// Parse VLQ mappings (simplified version)
    pub fn parse_mappings(&self) -> Result<Vec<Vec<MappingSegment>>, Box<dyn std::error::Error>> {
        let lines: Vec<&str> = self.source_map.mappings.split(';').collect();
        let mut result = Vec::with_capacity(lines.len());

        // State variables for VLQ decoding
        let mut generated_column = 0i32;
        let mut source_index = 0i32;
        let mut original_line = 0i32;
        let mut original_column = 0i32;
        let mut name_index = 0i32;

        for (line_index, line) in lines.iter().enumerate() {
            if line_index > 0 {
                generated_column = 0; // Reset for each line
            }

            let segments: Vec<&str> = line.split(',').collect();
            let mut line_segments = Vec::new();

            for segment in segments {
                if segment.is_empty() {
                    continue;
                }

                let mut index = 0;
                
                // Decode generated column
                let (gen_col, new_index) = decode_vlq(segment, index)?;
                index = new_index;
                generated_column += gen_col;

                let mut mapping_segment = MappingSegment {
                    generated_line: line_index as u32,
                    generated_column: generated_column as u32,
                    source: None,
                    original_line: 0,
                    original_column: 0,
                    name: None,
                };

                // If there's more data, decode source information
                if index < segment.len() {
                    let (src_idx, new_index) = decode_vlq(segment, index)?;
                    index = new_index;
                    source_index += src_idx;

                    if source_index >= 0 && (source_index as usize) < self.source_map.sources.len() {
                        mapping_segment.source = Some(self.source_map.sources[source_index as usize].clone());
                    }

                    // Decode original line
                    let (orig_line, new_index) = decode_vlq(segment, index)?;
                    index = new_index;
                    original_line += orig_line;
                    mapping_segment.original_line = original_line as u32;

                    // Decode original column
                    let (orig_col, new_index) = decode_vlq(segment, index)?;
                    index = new_index;
                    original_column += orig_col;
                    mapping_segment.original_column = original_column as u32;

                    // If there's a name index
                    if index < segment.len() {
                        if let Ok((name_idx, _)) = decode_vlq(segment, index) {
                            name_index += name_idx;
                            if name_index >= 0 && (name_index as usize) < self.source_map.names.len() {
                                mapping_segment.name = Some(self.source_map.names[name_index as usize].clone());
                            }
                        }
                    }
                }

                line_segments.push(mapping_segment);
            }

            result.push(line_segments);
        }

        Ok(result)
    }
}

/// Original position in source code
#[derive(Debug, Clone)]
pub struct OriginalPosition {
    pub source: String,
    pub line: u32,
    pub column: u32,
    pub name: Option<String>,
}

/// Mapping segment from source map
#[derive(Debug, Clone)]
pub struct MappingSegment {
    pub generated_line: u32,
    pub generated_column: u32,
    pub source: Option<String>,
    pub original_line: u32,
    pub original_column: u32,
    pub name: Option<String>,
}

/// Get mapping from source map for a generated location
pub fn get_mapping(source_map: &SourceMap, generated_location: &Location, _orig_file: &str) -> Option<Mapping> {
    if source_map.sources.is_empty() {
        return None;
    }

    let decoder = SourceMapDecoder::new(source_map.clone());
    
    // Get mapping for start position
    let start_pos = decoder.get_original_position(
        generated_location.start.line,
        generated_location.start.column,
    )?;

    // Get mapping for end position
    let end_pos = decoder.get_original_position(
        generated_location.end.line,
        generated_location.end.column,
    )?;

    // Ensure both positions map to the same source
    if start_pos.source != end_pos.source {
        return None;
    }

    Some(Mapping {
        source: relative_to(&start_pos.source, _orig_file),
        loc: Location {
            start: Position {
                line: start_pos.line,
                column: start_pos.column,
            },
            end: Position {
                line: end_pos.line,
                column: end_pos.column,
            },
        },
    })
}

/// Calculate relative path (simplified)
fn relative_to(source: &str, _orig_file: &str) -> String {
    source.to_string()
}

/// VLQ decoding constants
const VLQ_BASE_SHIFT: u32 = 5;
const VLQ_BASE: u32 = 1 << VLQ_BASE_SHIFT;
const VLQ_BASE_MASK: u32 = VLQ_BASE - 1;
const VLQ_CONTINUATION_BIT: u32 = VLQ_BASE;

/// Base64 character mapping
fn get_base64_value(c: char) -> Result<u32, Box<dyn std::error::Error>> {
    match c {
        'A'..='Z' => Ok(c as u32 - 'A' as u32),
        'a'..='z' => Ok(c as u32 - 'a' as u32 + 26),
        '0'..='9' => Ok(c as u32 - '0' as u32 + 52),
        '+' => Ok(62),
        '/' => Ok(63),
        _ => Err(format!("Invalid base64 character: {}", c).into()),
    }
}

/// Decode VLQ encoded integer
fn decode_vlq(mappings: &str, mut index: usize) -> Result<(i32, usize), Box<dyn std::error::Error>> {
    let mut result = 0u32;
    let mut shift = 0u32;
    let mut continuation = true;

    while continuation && index < mappings.len() {
        let c = mappings.chars().nth(index).unwrap();
        let value = get_base64_value(c)?;
        
        continuation = (value & VLQ_CONTINUATION_BIT) != 0;
        let masked_value = value & VLQ_BASE_MASK;
        result += masked_value << shift;
        shift += VLQ_BASE_SHIFT;
        index += 1;
    }

    // Handle sign bit
    let final_result = if (result & 1) == 1 {
        -((result >> 1) as i32)
    } else {
        (result >> 1) as i32
    };

    Ok((final_result, index))
}