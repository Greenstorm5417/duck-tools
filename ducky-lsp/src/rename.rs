use tower_lsp::lsp_types::*;
use crate::references::find_references;

/// Prepare rename - check if symbol can be renamed
pub fn prepare_rename(content: &str, position: Position) -> Option<Range> {
    let lines: Vec<&str> = content.lines().collect();
    
    if let Some(line) = lines.get(position.line as usize) {
        if let Some(symbol) = extract_symbol_at_position(line, position.character as usize) {
            // Can rename variables, functions, and constants
            if symbol.starts_with('$') || symbol.starts_with('#') || is_function_name(line, &symbol) {
                let start_col = line.find(&symbol)?;
                return Some(Range {
                    start: Position {
                        line: position.line,
                        character: start_col as u32,
                    },
                    end: Position {
                        line: position.line,
                        character: (start_col + symbol.len()) as u32,
                    },
                });
            }
        }
    }
    
    None
}

/// Perform rename operation
pub fn rename_symbol(content: &str, position: Position, new_name: &str, uri: &Url) -> Option<WorkspaceEdit> {
    // Find all references (including declaration)
    let locations = find_references(content, position, true, uri);
    
    if locations.is_empty() {
        return None;
    }
    
    let mut changes = std::collections::HashMap::new();
    
    let edits: Vec<TextEdit> = locations
        .into_iter()
        .map(|loc| TextEdit {
            range: loc.range,
            new_text: new_name.to_string(),
        })
        .collect();
    
    changes.insert(uri.clone(), edits);
    
    Some(WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    })
}

/// Extract symbol at position
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

/// Check if symbol is a function name
fn is_function_name(line: &str, symbol: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("FUNCTION") && trimmed.contains(symbol)
}
