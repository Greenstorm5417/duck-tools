use ducky_core::DuckyCompiler;
use tower_lsp::lsp_types::*;

/// Convert compiler errors and warnings to LSP Diagnostics
pub fn compiler_to_diagnostics(compiler: &DuckyCompiler, _uri: &Url) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Convert errors
    for error in &compiler.errors {
        // Extract line number from error (already 1-indexed from compiler)
        // Subtract 1 for LSP which uses 0-indexed lines
        let error_line = error.line().unwrap_or(compiler.current_line_index + 1);
        let line = if error_line > 0 {
            (error_line - 1) as u32
        } else {
            compiler.current_line_index as u32
        };
        let column = error.column().unwrap_or(0) as u32;
        let length = error.length().unwrap_or(0) as u32;
        
        let diagnostic = Diagnostic {
            range: Range {
                start: Position {
                    line,
                    character: column,
                },
                end: Position {
                    line,
                    character: if length > 0 { column + length } else { u32::MAX },
                },
            },
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String(error.code().to_string())),
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
        let line = warning.line().saturating_sub(1) as u32; // Convert to 0-indexed
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
            code: Some(NumberOrString::String(warning.code().to_string())),
            code_description: None,
            source: Some("ducky-compiler".to_string()),
            message: warning.message(),
            related_information: None,
            tags: None,
            data: None,
        };
        diagnostics.push(diagnostic);
    }

    diagnostics
}
