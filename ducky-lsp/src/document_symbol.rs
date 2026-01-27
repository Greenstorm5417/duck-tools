use tower_lsp::lsp_types::*;
use ducky_core::lexer::{tokenize_line, TokenType};

/// Generate document symbols (outline view)
#[allow(deprecated)]
pub fn get_document_symbols(content: &str) -> Vec<DocumentSymbol> {
    let mut symbols = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();
        let token_type = tokenize_line(line);
        
        // Function definitions
        if token_type == TokenType::Function {
            if let Some(func_name) = extract_function_name(trimmed) {
                let start_line = i as u32;
                let end_line = find_function_end(&lines, i).unwrap_or(i) as u32;
                
                symbols.push(DocumentSymbol {
                    name: func_name.clone(),
                    detail: Some("Function".to_string()),
                    kind: SymbolKind::FUNCTION,
                    tags: None,
                    deprecated: None,
                    range: Range {
                        start: Position { line: start_line, character: 0 },
                        end: Position { line: end_line, character: lines.get(end_line as usize).map(|l| l.len()).unwrap_or(0) as u32 },
                    },
                    selection_range: Range {
                        start: Position { line: start_line, character: line.find(&func_name).unwrap_or(0) as u32 },
                        end: Position { line: start_line, character: (line.find(&func_name).unwrap_or(0) + func_name.len()) as u32 },
                    },
                    children: None,
                });
                
                i = end_line as usize;
            }
        }
        // Variable declarations
        else if token_type == TokenType::Declaration || token_type == TokenType::Assignment {
            if let Some(var_name) = extract_variable_name(trimmed) {
                symbols.push(DocumentSymbol {
                    name: var_name.clone(),
                    detail: Some("Variable".to_string()),
                    kind: SymbolKind::VARIABLE,
                    tags: None,
                    deprecated: None,
                    range: Range {
                        start: Position { line: i as u32, character: 0 },
                        end: Position { line: i as u32, character: line.len() as u32 },
                    },
                    selection_range: Range {
                        start: Position { line: i as u32, character: line.find(&var_name).unwrap_or(0) as u32 },
                        end: Position { line: i as u32, character: (line.find(&var_name).unwrap_or(0) + var_name.len()) as u32 },
                    },
                    children: None,
                });
            }
        }
        // Extension blocks
        else if token_type == TokenType::Extension {
            if let Some(ext_name) = extract_extension_name(trimmed) {
                let start_line = i as u32;
                let end_line = find_extension_end(&lines, i).unwrap_or(i) as u32;
                
                symbols.push(DocumentSymbol {
                    name: ext_name.clone(),
                    detail: Some("Extension".to_string()),
                    kind: SymbolKind::MODULE,
                    tags: None,
                    deprecated: None,
                    range: Range {
                        start: Position { line: start_line, character: 0 },
                        end: Position { line: end_line, character: lines.get(end_line as usize).map(|l| l.len()).unwrap_or(0) as u32 },
                    },
                    selection_range: Range {
                        start: Position { line: start_line, character: line.find(&ext_name).unwrap_or(0) as u32 },
                        end: Position { line: start_line, character: (line.find(&ext_name).unwrap_or(0) + ext_name.len()) as u32 },
                    },
                    children: None,
                });
                
                i = end_line as usize;
            }
        }
        // Preprocessor defines
        else if token_type == TokenType::Define {
            if let Some(const_name) = extract_define_name(trimmed) {
                symbols.push(DocumentSymbol {
                    name: const_name.clone(),
                    detail: Some("Constant".to_string()),
                    kind: SymbolKind::CONSTANT,
                    tags: None,
                    deprecated: None,
                    range: Range {
                        start: Position { line: i as u32, character: 0 },
                        end: Position { line: i as u32, character: line.len() as u32 },
                    },
                    selection_range: Range {
                        start: Position { line: i as u32, character: line.find(&const_name).unwrap_or(0) as u32 },
                        end: Position { line: i as u32, character: (line.find(&const_name).unwrap_or(0) + const_name.len()) as u32 },
                    },
                    children: None,
                });
            }
        }
        
        i += 1;
    }
    
    symbols
}

fn extract_function_name(line: &str) -> Option<String> {
    line.strip_prefix("FUNCTION")?
        .trim()
        .split_whitespace()
        .next()
        .map(|s| s.trim_end_matches("()").to_string())
}

fn extract_variable_name(line: &str) -> Option<String> {
    if let Some(rest) = line.strip_prefix("VAR") {
        rest.trim()
            .split_whitespace()
            .next()
            .map(|s| s.to_string())
    } else {
        // Assignment like $var = value
        line.split('=')
            .next()
            .and_then(|s| s.trim().split_whitespace().next())
            .map(|s| s.to_string())
    }
}

fn extract_extension_name(line: &str) -> Option<String> {
    line.strip_prefix("EXTENSION")?
        .trim()
        .split_whitespace()
        .next()
        .map(|s| s.to_string())
}

fn extract_define_name(line: &str) -> Option<String> {
    line.strip_prefix("DEFINE")?
        .trim()
        .split_whitespace()
        .next()
        .map(|s| s.to_string())
}

fn find_function_end(lines: &[&str], start: usize) -> Option<usize> {
    for i in (start + 1)..lines.len() {
        if tokenize_line(lines[i]) == TokenType::EndFunction {
            return Some(i);
        }
    }
    None
}

fn find_extension_end(lines: &[&str], start: usize) -> Option<usize> {
    for i in (start + 1)..lines.len() {
        if tokenize_line(lines[i]) == TokenType::EndExtension {
            return Some(i);
        }
    }
    None
}
