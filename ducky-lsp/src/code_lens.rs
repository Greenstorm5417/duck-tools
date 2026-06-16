use tower_lsp::lsp_types::*;

/// Generate code lenses (inline action buttons)
pub fn get_code_lenses(content: &str, uri: &Url) -> Vec<CodeLens> {
    let mut lenses = Vec::new();

    // Add "▶ Compile" button at the top of the file
    if !content.trim().is_empty() {
        lenses.push(CodeLens {
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 0,
                },
            },
            command: Some(Command {
                title: "▶ Compile DuckyScript".to_string(),
                command: "duckyscript.compile".to_string(),
                arguments: Some(vec![serde_json::json!(uri.to_string())]),
            }),
            data: None,
        });
    }

    // Add "▶ Run Function" for each function definition
    for (line_idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("FUNCTION")
            && let Some(func_name) = trimmed.strip_prefix("FUNCTION")
        {
            let func_name = func_name
                .split_whitespace()
                .next()
                .unwrap_or("")
                .trim_end_matches("()");
            if !func_name.is_empty() {
                lenses.push(CodeLens {
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
                    command: Some(Command {
                        title: format!("▶ Call {}()", func_name),
                        command: "duckyscript.runFunction".to_string(),
                        arguments: Some(vec![
                            serde_json::json!(uri.to_string()),
                            serde_json::json!(func_name),
                        ]),
                    }),
                    data: None,
                });
            }
        }
    }

    lenses
}
