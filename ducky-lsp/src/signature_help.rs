use tower_lsp::lsp_types::*;

/// Provide signature help for function calls
pub fn get_signature_help(content: &str, position: Position) -> Option<SignatureHelp> {
    let lines: Vec<&str> = content.lines().collect();
    
    if let Some(line) = lines.get(position.line as usize) {
        // Check if we're inside a function call
        if let Some(func_name) = extract_function_call_at_position(line, position.character as usize) {
            // Find the function definition
            if let Some(signature) = find_function_signature(content, &func_name) {
                return Some(SignatureHelp {
                    signatures: vec![signature],
                    active_signature: Some(0),
                    active_parameter: Some(0),
                });
            }
        }
        
        // Built-in commands with parameters
        let trimmed = line[..position.character.min(line.len() as u32) as usize].trim();
        
        if trimmed.starts_with("DELAY") {
            return Some(SignatureHelp {
                signatures: vec![SignatureInformation {
                    label: "DELAY milliseconds".to_string(),
                    documentation: Some(Documentation::String("Wait for specified milliseconds (1-60000)".to_string())),
                    parameters: Some(vec![ParameterInformation {
                        label: ParameterLabel::Simple("milliseconds".to_string()),
                        documentation: Some(Documentation::String("Time to wait in milliseconds".to_string())),
                    }]),
                    active_parameter: None,
                }],
                active_signature: Some(0),
                active_parameter: Some(0),
            });
        }
        
        if trimmed.starts_with("STRING") {
            return Some(SignatureHelp {
                signatures: vec![SignatureInformation {
                    label: "STRING text".to_string(),
                    documentation: Some(Documentation::String("Type a string of characters".to_string())),
                    parameters: Some(vec![ParameterInformation {
                        label: ParameterLabel::Simple("text".to_string()),
                        documentation: Some(Documentation::String("Text to type".to_string())),
                    }]),
                    active_parameter: None,
                }],
                active_signature: Some(0),
                active_parameter: Some(0),
            });
        }
        
        if trimmed.starts_with("STRINGLN") {
            return Some(SignatureHelp {
                signatures: vec![SignatureInformation {
                    label: "STRINGLN text".to_string(),
                    documentation: Some(Documentation::String("Type a string followed by ENTER".to_string())),
                    parameters: Some(vec![ParameterInformation {
                        label: ParameterLabel::Simple("text".to_string()),
                        documentation: Some(Documentation::String("Text to type".to_string())),
                    }]),
                    active_parameter: None,
                }],
                active_signature: Some(0),
                active_parameter: Some(0),
            });
        }
    }
    
    None
}

fn extract_function_call_at_position(line: &str, char_pos: usize) -> Option<String> {
    // Find if we're inside a function call with parentheses
    let before = &line[..char_pos.min(line.len())];
    
    // Look for last word before (
    if let Some(paren_pos) = before.rfind('(') {
        let func_part = &before[..paren_pos];
        let func_name = func_part.split_whitespace().last()?;
        return Some(func_name.to_string());
    }
    
    None
}

fn find_function_signature(content: &str, func_name: &str) -> Option<SignatureInformation> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("FUNCTION") {
            if let Some(sig) = trimmed.strip_prefix("FUNCTION") {
                let sig = sig.trim();
                if sig.starts_with(func_name) {
                    return Some(SignatureInformation {
                        label: sig.to_string(),
                        documentation: Some(Documentation::String(format!("User-defined function: {}", func_name))),
                        parameters: None, // TODO: Parse parameters
                        active_parameter: None,
                    });
                }
            }
        }
    }
    
    None
}
