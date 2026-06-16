use ducky_core::lexer::{TokenType, tokenize_line};
use tower_lsp::lsp_types::*;

/// Find the definition of a symbol at the given position
pub fn find_definition(content: &str, position: Position, uri: &Url) -> Option<Location> {
    let lines: Vec<&str> = content.lines().collect();

    if let Some(line) = lines.get(position.line as usize) {
        // Get the word at cursor position
        let word = extract_word_at_position(line, position.character as usize)?;

        // If it's a variable reference, find its declaration
        if word.starts_with('$') {
            return find_variable_declaration(content, &word, uri);
        }

        // If it's a function call, find the function definition
        let token_type = tokenize_line(line);
        if token_type == TokenType::FunctionCall {
            return find_function_definition(content, &word, uri);
        }

        // If it's a preprocessor constant, find its definition
        if line.trim_start().starts_with("DEFINE") && word.starts_with('#') {
            return find_define_declaration(content, &word, uri);
        }
    }

    None
}

/// Extract word at given character position
fn extract_word_at_position(line: &str, char_pos: usize) -> Option<String> {
    let chars: Vec<char> = line.chars().collect();

    if char_pos >= chars.len() {
        return None;
    }

    // Find word boundaries
    let mut start = char_pos;
    let mut end = char_pos;

    // Move start backward to word beginning (including $ and #)
    while start > 0 {
        let ch = chars[start - 1];
        if ch.is_alphanumeric() || ch == '_' || ch == '$' || ch == '#' {
            start -= 1;
        } else {
            break;
        }
    }

    // Move end forward to word end
    while end < chars.len() {
        let ch = chars[end];
        if ch.is_alphanumeric() || ch == '_' || ch == '$' || ch == '#' {
            end += 1;
        } else {
            break;
        }
    }

    if start < end {
        Some(chars[start..end].iter().collect())
    } else {
        None
    }
}

/// Find where a variable is declared
fn find_variable_declaration(content: &str, var_name: &str, uri: &Url) -> Option<Location> {
    for (line_idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        // Check for VAR declaration
        if trimmed.starts_with("VAR") && trimmed.contains(var_name) {
            // Find the position of the variable name in the line
            if let Some(col) = line.find(var_name) {
                return Some(Location {
                    uri: uri.clone(),
                    range: Range {
                        start: Position {
                            line: line_idx as u32,
                            character: col as u32,
                        },
                        end: Position {
                            line: line_idx as u32,
                            character: (col + var_name.len()) as u32,
                        },
                    },
                });
            }
        }
    }

    None
}

/// Find where a function is defined
fn find_function_definition(content: &str, func_name: &str, uri: &Url) -> Option<Location> {
    for (line_idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        // Check for FUNCTION definition
        if trimmed.starts_with("FUNCTION") {
            // Extract function name
            if let Some(name_part) = trimmed.strip_prefix("FUNCTION") {
                let name = name_part.trim().trim_end_matches("()").trim();
                if name == func_name.trim_end_matches("()")
                    && let Some(col) = line.find(name)
                {
                    return Some(Location {
                        uri: uri.clone(),
                        range: Range {
                            start: Position {
                                line: line_idx as u32,
                                character: col as u32,
                            },
                            end: Position {
                                line: line_idx as u32,
                                character: (col + name.len()) as u32,
                            },
                        },
                    });
                }
            }
        }
    }

    None
}

/// Find where a preprocessor constant is defined
fn find_define_declaration(content: &str, const_name: &str, uri: &Url) -> Option<Location> {
    for (line_idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        // Check for DEFINE
        if trimmed.starts_with("DEFINE")
            && trimmed.contains(const_name)
            && let Some(col) = line.find(const_name)
        {
            return Some(Location {
                uri: uri.clone(),
                range: Range {
                    start: Position {
                        line: line_idx as u32,
                        character: col as u32,
                    },
                    end: Position {
                        line: line_idx as u32,
                        character: (col + const_name.len()) as u32,
                    },
                },
            });
        }
    }

    None
}
