use ducky_core::lexer::{TokenType, tokenize_line};
use tower_lsp::lsp_types::*;

/// Get formatting edits when typing a trigger character
pub fn get_on_type_formatting(
    content: &str,
    position: Position,
    ch: &str,
    options: &FormattingOptions,
) -> Vec<TextEdit> {
    let mut edits = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    // Only handle newline character
    if ch != "\n" {
        return edits;
    }

    // Get the previous line (the one that was just completed)
    if position.line == 0 {
        return edits;
    }

    let prev_line_idx = (position.line - 1) as usize;
    if let Some(prev_line) = lines.get(prev_line_idx) {
        let trimmed = prev_line.trim();
        let token_type = tokenize_line(trimmed);

        let indent_char = if options.insert_spaces {
            " ".repeat(options.tab_size as usize)
        } else {
            "\t".to_string()
        };

        // Calculate current indent level
        let current_indent = get_indent_level(prev_line, &indent_char);

        // Auto-complete block structures
        match token_type {
            TokenType::If | TokenType::ElseIf | TokenType::Else => {
                // Add END_IF if not present
                if !has_matching_end(&lines, prev_line_idx, TokenType::If, TokenType::EndIf) {
                    let new_indent = indent_char.repeat(current_indent);
                    let inner_indent = indent_char.repeat(current_indent + 1);

                    edits.push(TextEdit {
                        range: Range {
                            start: position,
                            end: position,
                        },
                        new_text: format!("{}\n{}END_IF", inner_indent, new_indent),
                    });
                } else {
                    // Just add proper indentation for next line
                    let inner_indent = indent_char.repeat(current_indent + 1);
                    edits.push(TextEdit {
                        range: Range {
                            start: position,
                            end: position,
                        },
                        new_text: inner_indent,
                    });
                }
            }
            TokenType::While => {
                // Add END_WHILE if not present
                if !has_matching_end(&lines, prev_line_idx, TokenType::While, TokenType::EndWhile) {
                    let new_indent = indent_char.repeat(current_indent);
                    let inner_indent = indent_char.repeat(current_indent + 1);

                    edits.push(TextEdit {
                        range: Range {
                            start: position,
                            end: position,
                        },
                        new_text: format!("{}\n{}END_WHILE", inner_indent, new_indent),
                    });
                } else {
                    let inner_indent = indent_char.repeat(current_indent + 1);
                    edits.push(TextEdit {
                        range: Range {
                            start: position,
                            end: position,
                        },
                        new_text: inner_indent,
                    });
                }
            }
            TokenType::Function => {
                // Add END_FUNCTION if not present
                if !has_matching_end(
                    &lines,
                    prev_line_idx,
                    TokenType::Function,
                    TokenType::EndFunction,
                ) {
                    let new_indent = indent_char.repeat(current_indent);
                    let inner_indent = indent_char.repeat(current_indent + 1);

                    edits.push(TextEdit {
                        range: Range {
                            start: position,
                            end: position,
                        },
                        new_text: format!("{}\n{}END_FUNCTION", inner_indent, new_indent),
                    });
                } else {
                    let inner_indent = indent_char.repeat(current_indent + 1);
                    edits.push(TextEdit {
                        range: Range {
                            start: position,
                            end: position,
                        },
                        new_text: inner_indent,
                    });
                }
            }
            TokenType::StringBlock | TokenType::StringLnBlock => {
                // Add END_STRING if not present
                let new_indent = indent_char.repeat(current_indent);
                let inner_indent = indent_char.repeat(current_indent + 1);

                edits.push(TextEdit {
                    range: Range {
                        start: position,
                        end: position,
                    },
                    new_text: format!("{}\n{}END_STRING", inner_indent, new_indent),
                });
            }
            TokenType::RemBlock => {
                // Add END_REM if not present
                let new_indent = indent_char.repeat(current_indent);
                let inner_indent = indent_char.repeat(current_indent + 1);

                edits.push(TextEdit {
                    range: Range {
                        start: position,
                        end: position,
                    },
                    new_text: format!("{}\n{}END_REM", inner_indent, new_indent),
                });
            }
            _ => {
                // Default: maintain current indentation
                let new_indent = indent_char.repeat(current_indent);
                if !new_indent.is_empty() {
                    edits.push(TextEdit {
                        range: Range {
                            start: position,
                            end: position,
                        },
                        new_text: new_indent,
                    });
                }
            }
        }
    }

    edits
}

/// Get the indentation level of a line
fn get_indent_level(line: &str, indent_char: &str) -> usize {
    let mut level = 0;
    let mut chars = line.chars();

    while let Some(ch) = chars.next() {
        if ch == '\t' {
            level += 1;
        } else if ch == ' ' {
            // Count spaces based on indent_char length
            let spaces_per_indent = indent_char.len();
            let mut space_count = 1;

            while let Some(' ') = chars.next() {
                space_count += 1;
            }

            level += space_count / spaces_per_indent;
            break;
        } else {
            break;
        }
    }

    level
}

/// Check if a block has a matching end statement
fn has_matching_end(
    lines: &[&str],
    start_idx: usize,
    start_token: TokenType,
    end_token: TokenType,
) -> bool {
    let mut depth = 1;

    for line in lines.iter().skip(start_idx + 1) {
        let trimmed = line.trim();
        let token_type = tokenize_line(trimmed);

        if token_type == start_token {
            depth += 1;
        } else if token_type == end_token {
            depth -= 1;
            if depth == 0 {
                return true;
            }
        }
    }

    false
}
