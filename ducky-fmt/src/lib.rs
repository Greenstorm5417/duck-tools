use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatterConfig {
    #[serde(default)]
    pub enabled: bool,
    
    #[serde(default = "default_indent_size")]
    pub indent_size: usize,
    
    #[serde(default = "default_use_tabs")]
    pub use_tabs: bool,
    
    #[serde(default = "default_max_line_length")]
    pub max_line_length: usize,
    
    #[serde(default = "default_true")]
    pub align_comments: bool,
    
    #[serde(default = "default_true")]
    pub trim_trailing_whitespace: bool,
    
    #[serde(default = "default_true")]
    pub insert_final_newline: bool,
    
    #[serde(default)]
    pub space_after_command: bool,
}

fn default_indent_size() -> usize { 4 }
fn default_use_tabs() -> bool { false }
fn default_max_line_length() -> usize { 120 }
fn default_true() -> bool { true }

impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            indent_size: 4,
            use_tabs: false,
            max_line_length: 120,
            align_comments: true,
            trim_trailing_whitespace: true,
            insert_final_newline: true,
            space_after_command: false,
        }
    }
}

pub struct DuckyFormatter {
    config: FormatterConfig,
}

impl DuckyFormatter {
    pub fn new(config: FormatterConfig) -> Self {
        Self { config }
    }
    
    pub fn format(&self, source: &str) -> anyhow::Result<String> {
        if !self.config.enabled {
            return Ok(source.to_string());
        }
        
        let mut formatted = String::new();
        let mut indent_level: usize = 0;
        
        for line in source.lines() {
            let trimmed = line.trim();
            
            if trimmed.is_empty() {
                formatted.push('\n');
                continue;
            }
            
            if trimmed.starts_with("END_") || trimmed == "ELSE" || trimmed.starts_with("ELSE ") {
                indent_level = indent_level.saturating_sub(1);
            }
            
            let indent = if self.config.use_tabs {
                "\t".repeat(indent_level)
            } else {
                " ".repeat(indent_level * self.config.indent_size)
            };
            
            let mut formatted_line = trimmed.to_string();
            
            if self.config.trim_trailing_whitespace {
                formatted_line = formatted_line.trim_end().to_string();
            }
            
            formatted.push_str(&indent);
            formatted.push_str(&formatted_line);
            formatted.push('\n');
            
            if trimmed.starts_with("IF ") || trimmed.starts_with("WHILE ") 
                || trimmed.starts_with("FUNCTION ") || trimmed == "ELSE"
                || trimmed.starts_with("ELSE ") || trimmed == "STRING"
                || trimmed == "STRINGLN" || trimmed.starts_with("STRING_")
                || trimmed.starts_with("STRINGLN_") {
                indent_level += 1;
            }
        }
        
        if self.config.insert_final_newline && !formatted.ends_with('\n') {
            formatted.push('\n');
        }
        
        Ok(formatted)
    }
}
