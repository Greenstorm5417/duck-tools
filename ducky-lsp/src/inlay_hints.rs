use tower_lsp::lsp_types::*;

/// Get inlay hints for a document range
pub fn get_inlay_hints(content: &str, range: Range) -> Vec<InlayHint> {
    let mut hints = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    // Only show DELAY hints (in seconds)
    for (line_idx, line) in lines.iter().enumerate() {
        let line_num = line_idx as u32;
        
        // Skip lines outside the requested range
        if line_num < range.start.line || line_num > range.end.line {
            continue;
        }
        
        let trimmed = line.trim();
        
        // Hint for DELAY values (show in seconds)
        if trimmed.starts_with("DELAY ") {
            if let Some(delay_str) = trimmed.strip_prefix("DELAY ") {
                if let Ok(ms) = delay_str.trim().parse::<u64>() {
                    let seconds = ms as f64 / 1000.0;
                    hints.push(InlayHint {
                        position: Position {
                            line: line_num,
                            character: line.len() as u32,
                        },
                        label: InlayHintLabel::String(format!(" ({}s)", seconds)),
                        kind: Some(InlayHintKind::PARAMETER),
                        text_edits: None,
                        tooltip: Some(InlayHintTooltip::String(format!("{} seconds", seconds))),
                        padding_left: Some(true),
                        padding_right: None,
                        data: None,
                    });
                }
            }
        }
    }
    
    hints
}
