use ducky_core::DuckyCompiler;
use tower_lsp::lsp_types::*;

/// Convert compiler errors and warnings to LSP Diagnostics
pub fn compiler_to_diagnostics(compiler: &DuckyCompiler, _uri: &Url) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Convert errors
    for error in &compiler.errors {
        let line = compiler.current_line_index as u32;
        let diagnostic = Diagnostic {
            range: Range {
                start: Position {
                    line,
                    character: 0,
                },
                end: Position {
                    line,
                    character: u32::MAX, // End of line
                },
            },
            severity: Some(DiagnosticSeverity::ERROR),
            code: None,
            code_description: None,
            source: Some("ducky-compiler".to_string()),
            message: format!("{}", error),
            related_information: None,
            tags: None,
            data: None,
        };
        diagnostics.push(diagnostic);
    }

    // Convert warnings
    for warning in &compiler.warnings {
        let line = warning.line.saturating_sub(1) as u32; // Convert to 0-indexed
        let diagnostic = Diagnostic {
            range: Range {
                start: Position {
                    line,
                    character: 0,
                },
                end: Position {
                    line,
                    character: u32::MAX,
                },
            },
            severity: Some(DiagnosticSeverity::WARNING),
            code: None,
            code_description: None,
            source: Some("ducky-compiler".to_string()),
            message: warning.message.clone(),
            related_information: None,
            tags: None,
            data: None,
        };
        diagnostics.push(diagnostic);
    }

    diagnostics
}
