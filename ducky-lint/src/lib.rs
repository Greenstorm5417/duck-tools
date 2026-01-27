use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinterConfig {
    #[serde(default)]
    pub enabled: bool,
    
    #[serde(default = "default_true")]
    pub check_line_length: bool,
    
    #[serde(default = "default_max_line_length")]
    pub max_line_length: usize,
    
    #[serde(default = "default_true")]
    pub check_trailing_whitespace: bool,
    
    #[serde(default = "default_true")]
    pub check_mixed_indentation: bool,
    
    #[serde(default = "default_true")]
    pub check_unused_variables: bool,
    
    #[serde(default = "default_true")]
    pub check_undefined_labels: bool,
    
    #[serde(default)]
    pub require_final_newline: bool,
    
    #[serde(default)]
    pub warn_on_deprecated_syntax: bool,
    
    #[serde(default)]
    pub check_suspicious_delays: bool,
    
    #[serde(default = "default_suspicious_delay_threshold")]
    pub suspicious_delay_threshold: u32,
}

fn default_true() -> bool { true }
fn default_max_line_length() -> usize { 120 }
fn default_suspicious_delay_threshold() -> u32 { 10000 }

impl Default for LinterConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            check_line_length: true,
            max_line_length: 120,
            check_trailing_whitespace: true,
            check_mixed_indentation: true,
            check_unused_variables: true,
            check_undefined_labels: true,
            require_final_newline: false,
            warn_on_deprecated_syntax: false,
            check_suspicious_delays: false,
            suspicious_delay_threshold: 10000,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LintIssue {
    pub line: usize,
    pub column: usize,
    pub severity: LintSeverity,
    pub message: String,
    pub rule: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LintSeverity {
    Error,
    Warning,
    Info,
}

pub struct DuckyLinter {
    config: LinterConfig,
}

impl DuckyLinter {
    pub fn new(config: LinterConfig) -> Self {
        Self { config }
    }
    
    pub fn lint(&self, source: &str) -> Vec<LintIssue> {
        if !self.config.enabled {
            return Vec::new();
        }
        
        let mut issues = Vec::new();
        
        for (line_idx, line) in source.lines().enumerate() {
            let line_num = line_idx + 1;
            
            if self.config.check_line_length && line.len() > self.config.max_line_length {
                issues.push(LintIssue {
                    line: line_num,
                    column: self.config.max_line_length,
                    severity: LintSeverity::Warning,
                    message: format!("Line exceeds maximum length of {} characters", self.config.max_line_length),
                    rule: "max-line-length".to_string(),
                });
            }
            
            if self.config.check_trailing_whitespace && line.ends_with(' ') || line.ends_with('\t') {
                issues.push(LintIssue {
                    line: line_num,
                    column: line.trim_end().len(),
                    severity: LintSeverity::Info,
                    message: "Trailing whitespace detected".to_string(),
                    rule: "no-trailing-whitespace".to_string(),
                });
            }
            
            if self.config.check_mixed_indentation {
                if line.starts_with(' ') && line.contains('\t') {
                    issues.push(LintIssue {
                        line: line_num,
                        column: 0,
                        severity: LintSeverity::Warning,
                        message: "Mixed spaces and tabs in indentation".to_string(),
                        rule: "no-mixed-indentation".to_string(),
                    });
                }
            }
            
            if self.config.check_suspicious_delays {
                if let Some(delay_str) = line.trim().strip_prefix("DELAY ") {
                    if let Ok(delay) = delay_str.trim().parse::<u32>() {
                        if delay > self.config.suspicious_delay_threshold {
                            issues.push(LintIssue {
                                line: line_num,
                                column: 0,
                                severity: LintSeverity::Warning,
                                message: format!("Suspicious delay value: {}ms (very long)", delay),
                                rule: "suspicious-delay".to_string(),
                            });
                        }
                    }
                }
            }
        }
        
        if self.config.require_final_newline && !source.ends_with('\n') {
            issues.push(LintIssue {
                line: source.lines().count(),
                column: source.lines().last().map(|l| l.len()).unwrap_or(0),
                severity: LintSeverity::Info,
                message: "File should end with a newline".to_string(),
                rule: "final-newline".to_string(),
            });
        }
        
        issues
    }
}
