use crate::errors::{CompilerError, CompilerResult, CompilerWarning};
use crate::lexer::{
    is_define, is_elsedef, is_end_string, is_endifdef, is_ifdef, is_ifnotdef,
    is_preprocessor_disabled, is_rem, is_string_block, is_stringln_block,
};

pub struct Preprocessor {
    labels: Vec<String>,
    replacements: Vec<String>,
    pub warnings: Vec<CompilerWarning>,
}

impl Default for Preprocessor {
    fn default() -> Self {
        Self::new()
    }
}

impl Preprocessor {
    pub fn new() -> Self {
        Preprocessor {
            labels: Vec::new(),
            replacements: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn gather_defines(&mut self, lines: &[String]) -> CompilerResult<()> {
        for (line_idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if is_define(trimmed) {
                let words: Vec<&str> = trimmed.split_whitespace().collect();
                if words.len() < 3 {
                    continue;
                }
                let label = words[1].to_string();
                let replacement = trimmed
                    .split(&format!("DEFINE {} ", label))
                    .nth(1)
                    .unwrap_or("")
                    .to_string();

                if self.labels.contains(&label) {
                    return Err(CompilerError::DuplicateDefinition {
                        line: line_idx + 1,
                        name: label,
                    });
                }

                self.labels.push(label);
                self.replacements.push(replacement);
            }
        }
        Ok(())
    }

    pub fn parse_ifdefs(&self, lines: Vec<String>) -> CompilerResult<Vec<String>> {
        let mut result = Vec::with_capacity(lines.len());
        let mut ifdef_stack: Vec<(String, bool)> = Vec::new();

        for line in lines {
            {
                let trimmed = line.trim();
                let mut label = String::new();

                if is_ifdef(trimmed) || is_ifnotdef(trimmed) {
                    let words: Vec<&str> = trimmed.split_whitespace().collect();
                    if words.len() >= 2 {
                        label = words[1].to_string();
                    }
                }

                if is_ifdef(trimmed) {
                    let found = self.search_label(&label, "TRUE");
                    ifdef_stack.push((trimmed.to_string(), found));
                } else if is_ifnotdef(trimmed) {
                    let found = !self.search_label(&label, "FALSE");
                    ifdef_stack.push((trimmed.to_string(), found));
                } else if is_elsedef(trimmed) {
                    if let Some((mode, val)) = ifdef_stack.pop() {
                        ifdef_stack.push((mode, !val));
                    }
                } else if is_endifdef(trimmed) {
                    ifdef_stack.pop();
                }
            }

            let mut inside_enabled_code = true;
            for (mode, current_val) in &ifdef_stack {
                if is_ifdef(mode) {
                    if !current_val {
                        inside_enabled_code = false;
                        break;
                    }
                } else if is_ifnotdef(mode) && *current_val {
                    inside_enabled_code = false;
                    break;
                }
            }

            if !inside_enabled_code {
                result.push(format!("PREPROCESSOR_DISABLED {}", line));
            } else {
                result.push(line);
            }
        }

        Ok(result)
    }

    fn search_label(&self, label: &str, expected: &str) -> bool {
        for (i, l) in self.labels.iter().enumerate() {
            if l == label {
                return self
                    .replacements
                    .get(i)
                    .map(|r| r.trim() == expected)
                    .unwrap_or(false);
            }
        }

        expected == "FALSE"
    }

    pub fn process(&mut self, lines: Vec<String>) -> Vec<String> {
        let mut result = Vec::with_capacity(lines.len());
        let mut inside_string_block = false;
        let mut inside_stringln_block = false;
        let mut suppress_next_warning = false;

        for (line_num, line) in lines.into_iter().enumerate() {
            let trimmed = line.trim();

            if trimmed.starts_with("REM ducky-ignore-next-line") {
                suppress_next_warning = true;
                result.push(line);
                continue;
            }
            if trimmed.is_empty() {
                result.push(line);
                continue;
            }

            if is_rem(trimmed)
                || is_define(trimmed)
                || is_ifdef(trimmed)
                || is_ifnotdef(trimmed)
                || is_elsedef(trimmed)
                || is_preprocessor_disabled(trimmed)
            {
                result.push(line);
                continue;
            }

            if is_end_string(trimmed) {
                inside_string_block = false;
                inside_stringln_block = false;
            } else if is_string_block(trimmed) {
                inside_string_block = true;
            } else if is_stringln_block(trimmed) {
                inside_stringln_block = true;
            }

            let in_block = inside_string_block || inside_stringln_block;
            let mut modified_line = line;

            if in_block {
                if modified_line.contains('#') {
                    for (i, label) in self.labels.iter().enumerate() {
                        if label.starts_with('#') && modified_line.contains(label) {
                            if !suppress_next_warning {
                                self.warnings.push(CompilerWarning::DefineReplacement {
                                    line: line_num + 1,
                                    label: label.clone(),
                                    old: label.clone(),
                                    new: self.replacements[i].clone(),
                                });
                            }
                            modified_line = modified_line.replace(label, &self.replacements[i]);
                        }
                    }
                }

                suppress_next_warning = false;
                result.push(modified_line);
                continue;
            }

            for (i, label) in self.labels.iter().enumerate() {
                if !label.starts_with('#') && modified_line.split_whitespace().any(|w| w == label) {
                    let words: Vec<&str> = modified_line.split_whitespace().collect();
                    let mut new_words = Vec::with_capacity(words.len());
                    for word in words {
                        if word == label {
                            new_words.push(self.replacements[i].clone());
                            if !suppress_next_warning {
                                self.warnings.push(CompilerWarning::DefineReplacement {
                                    line: line_num + 1,
                                    label: label.clone(),
                                    old: label.clone(),
                                    new: self.replacements[i].clone(),
                                });
                            }
                        } else {
                            new_words.push(word.to_string());
                        }
                    }
                    modified_line = new_words.join(" ");
                    break;
                }
            }

            if modified_line.contains('#') {
                for (i, label) in self.labels.iter().enumerate() {
                    if label.starts_with('#') && modified_line.contains(label) {
                        if !suppress_next_warning {
                            self.warnings.push(CompilerWarning::DefineReplacement {
                                line: line_num + 1,
                                label: label.clone(),
                                old: label.clone(),
                                new: self.replacements[i].clone(),
                            });
                        }
                        modified_line = modified_line.replace(label, &self.replacements[i]);
                    }
                }
            }

            suppress_next_warning = false;
            result.push(modified_line);
        }

        result
    }
}
