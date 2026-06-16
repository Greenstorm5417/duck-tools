use tower_lsp::lsp_types::*;

/// Get code actions for a given diagnostic or position
pub fn get_code_actions(
    content: &str,
    range: Range,
    diagnostics: Vec<Diagnostic>,
    uri: &Url,
) -> Vec<CodeAction> {
    let mut actions = Vec::new();

    // Quick fixes for diagnostics
    for diagnostic in &diagnostics {
        if ranges_overlap(diagnostic.range, range) {
            actions.extend(get_diagnostic_fixes(content, diagnostic, uri));
        }
    }

    // Refactoring actions (always available)
    actions.extend(get_refactoring_actions(content, range, uri));

    // Source actions
    actions.extend(get_source_actions(content, uri));

    actions
}

/// Get quick fixes for specific diagnostics
fn get_diagnostic_fixes(content: &str, diagnostic: &Diagnostic, uri: &Url) -> Vec<CodeAction> {
    let mut actions = Vec::new();

    // Check error code for specific fixes
    let error_code = diagnostic.code.as_ref().and_then(|c| match c {
        NumberOrString::String(s) => Some(s.as_str()),
        _ => None,
    });

    // Add suppression comment for warnings (W001-W999)
    if let Some(code) = error_code
        && code.starts_with('W')
    {
        // Get the indentation of the current line
        let lines: Vec<&str> = content.lines().collect();
        let line_idx = diagnostic.range.start.line as usize;
        let indent = if let Some(line) = lines.get(line_idx) {
            line.chars()
                .take_while(|c| c.is_whitespace())
                .collect::<String>()
        } else {
            String::new()
        };

        actions.push(CodeAction {
            title: format!("Suppress warning {}", code),
            kind: Some(CodeActionKind::QUICKFIX),
            diagnostics: Some(vec![diagnostic.clone()]),
            edit: Some(WorkspaceEdit {
                changes: Some(std::collections::HashMap::from([(
                    uri.clone(),
                    vec![TextEdit {
                        range: Range {
                            start: Position {
                                line: diagnostic.range.start.line,
                                character: 0,
                            },
                            end: Position {
                                line: diagnostic.range.start.line,
                                character: 0,
                            },
                        },
                        new_text: format!("{}REM ducky-ignore-next-line {}\n", indent, code),
                    }],
                )])),
                document_changes: None,
                change_annotations: None,
            }),
            ..Default::default()
        });
    }

    match error_code {
        // E100: Missing END_IF
        Some("E100") if diagnostic.message.contains("END_IF") => {
            actions.push(CodeAction {
                title: "Add missing END_IF".to_string(),
                kind: Some(CodeActionKind::QUICKFIX),
                diagnostics: Some(vec![diagnostic.clone()]),
                edit: Some(WorkspaceEdit {
                    changes: Some(std::collections::HashMap::from([(
                        uri.clone(),
                        vec![TextEdit {
                            range: Range {
                                start: Position {
                                    line: diagnostic.range.end.line + 1,
                                    character: 0,
                                },
                                end: Position {
                                    line: diagnostic.range.end.line + 1,
                                    character: 0,
                                },
                            },
                            new_text: "END_IF\n".to_string(),
                        }],
                    )])),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }
        // E100: Missing END_WHILE
        Some("E100") if diagnostic.message.contains("END_WHILE") => {
            actions.push(CodeAction {
                title: "Add missing END_WHILE".to_string(),
                kind: Some(CodeActionKind::QUICKFIX),
                diagnostics: Some(vec![diagnostic.clone()]),
                edit: Some(WorkspaceEdit {
                    changes: Some(std::collections::HashMap::from([(
                        uri.clone(),
                        vec![TextEdit {
                            range: Range {
                                start: Position {
                                    line: diagnostic.range.end.line + 1,
                                    character: 0,
                                },
                                end: Position {
                                    line: diagnostic.range.end.line + 1,
                                    character: 0,
                                },
                            },
                            new_text: "END_WHILE\n".to_string(),
                        }],
                    )])),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }
        // E100: Missing END_FUNCTION
        Some("E100") if diagnostic.message.contains("END_FUNCTION") => {
            actions.push(CodeAction {
                title: "Add missing END_FUNCTION".to_string(),
                kind: Some(CodeActionKind::QUICKFIX),
                diagnostics: Some(vec![diagnostic.clone()]),
                edit: Some(WorkspaceEdit {
                    changes: Some(std::collections::HashMap::from([(
                        uri.clone(),
                        vec![TextEdit {
                            range: Range {
                                start: Position {
                                    line: diagnostic.range.end.line + 1,
                                    character: 0,
                                },
                                end: Position {
                                    line: diagnostic.range.end.line + 1,
                                    character: 0,
                                },
                            },
                            new_text: "END_FUNCTION\n".to_string(),
                        }],
                    )])),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }
        _ => {}
    }

    actions
}

/// Get refactoring actions
fn get_refactoring_actions(content: &str, range: Range, uri: &Url) -> Vec<CodeAction> {
    let mut actions = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    // Extract to function (if multiple lines selected)
    if range.start.line < range.end.line {
        let selected_lines: Vec<&str> = lines
            .iter()
            .skip(range.start.line as usize)
            .take((range.end.line - range.start.line + 1) as usize)
            .copied()
            .collect();

        if !selected_lines.is_empty() {
            actions.push(CodeAction {
                title: "Extract to FUNCTION".to_string(),
                kind: Some(CodeActionKind::REFACTOR_EXTRACT),
                edit: Some(create_extract_function_edit(range, &selected_lines, uri)),
                ..Default::default()
            });
        }
    }

    // Inline variable (if on a VAR line)
    if let Some(line) = lines.get(range.start.line as usize) {
        let trimmed = line.trim();
        if trimmed.starts_with("VAR ") {
            actions.push(CodeAction {
                title: "Inline variable".to_string(),
                kind: Some(CodeActionKind::REFACTOR_INLINE),
                edit: None, // Would need more complex logic
                ..Default::default()
            });
        }
    }

    actions
}

/// Get source actions
fn get_source_actions(content: &str, uri: &Url) -> Vec<CodeAction> {
    let mut actions = Vec::new();

    // Remove unused variables
    let unused_vars = find_unused_variables(content);
    if !unused_vars.is_empty() {
        actions.push(CodeAction {
            title: format!("Remove {} unused variable(s)", unused_vars.len()),
            kind: Some(CodeActionKind::SOURCE_ORGANIZE_IMPORTS),
            edit: Some(create_remove_unused_vars_edit(&unused_vars, uri)),
            ..Default::default()
        });
    }

    // Sort DEFINE statements
    if content.contains("DEFINE ") {
        actions.push(CodeAction {
            title: "Sort DEFINE statements".to_string(),
            kind: Some(CodeActionKind::SOURCE_ORGANIZE_IMPORTS),
            edit: None, // Would need implementation
            ..Default::default()
        });
    }

    actions
}

/// Create edit for extracting code to a function
fn create_extract_function_edit(range: Range, selected_lines: &[&str], uri: &Url) -> WorkspaceEdit {
    let function_body = selected_lines.join("\n");
    let function_name = "extracted_function";

    let function_def = format!(
        "FUNCTION {}\n{}\nEND_FUNCTION\n\n",
        function_name, function_body
    );

    WorkspaceEdit {
        changes: Some(std::collections::HashMap::from([(
            uri.clone(),
            vec![
                // Insert function definition at start of file
                TextEdit {
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
                    new_text: function_def,
                },
                // Replace selected code with function call
                TextEdit {
                    range,
                    new_text: function_name.to_string(),
                },
            ],
        )])),
        ..Default::default()
    }
}

/// Find unused variables in the document
fn find_unused_variables(content: &str) -> Vec<(u32, String)> {
    let mut unused = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    for (line_idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("VAR ") {
            // Extract variable name
            if let Some(var_name) = trimmed
                .strip_prefix("VAR ")
                .and_then(|s| s.split_whitespace().next())
            {
                // Check if variable is used anywhere else
                let var_ref = format!("${}", var_name);
                let mut used = false;

                for (check_idx, check_line) in lines.iter().enumerate() {
                    if check_idx != line_idx && check_line.contains(&var_ref) {
                        used = true;
                        break;
                    }
                }

                if !used {
                    unused.push((line_idx as u32, var_name.to_string()));
                }
            }
        }
    }

    unused
}

/// Create edit to remove unused variables
fn create_remove_unused_vars_edit(unused_vars: &[(u32, String)], uri: &Url) -> WorkspaceEdit {
    let edits: Vec<TextEdit> = unused_vars
        .iter()
        .map(|(line, _)| TextEdit {
            range: Range {
                start: Position {
                    line: *line,
                    character: 0,
                },
                end: Position {
                    line: *line + 1,
                    character: 0,
                },
            },
            new_text: String::new(),
        })
        .collect();

    WorkspaceEdit {
        changes: Some(std::collections::HashMap::from([(uri.clone(), edits)])),
        ..Default::default()
    }
}

/// Check if two ranges overlap
fn ranges_overlap(r1: Range, r2: Range) -> bool {
    !(r1.end.line < r2.start.line || r2.end.line < r1.start.line)
}
