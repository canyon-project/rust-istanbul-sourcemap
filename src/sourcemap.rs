use crate::{Location, Mapping, Position, SourceMap};
use sourcemap::vlq::parse_vlq_segment;

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

    /// Parse VLQ mappings using sourcemap crate's VLQ decoder
    pub fn parse_mappings(&self) -> Result<Vec<Vec<MappingSegment>>, Box<dyn std::error::Error>> {
        let lines: Vec<&str> = self.source_map.mappings.split(';').collect();
        let mut result = Vec::with_capacity(lines.len());

        // State variables for VLQ decoding
        let mut generated_column = 0i64;
        let mut source_index = 0i64;
        let mut original_line = 0i64;
        let mut original_column = 0i64;
        let mut name_index = 0i64;

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

                // Use sourcemap crate's parse_vlq_segment function
                let decoded_values =
                    parse_vlq_segment(segment).map_err(|e| format!("VLQ decode error: {e:?}"))?;

                if decoded_values.is_empty() {
                    continue;
                }

                // Update generated column
                generated_column += decoded_values[0];

                let mut mapping_segment = MappingSegment {
                    generated_line: line_index as u32,
                    generated_column: generated_column as u32,
                    source: None,
                    original_line: 0,
                    original_column: 0,
                    name: None,
                };

                // If there's source information (at least 4 values)
                if decoded_values.len() >= 4 {
                    source_index += decoded_values[1];
                    original_line += decoded_values[2];
                    original_column += decoded_values[3];

                    if source_index >= 0 && (source_index as usize) < self.source_map.sources.len()
                    {
                        mapping_segment.source =
                            Some(self.source_map.sources[source_index as usize].clone());
                    }

                    mapping_segment.original_line = original_line as u32;
                    mapping_segment.original_column = original_column as u32;

                    // If there's a name index (5th value)
                    if decoded_values.len() >= 5 {
                        name_index += decoded_values[4];
                        if name_index >= 0 && (name_index as usize) < self.source_map.names.len() {
                            mapping_segment.name =
                                Some(self.source_map.names[name_index as usize].clone());
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
pub fn get_mapping(
    source_map: &SourceMap,
    generated_location: &Location,
    _orig_file: &str,
) -> Option<Mapping> {
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
    let end_pos = decoder
        .get_original_position(generated_location.end.line, generated_location.end.column)?;

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
