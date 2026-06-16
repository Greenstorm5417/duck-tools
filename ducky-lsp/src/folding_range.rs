use ducky_core::lexer::{TokenType, tokenize_line};
use tower_lsp::lsp_types::*;

/// Generate folding ranges for code blocks
pub fn get_folding_ranges(content: &str) -> Vec<FoldingRange> {
    let mut ranges = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let token_type = tokenize_line(line);

        match token_type {
            TokenType::Function => {
                if let Some(end) = find_matching_end(&lines, i, TokenType::EndFunction) {
                    ranges.push(FoldingRange {
                        start_line: i as u32,
                        start_character: None,
                        end_line: end as u32,
                        end_character: None,
                        kind: Some(FoldingRangeKind::Region),
                        collapsed_text: Some("function...".to_string()),
                    });
                    i = end;
                }
            }
            TokenType::If => {
                if let Some(end) = find_matching_end(&lines, i, TokenType::EndIf) {
                    ranges.push(FoldingRange {
                        start_line: i as u32,
                        start_character: None,
                        end_line: end as u32,
                        end_character: None,
                        kind: Some(FoldingRangeKind::Region),
                        collapsed_text: Some("if...".to_string()),
                    });
                    i = end;
                }
            }
            TokenType::While => {
                if let Some(end) = find_matching_end(&lines, i, TokenType::EndWhile) {
                    ranges.push(FoldingRange {
                        start_line: i as u32,
                        start_character: None,
                        end_line: end as u32,
                        end_character: None,
                        kind: Some(FoldingRangeKind::Region),
                        collapsed_text: Some("while...".to_string()),
                    });
                    i = end;
                }
            }
            TokenType::Extension => {
                if let Some(end) = find_matching_end(&lines, i, TokenType::EndExtension) {
                    ranges.push(FoldingRange {
                        start_line: i as u32,
                        start_character: None,
                        end_line: end as u32,
                        end_character: None,
                        kind: Some(FoldingRangeKind::Region),
                        collapsed_text: Some("extension...".to_string()),
                    });
                    i = end;
                }
            }
            TokenType::Stage => {
                if let Some(end) = find_matching_end(&lines, i, TokenType::EndStage) {
                    ranges.push(FoldingRange {
                        start_line: i as u32,
                        start_character: None,
                        end_line: end as u32,
                        end_character: None,
                        kind: Some(FoldingRangeKind::Region),
                        collapsed_text: Some("stage...".to_string()),
                    });
                    i = end;
                }
            }
            TokenType::ButtonDef => {
                if let Some(end) = find_matching_end(&lines, i, TokenType::EndButtonDef) {
                    ranges.push(FoldingRange {
                        start_line: i as u32,
                        start_character: None,
                        end_line: end as u32,
                        end_character: None,
                        kind: Some(FoldingRangeKind::Region),
                        collapsed_text: Some("button...".to_string()),
                    });
                    i = end;
                }
            }
            TokenType::RemBlock => {
                if let Some(end) = find_matching_end(&lines, i, TokenType::EndRemBlock) {
                    ranges.push(FoldingRange {
                        start_line: i as u32,
                        start_character: None,
                        end_line: end as u32,
                        end_character: None,
                        kind: Some(FoldingRangeKind::Comment),
                        collapsed_text: Some("/* ... */".to_string()),
                    });
                    i = end;
                }
            }
            TokenType::StringBlock | TokenType::StringLnBlock => {
                if let Some(end) = find_matching_end(&lines, i, TokenType::EndString) {
                    ranges.push(FoldingRange {
                        start_line: i as u32,
                        start_character: None,
                        end_line: end as u32,
                        end_character: None,
                        kind: Some(FoldingRangeKind::Region),
                        collapsed_text: Some("string...".to_string()),
                    });
                    i = end;
                }
            }
            _ => {}
        }

        i += 1;
    }

    ranges
}

fn find_matching_end(lines: &[&str], start: usize, end_type: TokenType) -> Option<usize> {
    ((start + 1)..lines.len()).find(|&i| tokenize_line(lines[i]) == end_type)
}
