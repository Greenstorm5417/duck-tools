use tower_lsp::lsp_types::*;
use ducky_core::lexer::{tokenize_line, TokenType};

/// Prepare call hierarchy item at the given position
pub fn prepare_call_hierarchy(content: &str, position: Position, uri: &Url) -> Option<Vec<CallHierarchyItem>> {
    let lines: Vec<&str> = content.lines().collect();
    
    if let Some(line) = lines.get(position.line as usize) {
        let trimmed = line.trim();
        
        // Check if we're on a FUNCTION declaration
        if trimmed.starts_with("FUNCTION ") {
            if let Some(func_name) = trimmed.strip_prefix("FUNCTION ").map(|s| s.trim().split_whitespace().next()).flatten() {
                let range = get_function_range(&lines, position.line);
                let selection_range = get_function_name_range(line, position.line, func_name);
                
                return Some(vec![CallHierarchyItem {
                    name: func_name.to_string(),
                    kind: SymbolKind::FUNCTION,
                    tags: None,
                    detail: Some("DuckyScript Function".to_string()),
                    uri: uri.clone(),
                    range,
                    selection_range,
                    data: None,
                }]);
            }
        }
        
        // Check if we're on a function call
        if let Some(func_name) = extract_function_call(trimmed) {
            // Find the function definition
            if let Some(def_line) = find_function_definition(&lines, func_name) {
                let range = get_function_range(&lines, def_line);
                let selection_range = get_function_name_range(lines[def_line as usize], def_line, func_name);
                
                return Some(vec![CallHierarchyItem {
                    name: func_name.to_string(),
                    kind: SymbolKind::FUNCTION,
                    tags: None,
                    detail: Some("DuckyScript Function".to_string()),
                    uri: uri.clone(),
                    range,
                    selection_range,
                    data: None,
                }]);
            }
        }
    }
    
    None
}

/// Get incoming calls to a function
pub fn get_incoming_calls(content: &str, item: &CallHierarchyItem, uri: &Url) -> Vec<CallHierarchyIncomingCall> {
    let lines: Vec<&str> = content.lines().collect();
    let func_name = &item.name;
    let mut incoming_calls = Vec::new();
    
    // Find all calls to this function
    for (line_idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        
        // Skip the function definition itself
        if trimmed.starts_with("FUNCTION ") {
            continue;
        }
        
        // Check if this line calls our function
        if let Some(called_func) = extract_function_call(trimmed) {
            if called_func == func_name {
                // Find which function this call is inside
                if let Some(caller_info) = find_containing_function(&lines, line_idx as u32) {
                    let (caller_name, caller_line) = caller_info;
                    let caller_range = get_function_range(&lines, caller_line);
                    let caller_selection_range = get_function_name_range(
                        lines[caller_line as usize],
                        caller_line,
                        &caller_name,
                    );
                    
                    let from_item = CallHierarchyItem {
                        name: caller_name,
                        kind: SymbolKind::FUNCTION,
                        tags: None,
                        detail: Some("DuckyScript Function".to_string()),
                        uri: uri.clone(),
                        range: caller_range,
                        selection_range: caller_selection_range,
                        data: None,
                    };
                    
                    let call_range = Range {
                        start: Position {
                            line: line_idx as u32,
                            character: 0,
                        },
                        end: Position {
                            line: line_idx as u32,
                            character: line.len() as u32,
                        },
                    };
                    
                    incoming_calls.push(CallHierarchyIncomingCall {
                        from: from_item,
                        from_ranges: vec![call_range],
                    });
                }
            }
        }
    }
    
    incoming_calls
}

/// Get outgoing calls from a function
pub fn get_outgoing_calls(content: &str, item: &CallHierarchyItem, uri: &Url) -> Vec<CallHierarchyOutgoingCall> {
    let lines: Vec<&str> = content.lines().collect();
    let func_name = &item.name;
    let mut outgoing_calls = Vec::new();
    
    // Find the function definition
    if let Some(func_line) = find_function_definition(&lines, func_name) {
        // Find the function body range
        let (body_start, body_end) = get_function_body_range(&lines, func_line);
        
        // Find all function calls within this function
        for line_idx in body_start..=body_end {
            if let Some(line) = lines.get(line_idx as usize) {
                let trimmed = line.trim();
                
                if let Some(called_func) = extract_function_call(trimmed) {
                    // Find the called function's definition
                    if let Some(called_def_line) = find_function_definition(&lines, called_func) {
                        let called_range = get_function_range(&lines, called_def_line);
                        let called_selection_range = get_function_name_range(
                            lines[called_def_line as usize],
                            called_def_line,
                            called_func,
                        );
                        
                        let to_item = CallHierarchyItem {
                            name: called_func.to_string(),
                            kind: SymbolKind::FUNCTION,
                            tags: None,
                            detail: Some("DuckyScript Function".to_string()),
                            uri: uri.clone(),
                            range: called_range,
                            selection_range: called_selection_range,
                            data: None,
                        };
                        
                        let call_range = Range {
                            start: Position {
                                line: line_idx,
                                character: 0,
                            },
                            end: Position {
                                line: line_idx,
                                character: line.len() as u32,
                            },
                        };
                        
                        outgoing_calls.push(CallHierarchyOutgoingCall {
                            to: to_item,
                            from_ranges: vec![call_range],
                        });
                    }
                }
            }
        }
    }
    
    outgoing_calls
}

/// Extract function name from a potential function call
fn extract_function_call(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    
    // Skip built-in commands
    let builtins = [
        "STRING", "STRINGLN", "DELAY", "IF", "WHILE", "VAR", "FUNCTION", "END_FUNCTION",
        "ENTER", "CTRL", "ALT", "GUI", "SHIFT", "REM", "DEFINE", "REPEAT", "HOLD",
        "RELEASE", "ATTACKMODE", "INJECT", "KEYCODE", "ELSE", "ELSE_IF", "END_IF",
        "END_WHILE", "RETURN", "INJECT_VAR", "EXFIL"
    ];
    
    for builtin in &builtins {
        if trimmed.starts_with(builtin) {
            return None;
        }
    }
    
    // If it's a word by itself or followed by space, might be a function call
    if let Some(space_pos) = trimmed.find(' ') {
        let word = &trimmed[..space_pos];
        if word.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Some(word);
        }
    } else if trimmed.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Some(trimmed);
    }
    
    None
}

/// Find the line number where a function is defined
fn find_function_definition(lines: &[&str], func_name: &str) -> Option<u32> {
    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if let Some(name) = trimmed.strip_prefix("FUNCTION ") {
            if name.trim().split_whitespace().next() == Some(func_name) {
                return Some(idx as u32);
            }
        }
    }
    None
}

/// Get the full range of a function (including END_FUNCTION)
fn get_function_range(lines: &[&str], start_line: u32) -> Range {
    let mut depth = 1;
    let mut end_line = start_line;
    
    for (idx, line) in lines.iter().enumerate().skip(start_line as usize + 1) {
        let trimmed = line.trim();
        let token_type = tokenize_line(trimmed);
        
        if token_type == TokenType::Function {
            depth += 1;
        } else if token_type == TokenType::EndFunction {
            depth -= 1;
            if depth == 0 {
                end_line = idx as u32;
                break;
            }
        }
    }
    
    Range {
        start: Position { line: start_line, character: 0 },
        end: Position { line: end_line + 1, character: 0 },
    }
}

/// Get the range of just the function name
fn get_function_name_range(line: &str, line_num: u32, func_name: &str) -> Range {
    let start_char = line.find(func_name).unwrap_or(0) as u32;
    
    Range {
        start: Position {
            line: line_num,
            character: start_char,
        },
        end: Position {
            line: line_num,
            character: start_char + func_name.len() as u32,
        },
    }
}

/// Get the body range of a function (excluding FUNCTION and END_FUNCTION lines)
fn get_function_body_range(lines: &[&str], start_line: u32) -> (u32, u32) {
    let mut depth = 1;
    let body_start = start_line + 1;
    let mut body_end = start_line + 1;
    
    for (idx, line) in lines.iter().enumerate().skip(start_line as usize + 1) {
        let trimmed = line.trim();
        let token_type = tokenize_line(trimmed);
        
        if token_type == TokenType::Function {
            depth += 1;
        } else if token_type == TokenType::EndFunction {
            depth -= 1;
            if depth == 0 {
                body_end = idx.saturating_sub(1) as u32;
                break;
            }
        }
    }
    
    (body_start, body_end)
}

/// Find which function contains a given line
fn find_containing_function(lines: &[&str], line_num: u32) -> Option<(String, u32)> {
    let mut current_function: Option<(String, u32)> = None;
    let mut depth = 0;
    
    for (idx, line) in lines.iter().enumerate() {
        if idx > line_num as usize {
            break;
        }
        
        let trimmed = line.trim();
        let token_type = tokenize_line(trimmed);
        
        if token_type == TokenType::Function {
            if let Some(func_name) = trimmed.strip_prefix("FUNCTION ").map(|s| s.trim().split_whitespace().next()).flatten() {
                if depth == 0 {
                    current_function = Some((func_name.to_string(), idx as u32));
                }
                depth += 1;
            }
        } else if token_type == TokenType::EndFunction {
            depth -= 1;
            if depth == 0 {
                current_function = None;
            }
        }
    }
    
    current_function
}
