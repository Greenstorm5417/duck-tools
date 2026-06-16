use crate::errors::{CompilerError, CompilerResult, CompilerWarning};
use crate::lexer::{
    DEFINE_REGEX, ELSEDEF_REGEX, END_STRING_REGEX, ENDIFDEF_REGEX, IFDEF_REGEX, IFNOTDEF_REGEX,
    PREPROCESSOR_DISABLED, REM_REGEX, STRING_BLOCK_REGEX, STRINGLN_BLOCK_REGEX,
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
            if DEFINE_REGEX.is_match(trimmed) {
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
        let mut result = Vec::new();
        let mut ifdef_stack: Vec<(String, bool)> = Vec::new();

        for line in lines.iter() {
            let trimmed = line.trim();
            let found;
            let mut label = String::new();

            if IFDEF_REGEX.is_match(trimmed) || IFNOTDEF_REGEX.is_match(trimmed) {
                let words: Vec<&str> = trimmed.split_whitespace().collect();
                if words.len() >= 2 {
                    label = words[1].to_string();
                }
            }

            if IFDEF_REGEX.is_match(trimmed) {
                found = self.search_label(&label, "TRUE");
                ifdef_stack.push((trimmed.to_string(), found));
            } else if IFNOTDEF_REGEX.is_match(trimmed) {
                found = !self.search_label(&label, "FALSE");
                ifdef_stack.push((trimmed.to_string(), found));
            } else if ELSEDEF_REGEX.is_match(trimmed) {
                if let Some((mode, val)) = ifdef_stack.pop() {
                    ifdef_stack.push((mode, !val));
                }
            } else if ENDIFDEF_REGEX.is_match(trimmed) {
                ifdef_stack.pop();
            }

            let mut inside_enabled_code = true;
            for (mode, current_val) in &ifdef_stack {
                if IFDEF_REGEX.is_match(mode) {
                    if !current_val {
                        inside_enabled_code = false;
                        break;
                    }
                } else if IFNOTDEF_REGEX.is_match(mode) && *current_val {
                    inside_enabled_code = false;
                    break;
                }
            }

            if !inside_enabled_code {
                result.push(format!("PREPROCESSOR_DISABLED {}", line));
            } else {
                result.push(line.clone());
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
        let mut result = Vec::new();
        let mut inside_string_block = false;
        let mut inside_stringln_block = false;
        let mut suppress_next_warning = false;

        for (line_num, line) in lines.iter().enumerate() {
            // Check if this line has a suppression comment
            let trimmed = line.trim();
            if trimmed.starts_with("REM ducky-ignore-next-line") {
                suppress_next_warning = true;
                result.push(line.clone());
                continue;
            }
            if line.trim().is_empty() {
                result.push(line.clone());
                continue;
            }

            if REM_REGEX.is_match(line.trim())
                || DEFINE_REGEX.is_match(line.trim())
                || IFDEF_REGEX.is_match(line.trim())
                || IFNOTDEF_REGEX.is_match(line.trim())
                || ELSEDEF_REGEX.is_match(line.trim())
                || PREPROCESSOR_DISABLED.is_match(line.trim())
            {
                result.push(line.clone());
                continue;
            }

            let trimmed = line.trim();

            if END_STRING_REGEX.is_match(trimmed) {
                inside_string_block = false;
                inside_stringln_block = false;
            } else if STRING_BLOCK_REGEX.is_match(trimmed) {
                inside_string_block = true;
            } else if STRINGLN_BLOCK_REGEX.is_match(trimmed) {
                inside_stringln_block = true;
            }

            let mut modified_line = line.clone();
            if inside_string_block || inside_stringln_block {
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

                suppress_next_warning = false;
                result.push(modified_line);
                continue;
            }

            let mut replacement_made = false;
            for (i, label) in self.labels.iter().enumerate() {
                if !label.starts_with('#') && modified_line.split_whitespace().any(|w| w == label) {
                    let words: Vec<&str> = modified_line.split_whitespace().collect();
                    let mut new_words = Vec::new();
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
                            replacement_made = true;
                        } else {
                            new_words.push(word.to_string());
                        }
                    }
                    modified_line = new_words.join(" ");
                    break;
                }
            }

            if !replacement_made {
                modified_line = line.clone();
            }

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

            suppress_next_warning = false;
            result.push(modified_line.clone());
        }

        result
    }
}
