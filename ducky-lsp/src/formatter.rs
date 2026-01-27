use ducky_core::lexer::{tokenize_line, TokenType};
use tower_lsp::lsp_types::*;

/// Format a DuckyScript document
pub fn format_document(content: &str, options: &FormattingOptions) -> Vec<TextEdit> {
    let lines: Vec<&str> = content.lines().collect();
    let mut edits = Vec::new();
    let mut indent_level: usize = 0;
    let indent_char = if options.insert_spaces {
        " ".repeat(options.tab_size as usize)
    } else {
        "\t".to_string()
    };
    
    for (line_idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        
        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }
        
        let token_type = tokenize_line(trimmed);
        
        // Determine if this decreases indent (before applying)
        let decreases_indent = matches!(
            token_type,
            TokenType::EndIf | TokenType::Else | TokenType::ElseIf |
            TokenType::EndWhile | TokenType::EndFunction | 
            TokenType::EndButtonDef | TokenType::EndExtension | TokenType::EndStage |
            TokenType::EndRemBlock | TokenType::EndString
        );
        
        if decreases_indent {
            indent_level = indent_level.saturating_sub(1);
        }
        
        // Calculate proper indentation
        let proper_indent = indent_char.repeat(indent_level);
        let formatted_line = format!("{}{}", proper_indent, trimmed);
        
        // Check if line needs reformatting
        if line != &formatted_line {
            edits.push(TextEdit {
                range: Range {
                    start: Position {
                        line: line_idx as u32,
                        character: 0,
                    },
                    end: Position {
                        line: line_idx as u32,
                        character: line.len() as u32,
                    },
                },
                new_text: formatted_line,
            });
        }
        
        // Determine if this increases indent (after applying)
        let increases_indent = matches!(
            token_type,
            TokenType::If | TokenType::ElseIf | TokenType::Else |
            TokenType::While | TokenType::Function | 
            TokenType::ButtonDef | TokenType::Extension | TokenType::Stage |
            TokenType::RemBlock | TokenType::StringBlock | TokenType::StringLnBlock
        );
        
        if increases_indent {
            indent_level += 1;
        }
    }
    
    edits
}
