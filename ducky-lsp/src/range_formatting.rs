use ducky_core::lexer::{TokenType, tokenize_line};
use tower_lsp::lsp_types::*;

/// Format only a specific range of the document
pub fn format_range(content: &str, range: Range, options: &FormattingOptions) -> Vec<TextEdit> {
    let lines: Vec<&str> = content.lines().collect();
    let mut edits = Vec::new();

    let indent_char = if options.insert_spaces {
        " ".repeat(options.tab_size as usize)
    } else {
        "\t".to_string()
    };

    // Calculate the starting indent level by looking at context before the range
    let mut indent_level =
        calculate_indent_at_line(&lines, range.start.line as usize, &indent_char);

    // Format each line in the range
    for line_idx in range.start.line..=range.end.line.min((lines.len() as u32).saturating_sub(1)) {
        if let Some(line) = lines.get(line_idx as usize) {
            let trimmed = line.trim();

            // Skip empty lines
            if trimmed.is_empty() {
                continue;
            }

            let token_type = tokenize_line(trimmed);

            // Determine if this decreases indent (before applying)
            let decreases_indent = matches!(
                token_type,
                TokenType::EndIf
                    | TokenType::Else
                    | TokenType::ElseIf
                    | TokenType::EndWhile
                    | TokenType::EndFunction
                    | TokenType::EndButtonDef
                    | TokenType::EndExtension
                    | TokenType::EndStage
                    | TokenType::EndRemBlock
                    | TokenType::EndString
            );

            if decreases_indent {
                indent_level = indent_level.saturating_sub(1);
            }

            // Calculate proper indentation
            let proper_indent = indent_char.repeat(indent_level);
            let formatted_line = format!("{}{}", proper_indent, trimmed);

            // Check if line needs reformatting
            if line != &formatted_line {
                // Determine the actual range to replace
                let start_char = if line_idx == range.start.line {
                    range.start.character
                } else {
                    0
                };

                let end_char = if line_idx == range.end.line {
                    range.end.character.min(line.len() as u32)
                } else {
                    line.len() as u32
                };

                edits.push(TextEdit {
                    range: Range {
                        start: Position {
                            line: line_idx,
                            character: start_char,
                        },
                        end: Position {
                            line: line_idx,
                            character: end_char,
                        },
                    },
                    new_text: if start_char == 0 && end_char == line.len() as u32 {
                        formatted_line
                    } else {
                        // Partial line formatting - just fix the selected part
                        let selected_part = &line[start_char as usize..end_char as usize];
                        selected_part.trim().to_string()
                    },
                });
            }

            // Determine if this increases indent (after applying)
            let increases_indent = matches!(
                token_type,
                TokenType::If
                    | TokenType::ElseIf
                    | TokenType::Else
                    | TokenType::While
                    | TokenType::Function
                    | TokenType::ButtonDef
                    | TokenType::Extension
                    | TokenType::Stage
                    | TokenType::RemBlock
                    | TokenType::StringBlock
                    | TokenType::StringLnBlock
            );

            if increases_indent {
                indent_level += 1;
            }
        }
    }

    edits
}

/// Calculate the indent level at a specific line by analyzing context
fn calculate_indent_at_line(lines: &[&str], line_idx: usize, _indent_char: &str) -> usize {
    let mut indent_level: usize = 0;

    // Scan from beginning to the target line to calculate proper indent
    for i in 0..line_idx {
        if let Some(line) = lines.get(i) {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            let token_type = tokenize_line(trimmed);

            // Decreases indent
            if matches!(
                token_type,
                TokenType::EndIf
                    | TokenType::Else
                    | TokenType::ElseIf
                    | TokenType::EndWhile
                    | TokenType::EndFunction
                    | TokenType::EndButtonDef
                    | TokenType::EndExtension
                    | TokenType::EndStage
                    | TokenType::EndRemBlock
                    | TokenType::EndString
            ) {
                indent_level = indent_level.saturating_sub(1);
            }

            // Increases indent
            if matches!(
                token_type,
                TokenType::If
                    | TokenType::ElseIf
                    | TokenType::Else
                    | TokenType::While
                    | TokenType::Function
                    | TokenType::ButtonDef
                    | TokenType::Extension
                    | TokenType::Stage
                    | TokenType::RemBlock
                    | TokenType::StringBlock
                    | TokenType::StringLnBlock
            ) {
                indent_level += 1;
            }
        }
    }

    indent_level
}
