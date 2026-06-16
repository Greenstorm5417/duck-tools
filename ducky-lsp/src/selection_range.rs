use ducky_core::lexer::{TokenType, tokenize_line};
use tower_lsp::lsp_types::*;

/// Get selection ranges for smart select/expand selection
pub fn get_selection_ranges(content: &str, positions: Vec<Position>) -> Vec<SelectionRange> {
    positions
        .into_iter()
        .map(|pos| get_selection_range_at_position(content, pos))
        .collect()
}

/// Get selection range hierarchy for a single position
fn get_selection_range_at_position(content: &str, position: Position) -> SelectionRange {
    let lines: Vec<&str> = content.lines().collect();

    if let Some(line) = lines.get(position.line as usize) {
        // Level 1: Current word
        let word_range = get_word_range(line, position);

        // Level 2: Current line (trimmed)
        let line_range = get_line_range(line, position.line);

        // Level 3: Current block (IF/WHILE/FUNCTION)
        let block_range = get_block_range(&lines, position.line);

        // Level 4: Entire document
        let document_range = Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: lines.len() as u32,
                character: 0,
            },
        };

        // Build hierarchy from innermost to outermost
        SelectionRange {
            range: word_range,
            parent: Some(Box::new(SelectionRange {
                range: line_range,
                parent: Some(Box::new(SelectionRange {
                    range: block_range,
                    parent: Some(Box::new(SelectionRange {
                        range: document_range,
                        parent: None,
                    })),
                })),
            })),
        }
    } else {
        // Fallback: entire document
        SelectionRange {
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: lines.len() as u32,
                    character: 0,
                },
            },
            parent: None,
        }
    }
}

/// Get the range of the word at the cursor position
fn get_word_range(line: &str, position: Position) -> Range {
    let chars: Vec<char> = line.chars().collect();
    let pos = position.character as usize;

    if pos >= chars.len() {
        return Range {
            start: position,
            end: position,
        };
    }

    // Find word boundaries
    let mut start = pos;
    let mut end = pos;

    // Expand left
    while start > 0 && is_word_char(chars[start - 1]) {
        start -= 1;
    }

    // Expand right
    while end < chars.len() && is_word_char(chars[end]) {
        end += 1;
    }

    Range {
        start: Position {
            line: position.line,
            character: start as u32,
        },
        end: Position {
            line: position.line,
            character: end as u32,
        },
    }
}

/// Get the range of the entire line (trimmed content)
fn get_line_range(line: &str, line_num: u32) -> Range {
    let trimmed = line.trim();
    let start_char = line.find(trimmed).unwrap_or(0) as u32;

    Range {
        start: Position {
            line: line_num,
            character: start_char,
        },
        end: Position {
            line: line_num,
            character: (start_char + trimmed.len() as u32),
        },
    }
}

/// Get the range of the current block (IF/WHILE/FUNCTION/etc.)
fn get_block_range(lines: &[&str], line_num: u32) -> Range {
    let mut start_line = line_num;
    let mut end_line = line_num;
    let mut depth = 0;

    // Find block start by going backwards
    for i in (0..=line_num as usize).rev() {
        let trimmed = lines[i].trim();
        let token_type = tokenize_line(trimmed);

        match token_type {
            TokenType::EndIf
            | TokenType::EndWhile
            | TokenType::EndFunction
            | TokenType::EndButtonDef
            | TokenType::EndExtension
            | TokenType::EndStage => {
                depth += 1;
            }
            TokenType::If
            | TokenType::While
            | TokenType::Function
            | TokenType::ButtonDef
            | TokenType::Extension
            | TokenType::Stage => {
                if depth == 0 {
                    start_line = i as u32;
                    break;
                }
                depth -= 1;
            }
            _ => {}
        }
    }

    // Find block end by going forwards
    depth = 0;
    for (i, line) in lines.iter().enumerate().skip(line_num as usize) {
        let trimmed = line.trim();
        let token_type = tokenize_line(trimmed);

        match token_type {
            TokenType::If
            | TokenType::While
            | TokenType::Function
            | TokenType::ButtonDef
            | TokenType::Extension
            | TokenType::Stage => {
                depth += 1;
            }
            TokenType::EndIf
            | TokenType::EndWhile
            | TokenType::EndFunction
            | TokenType::EndButtonDef
            | TokenType::EndExtension
            | TokenType::EndStage => {
                if depth == 0 {
                    end_line = i as u32;
                    break;
                }
                depth -= 1;
            }
            _ => {}
        }
    }

    Range {
        start: Position {
            line: start_line,
            character: 0,
        },
        end: Position {
            line: end_line + 1,
            character: 0,
        },
    }
}

/// Check if a character is part of a word
fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '$'
}
