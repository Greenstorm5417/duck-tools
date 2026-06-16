use tower_lsp::lsp_types::*;

/// Highlight all occurrences of symbol under cursor
pub fn get_document_highlights(content: &str, position: Position) -> Vec<DocumentHighlight> {
    let mut highlights = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    if let Some(line) = lines.get(position.line as usize)
        && let Some(symbol) = extract_symbol_at_position(line, position.character as usize)
    {
        // Find all occurrences
        for (line_idx, line) in content.lines().enumerate() {
            let mut search_pos = 0;

            while let Some(pos) = line[search_pos..].find(&symbol) {
                let actual_pos = search_pos + pos;

                // Verify complete word match
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
                    let kind = if is_write_occurrence(line, &symbol, actual_pos) {
                        DocumentHighlightKind::WRITE
                    } else {
                        DocumentHighlightKind::READ
                    };

                    highlights.push(DocumentHighlight {
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
                        kind: Some(kind),
                    });
                }

                search_pos = actual_pos + 1;
            }
        }
    }

    highlights
}

fn extract_symbol_at_position(line: &str, char_pos: usize) -> Option<String> {
    let chars: Vec<char> = line.chars().collect();

    if char_pos >= chars.len() {
        return None;
    }

    let mut start = char_pos;
    let mut end = char_pos;

    while start > 0 {
        let ch = chars[start - 1];
        if ch.is_alphanumeric() || ch == '_' || ch == '$' || ch == '#' {
            start -= 1;
        } else {
            break;
        }
    }

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

fn is_write_occurrence(line: &str, _symbol: &str, _pos: usize) -> bool {
    let trimmed = line.trim();

    // Variable assignments and declarations are writes
    if trimmed.starts_with("VAR") {
        return true;
    }

    // Check if there's an assignment operator after the symbol
    if line.contains('=') {
        return true;
    }

    false
}
