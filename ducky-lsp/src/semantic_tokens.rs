use ducky_core::lexer::{TokenType, tokenize_line};
use tower_lsp::lsp_types::*;

/// Map TokenType to LSP semantic token type and modifiers
pub fn token_type_to_semantic(
    token: &TokenType,
) -> (SemanticTokenType, Vec<SemanticTokenModifier>) {
    match token {
        // Keywords - Control flow
        TokenType::If
        | TokenType::ElseIf
        | TokenType::Else
        | TokenType::EndIf
        | TokenType::While
        | TokenType::EndWhile => (SemanticTokenType::KEYWORD, vec![]),

        // Keywords - Functions
        TokenType::Function
        | TokenType::EndFunction
        | TokenType::Return
        | TokenType::FunctionCall => (SemanticTokenType::FUNCTION, vec![]),

        // Keywords - Blocks
        TokenType::ButtonDef
        | TokenType::EndButtonDef
        | TokenType::Extension
        | TokenType::EndExtension
        | TokenType::Stage
        | TokenType::EndStage => (SemanticTokenType::KEYWORD, vec![]),

        // Preprocessor
        TokenType::Define
        | TokenType::IfDefined
        | TokenType::IfNotDefined
        | TokenType::ElseDefined
        | TokenType::EndIfDefined
        | TokenType::PreprocessorDisabled => (SemanticTokenType::MACRO, vec![]),

        // Comments
        TokenType::Rem | TokenType::RemBlock | TokenType::EndRemBlock | TokenType::Comment => {
            (SemanticTokenType::COMMENT, vec![])
        }

        // String commands
        TokenType::String
        | TokenType::StringLn
        | TokenType::StringBlock
        | TokenType::StringLnBlock
        | TokenType::EndString => (
            SemanticTokenType::KEYWORD,
            vec![SemanticTokenModifier::DECLARATION],
        ),

        // Timing commands
        TokenType::Delay
        | TokenType::DelayVar
        | TokenType::DefaultDelay
        | TokenType::StringDelay => (SemanticTokenType::KEYWORD, vec![]),

        // Variables
        TokenType::Assignment | TokenType::Declaration => (
            SemanticTokenType::VARIABLE,
            vec![SemanticTokenModifier::DECLARATION],
        ),

        // System commands
        TokenType::Attackmode
        | TokenType::InjectVar
        | TokenType::Inject
        | TokenType::InjectMod
        | TokenType::Keycode
        | TokenType::ExfilVar => (
            SemanticTokenType::KEYWORD,
            vec![SemanticTokenModifier::MODIFICATION],
        ),

        // Key commands
        TokenType::Hold
        | TokenType::Release
        | TokenType::Repeat
        | TokenType::Enter
        | TokenType::ModKeyDown
        | TokenType::ModKeyUp
        | TokenType::KeyDown
        | TokenType::KeyUp
        | TokenType::ModDown
        | TokenType::ModUp => (SemanticTokenType::KEYWORD, vec![]),

        // Modifier keys
        TokenType::Modifier => (SemanticTokenType::new("modifier"), vec![]),

        // Random generators
        TokenType::RandomLowercaseLetter
        | TokenType::RandomUppercaseLetter
        | TokenType::RandomNumber
        | TokenType::RandomLetter
        | TokenType::RandomSpecial
        | TokenType::RandomChar => (
            SemanticTokenType::FUNCTION,
            vec![SemanticTokenModifier::STATIC],
        ),

        // Debug
        TokenType::Breakpoint | TokenType::InjectBreakpointLineNumber => (
            SemanticTokenType::KEYWORD,
            vec![SemanticTokenModifier::MODIFICATION],
        ),

        // Unknown
        TokenType::Unknown => (SemanticTokenType::new("unknown"), vec![]),
    }
}

/// Generate semantic tokens for entire document
pub fn generate_semantic_tokens(content: &str) -> Vec<SemanticToken> {
    let mut tokens = Vec::new();
    let mut prev_line = 0;
    let mut prev_start = 0;
    let mut in_rem_block = false;

    for (line_idx, line) in content.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        // Tokenize the line to get the command type
        let token_type = tokenize_line(line);

        // Track REM_BLOCK state
        if token_type == TokenType::RemBlock {
            in_rem_block = true;
        } else if token_type == TokenType::EndRemBlock {
            in_rem_block = false;
        }

        // If we're inside a REM_BLOCK, highlight entire line as comment
        let (semantic_type, modifiers) = if in_rem_block
            && token_type != TokenType::RemBlock
            && token_type != TokenType::EndRemBlock
        {
            (SemanticTokenType::COMMENT, vec![])
        } else {
            token_type_to_semantic(&token_type)
        };

        // Find the first non-whitespace character
        let start_char = line.chars().take_while(|c| c.is_whitespace()).count();

        // Highlight the command keyword (first word)
        let token_text = line.trim();

        // For comments (REM, REM_BLOCK, //, END_REM) or lines inside REM_BLOCK, highlight the entire line
        let keyword_length = if in_rem_block
            || matches!(
                token_type,
                TokenType::Rem | TokenType::RemBlock | TokenType::EndRemBlock | TokenType::Comment
            ) {
            token_text.len()
        } else if let Some(space_idx) = token_text.find(char::is_whitespace) {
            space_idx
        } else {
            token_text.len()
        };

        // Calculate delta encoding for keyword
        let delta_line = line_idx as u32 - prev_line;
        let delta_start = if delta_line == 0 {
            start_char as u32 - prev_start
        } else {
            start_char as u32
        };

        let token_type_idx = semantic_type_to_index(&semantic_type);
        let token_modifiers_bits = modifiers_to_bits(&modifiers);

        // Add keyword token
        tokens.push(SemanticToken {
            delta_line,
            delta_start,
            length: keyword_length as u32,
            token_type: token_type_idx,
            token_modifiers_bitset: token_modifiers_bits,
        });

        prev_line = line_idx as u32;
        prev_start = start_char as u32;

        // Parse rest of line for variables, operators, numbers, constants, booleans
        // Skip if we're in a comment block
        if !in_rem_block
            && !matches!(
                token_type,
                TokenType::Rem | TokenType::RemBlock | TokenType::EndRemBlock | TokenType::Comment
            )
        {
            let rest_of_line = &token_text[keyword_length..];
            let char_offset = start_char + keyword_length;

            for (i, ch) in rest_of_line.chars().enumerate() {
                let current_pos = char_offset + i;

                // Preprocessor constant detection (#CONSTANT)
                if ch == '#' {
                    let const_name: String = rest_of_line
                        .chars()
                        .skip(i)
                        .take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '#')
                        .collect();

                    if !const_name.is_empty() {
                        let delta_line_const = 0;
                        let delta_start_const = (current_pos - prev_start as usize) as u32;

                        tokens.push(SemanticToken {
                            delta_line: delta_line_const,
                            delta_start: delta_start_const,
                            length: const_name.len() as u32,
                            token_type: semantic_type_to_index(&SemanticTokenType::PROPERTY),
                            token_modifiers_bitset: 0,
                        });

                        prev_start = current_pos as u32;
                    }
                }
                // Variable detection ($var)
                else if ch == '$' {
                    let var_name: String = rest_of_line
                        .chars()
                        .skip(i)
                        .take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '$')
                        .collect();

                    if !var_name.is_empty() {
                        let delta_line_var = 0; // Same line
                        let delta_start_var = (current_pos - prev_start as usize) as u32;

                        tokens.push(SemanticToken {
                            delta_line: delta_line_var,
                            delta_start: delta_start_var,
                            length: var_name.len() as u32,
                            token_type: semantic_type_to_index(&SemanticTokenType::VARIABLE),
                            token_modifiers_bitset: 0,
                        });

                        prev_start = current_pos as u32;
                    }
                }
                // Operator detection
                else if matches!(
                    ch,
                    '=' | '+' | '-' | '*' | '/' | '%' | '<' | '>' | '&' | '|' | '!' | '^'
                ) {
                    let delta_line_op = 0;
                    let delta_start_op = (current_pos - prev_start as usize) as u32;

                    tokens.push(SemanticToken {
                        delta_line: delta_line_op,
                        delta_start: delta_start_op,
                        length: 1,
                        token_type: semantic_type_to_index(&SemanticTokenType::OPERATOR),
                        token_modifiers_bitset: 0,
                    });

                    prev_start = current_pos as u32;
                }
                // Number detection
                else if ch.is_ascii_digit()
                    && (i == 0
                        || !rest_of_line
                            .chars()
                            .nth(i - 1)
                            .unwrap_or(' ')
                            .is_alphanumeric())
                {
                    let number: String = rest_of_line
                        .chars()
                        .skip(i)
                        .take_while(|c| {
                            c.is_ascii_digit()
                                || *c == 'x'
                                || (*c >= 'a' && *c <= 'f')
                                || (*c >= 'A' && *c <= 'F')
                        })
                        .collect();

                    if !number.is_empty() {
                        let delta_line_num = 0;
                        let delta_start_num = (current_pos - prev_start as usize) as u32;

                        tokens.push(SemanticToken {
                            delta_line: delta_line_num,
                            delta_start: delta_start_num,
                            length: number.len() as u32,
                            token_type: semantic_type_to_index(&SemanticTokenType::NUMBER),
                            token_modifiers_bitset: 0,
                        });

                        prev_start = current_pos as u32;
                    }
                }
                // Boolean detection (TRUE/FALSE)
                else if ch == 'T' || ch == 'F' {
                    let word: String = rest_of_line
                        .chars()
                        .skip(i)
                        .take_while(|c| c.is_alphabetic())
                        .collect();

                    if word == "TRUE" || word == "FALSE" {
                        // Check it's a complete word
                        let is_word_boundary = i == 0
                            || !rest_of_line
                                .chars()
                                .nth(i - 1)
                                .unwrap_or(' ')
                                .is_alphanumeric();
                        let next_pos = i + word.len();
                        let is_end_boundary = next_pos >= rest_of_line.len()
                            || !rest_of_line
                                .chars()
                                .nth(next_pos)
                                .unwrap_or(' ')
                                .is_alphanumeric();

                        if is_word_boundary && is_end_boundary {
                            let delta_line_bool = 0;
                            let delta_start_bool = (current_pos - prev_start as usize) as u32;

                            tokens.push(SemanticToken {
                                delta_line: delta_line_bool,
                                delta_start: delta_start_bool,
                                length: word.len() as u32,
                                token_type: semantic_type_to_index(&SemanticTokenType::new(
                                    "boolean",
                                )),
                                token_modifiers_bitset: 0,
                            });

                            prev_start = current_pos as u32;
                        }
                    }
                }
            }
        }
    }

    tokens
}

/// Map semantic token type to index in legend
fn semantic_type_to_index(token_type: &SemanticTokenType) -> u32 {
    let types = semantic_token_types_legend();
    types.iter().position(|t| t == token_type).unwrap_or(0) as u32
}

/// Convert modifiers to bitset
fn modifiers_to_bits(modifiers: &[SemanticTokenModifier]) -> u32 {
    let legend = semantic_token_modifiers_legend();
    modifiers.iter().fold(0u32, |acc, modifier| {
        if let Some(idx) = legend.iter().position(|m| m == modifier) {
            acc | (1 << idx)
        } else {
            acc
        }
    })
}

/// Define the semantic token types legend
pub fn semantic_token_types_legend() -> Vec<SemanticTokenType> {
    vec![
        SemanticTokenType::KEYWORD,
        SemanticTokenType::FUNCTION,
        SemanticTokenType::VARIABLE,
        SemanticTokenType::STRING,
        SemanticTokenType::NUMBER,
        SemanticTokenType::COMMENT,
        SemanticTokenType::MACRO,
        SemanticTokenType::OPERATOR,
        SemanticTokenType::PROPERTY,
        SemanticTokenType::new("boolean"),
        SemanticTokenType::new("modifier"),
        SemanticTokenType::new("unknown"),
    ]
}

/// Define the semantic token modifiers legend
pub fn semantic_token_modifiers_legend() -> Vec<SemanticTokenModifier> {
    vec![
        SemanticTokenModifier::DECLARATION,
        SemanticTokenModifier::DEFINITION,
        SemanticTokenModifier::READONLY,
        SemanticTokenModifier::STATIC,
        SemanticTokenModifier::MODIFICATION,
    ]
}
