use tower_lsp::lsp_types::*;
use ducky_core::lexer::{tokenize_line, TokenType};

/// Get linked editing ranges for simultaneous editing of related symbols
pub fn get_linked_editing_ranges(content: &str, position: Position) -> Option<LinkedEditingRanges> {
    let lines: Vec<&str> = content.lines().collect();
    
    if let Some(line) = lines.get(position.line as usize) {
        let trimmed = line.trim();
        let token_type = tokenize_line(trimmed);
        
        // Find matching block start/end pairs
        match token_type {
            TokenType::If => {
                find_block_pair(&lines, position.line, TokenType::If, TokenType::EndIf, "IF")
            }
            TokenType::EndIf => {
                find_block_pair_reverse(&lines, position.line, TokenType::If, TokenType::EndIf, "IF")
            }
            TokenType::While => {
                find_block_pair(&lines, position.line, TokenType::While, TokenType::EndWhile, "WHILE")
            }
            TokenType::EndWhile => {
                find_block_pair_reverse(&lines, position.line, TokenType::While, TokenType::EndWhile, "WHILE")
            }
            TokenType::Function => {
                find_function_pair(&lines, position.line)
            }
            TokenType::EndFunction => {
                find_function_pair_reverse(&lines, position.line)
            }
            TokenType::Extension => {
                find_block_pair(&lines, position.line, TokenType::Extension, TokenType::EndExtension, "EXTENSION")
            }
            TokenType::EndExtension => {
                find_block_pair_reverse(&lines, position.line, TokenType::Extension, TokenType::EndExtension, "EXTENSION")
            }
            TokenType::Stage => {
                find_block_pair(&lines, position.line, TokenType::Stage, TokenType::EndStage, "STAGE")
            }
            TokenType::EndStage => {
                find_block_pair_reverse(&lines, position.line, TokenType::Stage, TokenType::EndStage, "STAGE")
            }
            TokenType::ButtonDef => {
                find_block_pair(&lines, position.line, TokenType::ButtonDef, TokenType::EndButtonDef, "BUTTON_DEF")
            }
            TokenType::EndButtonDef => {
                find_block_pair_reverse(&lines, position.line, TokenType::ButtonDef, TokenType::EndButtonDef, "BUTTON_DEF")
            }
            TokenType::StringBlock | TokenType::StringLnBlock => {
                find_block_pair(&lines, position.line, token_type, TokenType::EndString, "STRING")
            }
            TokenType::EndString => {
                find_string_block_reverse(&lines, position.line)
            }
            TokenType::RemBlock => {
                find_block_pair(&lines, position.line, TokenType::RemBlock, TokenType::EndRemBlock, "REM_BLOCK")
            }
            TokenType::EndRemBlock => {
                find_block_pair_reverse(&lines, position.line, TokenType::RemBlock, TokenType::EndRemBlock, "REM_BLOCK")
            }
            _ => None,
        }
    } else {
        None
    }
}

/// Find the matching end for a block start
fn find_block_pair(
    lines: &[&str],
    start_line: u32,
    start_token: TokenType,
    end_token: TokenType,
    keyword: &str,
) -> Option<LinkedEditingRanges> {
    let mut depth = 1;
    
    for (idx, line) in lines.iter().enumerate().skip(start_line as usize + 1) {
        let trimmed = line.trim();
        let token_type = tokenize_line(trimmed);
        
        if token_type == start_token {
            depth += 1;
        } else if token_type == end_token {
            depth -= 1;
            if depth == 0 {
                // Found matching end
                let start_range = get_keyword_range(lines[start_line as usize], start_line, keyword);
                let end_range = get_keyword_range(line, idx as u32, &format!("END_{}", keyword));
                
                return Some(LinkedEditingRanges {
                    ranges: vec![start_range, end_range],
                    word_pattern: None,
                });
            }
        }
    }
    
    None
}

/// Find the matching start for a block end (reverse search)
fn find_block_pair_reverse(
    lines: &[&str],
    end_line: u32,
    start_token: TokenType,
    end_token: TokenType,
    keyword: &str,
) -> Option<LinkedEditingRanges> {
    let mut depth = 1;
    
    for idx in (0..end_line as usize).rev() {
        let line = lines[idx];
        let trimmed = line.trim();
        let token_type = tokenize_line(trimmed);
        
        if token_type == end_token {
            depth += 1;
        } else if token_type == start_token {
            depth -= 1;
            if depth == 0 {
                // Found matching start
                let start_range = get_keyword_range(line, idx as u32, keyword);
                let end_range = get_keyword_range(lines[end_line as usize], end_line, &format!("END_{}", keyword));
                
                return Some(LinkedEditingRanges {
                    ranges: vec![start_range, end_range],
                    word_pattern: None,
                });
            }
        }
    }
    
    None
}

/// Find function name in FUNCTION declaration and END_FUNCTION
fn find_function_pair(lines: &[&str], start_line: u32) -> Option<LinkedEditingRanges> {
    let start_line_text = lines[start_line as usize];
    let trimmed = start_line_text.trim();
    
    // Extract function name from "FUNCTION name"
    if let Some(func_name) = trimmed.strip_prefix("FUNCTION ").map(|s| s.trim()) {
        // Find matching END_FUNCTION
        let mut depth = 1;
        
        for (_idx, line) in lines.iter().enumerate().skip(start_line as usize + 1) {
            let trimmed = line.trim();
            let token_type = tokenize_line(trimmed);
            
            if token_type == TokenType::Function {
                depth += 1;
            } else if token_type == TokenType::EndFunction {
                depth -= 1;
                if depth == 0 {
                    // Found matching end - link function names
                    let start_range = get_function_name_range(start_line_text, start_line);
                    
                    return Some(LinkedEditingRanges {
                        ranges: vec![start_range],
                        word_pattern: Some(func_name.to_string()),
                    });
                }
            }
        }
    }
    
    None
}

/// Find function start from END_FUNCTION
fn find_function_pair_reverse(lines: &[&str], end_line: u32) -> Option<LinkedEditingRanges> {
    let mut depth = 1;
    
    for idx in (0..end_line as usize).rev() {
        let line = lines[idx];
        let trimmed = line.trim();
        let token_type = tokenize_line(trimmed);
        
        if token_type == TokenType::EndFunction {
            depth += 1;
        } else if token_type == TokenType::Function {
            depth -= 1;
            if depth == 0 {
                // Found matching start
                if let Some(func_name) = trimmed.strip_prefix("FUNCTION ").map(|s| s.trim()) {
                    let start_range = get_function_name_range(line, idx as u32);
                    
                    return Some(LinkedEditingRanges {
                        ranges: vec![start_range],
                        word_pattern: Some(func_name.to_string()),
                    });
                }
            }
        }
    }
    
    None
}

/// Find string block start from END_STRING
fn find_string_block_reverse(lines: &[&str], end_line: u32) -> Option<LinkedEditingRanges> {
    for idx in (0..end_line as usize).rev() {
        let line = lines[idx];
        let trimmed = line.trim();
        let token_type = tokenize_line(trimmed);
        
        if matches!(token_type, TokenType::StringBlock | TokenType::StringLnBlock) {
            let start_range = get_keyword_range(line, idx as u32, "STRING");
            let end_range = get_keyword_range(lines[end_line as usize], end_line, "END_STRING");
            
            return Some(LinkedEditingRanges {
                ranges: vec![start_range, end_range],
                word_pattern: None,
            });
        }
    }
    
    None
}

/// Get the range of a keyword in a line
fn get_keyword_range(line: &str, line_num: u32, keyword: &str) -> Range {
    let start_char = line.find(keyword).unwrap_or(0) as u32;
    
    Range {
        start: Position {
            line: line_num,
            character: start_char,
        },
        end: Position {
            line: line_num,
            character: start_char + keyword.len() as u32,
        },
    }
}

/// Get the range of the function name in a FUNCTION declaration
fn get_function_name_range(line: &str, line_num: u32) -> Range {
    let trimmed = line.trim();
    
    if let Some(func_name) = trimmed.strip_prefix("FUNCTION ") {
        let name = func_name.trim();
        let start_char = line.find(name).unwrap_or(0) as u32;
        
        Range {
            start: Position {
                line: line_num,
                character: start_char,
            },
            end: Position {
                line: line_num,
                character: start_char + name.len() as u32,
            },
        }
    } else {
        Range {
            start: Position { line: line_num, character: 0 },
            end: Position { line: line_num, character: 0 },
        }
    }
}
