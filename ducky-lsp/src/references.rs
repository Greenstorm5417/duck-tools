use tower_lsp::lsp_types::*;

/// Find all references to a symbol
pub fn find_references(
    content: &str,
    position: Position,
    include_declaration: bool,
    uri: &Url,
) -> Vec<Location> {
    let mut references = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    // Get the symbol at the cursor
    if let Some(line) = lines.get(position.line as usize)
        && let Some(symbol) = extract_symbol_at_position(line, position.character as usize)
    {
        // Find all occurrences of this symbol
        for (line_idx, line) in content.lines().enumerate() {
            let mut search_pos = 0;

            while let Some(pos) = line[search_pos..].find(&symbol) {
                let actual_pos = search_pos + pos;

                // Verify it's a complete word match, not part of another word
                let is_start = actual_pos == 0
                    || !line
                        .chars()
                        .nth(actual_pos - 1)
                        .unwrap_or(' ')
                        .is_alphanumeric();
                let is_end = actual_pos + symbol.len() >= line.len()
                    || !line
                        .chars()
                        .nth(actual_pos + symbol.len())
                        .unwrap_or(' ')
                        .is_alphanumeric();

                if is_start && is_end {
                    // Check if this is a declaration
                    let is_decl = is_declaration_line(line, &symbol);

                    if include_declaration || !is_decl {
                        references.push(Location {
                            uri: uri.clone(),
                            range: Range {
                                start: Position {
                                    line: line_idx as u32,
                                    character: actual_pos as u32,
                                },
                                end: Position {
                                    line: line_idx as u32,
                                    character: (actual_pos + symbol.len()) as u32,
                                },
                            },
                        });
                    }
                }

                search_pos = actual_pos + 1;
            }
        }
    }

    references
}

/// Extract symbol at cursor position
fn extract_symbol_at_position(line: &str, char_pos: usize) -> Option<String> {
    let chars: Vec<char> = line.chars().collect();

    if char_pos >= chars.len() {
        return None;
    }

    let mut start = char_pos;
    let mut end = char_pos;

    // Move start backward
    while start > 0 {
        let ch = chars[start - 1];
        if ch.is_alphanumeric() || ch == '_' || ch == '$' || ch == '#' {
            start -= 1;
        } else {
            break;
        }
    }

    // Move end forward
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

/// Check if a line is a declaration for the given symbol
fn is_declaration_line(line: &str, symbol: &str) -> bool {
    let trimmed = line.trim();

    // Variable declaration
    if trimmed.starts_with("VAR") && symbol.starts_with('$') {
        return true;
    }

    // Function definition
    if trimmed.starts_with("FUNCTION") && trimmed.contains(symbol) {
        return true;
    }

    // Define declaration
    if trimmed.starts_with("DEFINE") && symbol.starts_with('#') {
        return true;
    }

    false
}
