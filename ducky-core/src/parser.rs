use crate::constants::*;
use crate::errors::{CompilerError, CompilerResult};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Variable {
    pub address: usize,
    pub value: String,
}

#[derive(Debug)]
pub struct ParserState {
    pub var_map: HashMap<String, usize>,
    pub var_values: Vec<String>,
    pub next_var_address: usize,
    pub label_map: HashMap<String, usize>,
    pub block_stack: Vec<u16>,
    pub current_block: u16,
    pub next_block_address: u16,
    pub total_eval_registers: usize,
    pub free_eval_registers: usize,
    pub requires_lang_pack: bool,
}

impl ParserState {
    pub fn new() -> Self {
        ParserState {
            var_map: HashMap::new(),
            var_values: vec!["VALUES".to_string()],
            next_var_address: 1,
            label_map: HashMap::new(),
            block_stack: Vec::new(),
            current_block: 0,
            next_block_address: 0,
            total_eval_registers: 0,
            free_eval_registers: 0,
            requires_lang_pack: false,
        }
    }

    pub fn allocate_var(&mut self, name: &str) -> usize {
        let addr = self.next_var_address;
        self.var_map.insert(name.to_string(), addr);
        self.var_values.push("0000".to_string());
        self.next_var_address += 1;
        addr
    }

    pub fn assign_value(&mut self, name: &str, value: String) -> CompilerResult<usize> {
        if let Some(&addr) = self.var_map.get(name) {
            if addr < self.var_values.len() {
                self.var_values[addr] = value;
                Ok(addr)
            } else {
                Err(CompilerError::UnknownVariable(name.to_string()))
            }
        } else {
            Err(CompilerError::UnknownVariable(name.to_string()))
        }
    }

    pub fn get_var_address(&self, name: &str) -> Option<usize> {
        self.var_map.get(name).copied()
    }

    pub fn var_exists(&self, name: &str) -> bool {
        self.var_map.contains_key(name)
    }

    pub fn is_reserved_var(var: &str) -> bool {
        RESERVED_VARIABLES.contains_key(var)
    }

    pub fn is_reserved_constant(constant: &str) -> bool {
        RESERVED_CONSTANTS.contains_key(constant)
    }

    pub fn is_operator(op: &str) -> bool {
        OPERATOR_MAP.contains_key(op)
    }

    pub fn is_var(word: &str) -> bool {
        word.starts_with('$')
    }

    pub fn is_hex(word: &str) -> bool {
        word.starts_with("0x")
    }

    pub fn is_numeric(word: &str) -> bool {
        word.parse::<i32>().is_ok() || Self::is_hex(word)
    }

    pub fn contains_function_call(word: &str) -> bool {
        word.contains("()")
    }

    pub fn label_exists(&self, label: &str) -> bool {
        self.label_map.contains_key(label)
    }

    pub fn create_label(&mut self, label: &str, address: usize) -> CompilerResult<()> {
        if self.label_exists(label) {
            return Err(CompilerError::DuplicateDefinition(label.to_string()));
        }
        self.label_map.insert(label.to_string(), address);
        Ok(())
    }

    pub fn next_register(&mut self) {
        if self.free_eval_registers == 0 {
            self.total_eval_registers += 1;
            self.free_eval_registers += 1;
        }
    }

    pub fn current_register(&self) -> String {
        format!("$__r{}", self.total_eval_registers - self.free_eval_registers)
    }
}

pub fn split_syntax_line(line: &str) -> Vec<String> {
    let mut result = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut current = String::new();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        let next = if i + 1 < chars.len() {
            Some(chars[i + 1])
        } else {
            None
        };

        if c == '(' && next == Some(')') && !current.is_empty() {
            result.push(format!("{}()", current));
            current.clear();
            i += 2;
            continue;
        }

        if c == ' ' {
            if !current.is_empty() {
                result.push(current.clone());
                current.clear();
            }
            i += 1;
            continue;
        }

        let double_op = if let Some(n) = next {
            let two_char = format!("{}{}", c, n);
            DOUBLE_OPERATORS.contains(&two_char.as_str())
        } else {
            false
        };

        if double_op {
            if !current.is_empty() {
                result.push(current.clone());
                current.clear();
            }
            result.push(format!("{}{}", c, next.unwrap()));
            i += 2;
        } else if "()".contains(c) || OPERATOR_MAP.contains_key(&c.to_string().as_str()) {
            if !current.is_empty() {
                result.push(current.clone());
                current.clear();
            }
            result.push(c.to_string());
            i += 1;
        } else {
            current.push(c);
            i += 1;
        }
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}
