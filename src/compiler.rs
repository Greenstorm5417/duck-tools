use crate::constants::*;
use crate::encoder::*;
use crate::errors::{CompilerError, CompilerResult, CompilerWarning};
use crate::language::KeyboardLayout;
use crate::lexer::*;
use crate::parser::{split_syntax_line, ParserState};
use crate::preprocessor::Preprocessor;
use std::collections::HashMap;

pub struct DuckyCompiler {
    pub state: ParserState,
    pub keyboard_layout: KeyboardLayout,
    pub output_buffer: Vec<String>,
    pub errors: Vec<CompilerError>,
    pub warnings: Vec<CompilerWarning>,
    pub ds3_detected: bool,
    pub default_delay: u32,
    pub delay_override: bool,
    pub current_line: String,
    pub current_line_index: usize,
    pub inside_string_block: bool,
    pub inside_stringln_block: bool,
    pub string_block_indentation_level: usize,
    pub preserve_leading_space: bool,
    pub inside_rem_block: bool,
    pub awaiting_future_addr: Vec<Vec<(usize, String, usize)>>,
    pub goto_awaiting_label: HashMap<String, Vec<usize>>,
    pub defining_function: bool,
    pub return_defined: bool,
    pub defining_button: u32,
    pub button_block_stack: Vec<u32>,
    pub previous_line: String,
    pub modifier_queue: Vec<String>,
}

impl DuckyCompiler {
    pub fn new(keyboard_layout: Option<KeyboardLayout>) -> Self {
        DuckyCompiler {
            state: ParserState::new(),
            keyboard_layout: keyboard_layout.unwrap_or_else(KeyboardLayout::default_us),
            output_buffer: Vec::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
            ds3_detected: false,
            default_delay: 0,
            delay_override: false,
            current_line: String::new(),
            current_line_index: 0,
            inside_string_block: false,
            inside_stringln_block: false,
            string_block_indentation_level: 1,
            preserve_leading_space: false,
            inside_rem_block: false,
            awaiting_future_addr: Vec::new(),
            goto_awaiting_label: HashMap::new(),
            defining_function: false,
            return_defined: false,
            defining_button: 0,
            button_block_stack: Vec::new(),
            previous_line: String::new(),
            modifier_queue: Vec::new(),
        }
    }

    pub fn compile(&mut self, source: &str) -> CompilerResult<Vec<u8>> {
        let sanitized = self.sanitize_input(source);
        let lines: Vec<String> = sanitized.lines().map(|l| l.to_string()).collect();

        let mut preprocessor = Preprocessor::new();
        preprocessor.gather_defines(&lines)?;
        
        let lines = preprocessor.parse_ifdefs(lines)?;
        let lines = preprocessor.process(lines);
        
        self.warnings.extend(preprocessor.warnings);

        for (index, line) in lines.iter().enumerate() {
            self.current_line = line.clone();
            self.current_line_index = index;

            if let Err(e) = self.process_line(line) {
                self.errors.push(e);
                if self.errors.len() > 50 {
                    break;
                }
            }

            if let Some(delay_bytes) = self.check_delay() {
                append_hex_string_array(&mut self.output_buffer, delay_bytes)?;
            }
            
            self.state.free_eval_registers = self.state.total_eval_registers;
            self.previous_line = line.clone();

            if self.output_buffer.len() > MAX_PAYLOAD_SIZE {
                return Err(CompilerError::PayloadTooLarge {
                    size: self.output_buffer.len(),
                });
            }
        }

        self.post_process()?;
        self.finalize()
    }

    fn sanitize_input(&self, input: &str) -> String {
        input.replace('\r', "").replace('"', "\"")
    }

    fn strip_block_indentation(&self, line: &str) -> String {
        let mut result = line.to_string();
        for _ in 0..self.string_block_indentation_level {
            if let Some(rest) = result.strip_prefix("    ") {
                result = rest.to_string();
            }
        }
        result
    }

    fn compute_string_block_indentation_level(&self, line: &str) -> usize {
        let mut level = 1;
        let mut tmp = line;
        while let Some(rest) = tmp.strip_prefix("    ") {
            level += 1;
            tmp = rest;
        }
        level
    }

    fn process_line(&mut self, line: &str) -> CompilerResult<()> {
        if line.trim().is_empty() {
            return Ok(());
        }

        if self.inside_rem_block {
            if END_REM_BLOCK_REGEX.is_match(line.trim()) {
                self.inside_rem_block = false;
            }
            return Ok(());
        }

        if self.inside_string_block || self.inside_stringln_block {
            if END_STRING_REGEX.is_match(line.trim()) {
                self.inside_string_block = false;
                self.inside_stringln_block = false;
                self.string_block_indentation_level = 1;
                self.preserve_leading_space = false;
                return Ok(());
            }

            let content = if self.inside_string_block {
                if self.preserve_leading_space {
                    self.strip_block_indentation(line)
                } else {
                    line.trim_start().to_string()
                }
            } else {
                self.strip_block_indentation(line)
            };

            for ch in content.chars() {
                self.inject_character(&ch.to_string())?;
            }
            
            if self.inside_stringln_block {
                self.inject_character("ENTER")?;
            }
            
            return Ok(());
        }

        let token_type = tokenize_line(line);

        match token_type {
            TokenType::PreprocessorDisabled => Ok(()),
            TokenType::RemBlock => {
                self.inside_rem_block = true;
                Ok(())
            }
            TokenType::EndRemBlock => {
                self.inside_rem_block = false;
                Ok(())
            }
            TokenType::Rem | TokenType::Comment => Ok(()),
            TokenType::Define | TokenType::IfDefined | TokenType::IfNotDefined
            | TokenType::ElseDefined | TokenType::EndIfDefined => Ok(()),
            
            TokenType::Delay => self.handle_delay(line),
            TokenType::DelayVar => self.handle_delay_var(line),
            TokenType::DefaultDelay => self.handle_default_delay(line),
            TokenType::StringDelay => self.handle_string_delay(line),
            
            TokenType::String => self.handle_string(line),
            TokenType::StringLn => self.handle_stringln(line),
            TokenType::StringBlock => {
                self.inside_string_block = true;
                self.preserve_leading_space = line.to_ascii_uppercase().contains("PYTHON");
                self.string_block_indentation_level = self.compute_string_block_indentation_level(line);
                Ok(())
            }
            TokenType::StringLnBlock => {
                self.inside_stringln_block = true;
                self.preserve_leading_space = false;
                self.string_block_indentation_level = self.compute_string_block_indentation_level(line);
                Ok(())
            }
            TokenType::EndString => {
                self.inside_string_block = false;
                self.inside_stringln_block = false;
                self.string_block_indentation_level = 1;
                self.preserve_leading_space = false;
                Ok(())
            }
            
            TokenType::If => self.handle_if(line, false),
            TokenType::ElseIf => self.handle_else_if(line),
            TokenType::Else => self.handle_else(line),
            TokenType::EndIf => self.handle_end_if(line, true),
            
            TokenType::While => self.handle_while(line),
            TokenType::EndWhile => self.handle_end_while(line),
            
            TokenType::Assignment | TokenType::Declaration => self.handle_assignment(line),
            
            TokenType::Function => self.handle_function_def(line),
            TokenType::EndFunction => self.handle_end_function(line),
            TokenType::FunctionCall => self.handle_function_call(line),
            TokenType::Return => self.handle_return(line),
            
            TokenType::Enter => self.handle_enter(line),
            TokenType::Repeat => self.handle_repeat(line),
            TokenType::Hold => self.handle_hold(line),
            TokenType::Release => self.handle_release(line),
            
            TokenType::Inject => self.handle_inject(line),
            TokenType::InjectMod => self.handle_inject_mod(line),
            TokenType::InjectVar => self.handle_inject_var(line),
            TokenType::Keycode => self.handle_keycode(line),
            TokenType::ExfilVar => self.handle_exfil_var(line),

            TokenType::ModKeyDown => self.handle_mod_key_down(line),
            TokenType::ModKeyUp => self.handle_mod_key_up(line),
            TokenType::KeyDown => self.handle_key_down(line),
            TokenType::KeyUp => self.handle_key_up(line),
            TokenType::ModDown => self.handle_mod_down(line),
            TokenType::ModUp => self.handle_mod_up(line),
            
            TokenType::Modifier => self.handle_modifier(line),
            
            TokenType::Stage | TokenType::EndStage => Ok(()),
            TokenType::Extension | TokenType::EndExtension => Ok(()),
            
            TokenType::ButtonDef => self.handle_button_def(line),
            TokenType::EndButtonDef => self.handle_end_button_def(line),
            
            TokenType::Attackmode => self.handle_attackmode(line),
            
            TokenType::RandomLowercaseLetter => self.handle_random_lowercase(line),
            TokenType::RandomUppercaseLetter => self.handle_random_uppercase(line),
            TokenType::RandomNumber => self.handle_random_number(line),
            TokenType::RandomLetter => self.handle_random_letter(line),
            TokenType::RandomSpecial => self.handle_random_special(line),
            TokenType::RandomChar => self.handle_random_char(line),
            
            TokenType::Breakpoint => self.handle_breakpoint(line),
            TokenType::InjectBreakpointLineNumber => self.handle_inject_breakpoint_line_number(line),
            
            TokenType::Unknown => self.handle_unknown(line),
        }
    }

    fn mark_ds3(&mut self) {
        self.ds3_detected = true;
    }

    fn handle_delay(&mut self, line: &str) -> CompilerResult<()> {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() < 2 {
            return Ok(());
        }
        
        if let Ok(delay) = parts[1].parse::<u32>() {
            self.delay_override = true;
            let delay_bytes = build_delay_bytes(delay);
            append_hex_string_array(&mut self.output_buffer, delay_bytes)?;
        }
        Ok(())
    }

    fn inject_breakpoint_line_number(&mut self) -> CompilerResult<()> {
        let line_num = self.current_line_index + 3;
        let s = line_num.to_string();
        for ch in s.chars() {
            self.inject_character(&ch.to_string())?;
        }
        self.inject_character(";")?;
        Ok(())
    }

    fn handle_inject_breakpoint_line_number(&mut self, _line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        self.inject_breakpoint_line_number()?;
        self.delay_override = true;
        let delay_bytes = build_delay_bytes(800);
        append_hex_string_array(&mut self.output_buffer, delay_bytes)?;
        Ok(())
    }

    fn handle_breakpoint(&mut self, _line: &str) -> CompilerResult<()> {
        self.mark_ds3();

        self.inject_character("ENTER")?;
        for ch in "BREAKPOINT ".chars() {
            self.inject_character(&ch.to_string())?;
        }
        self.inject_breakpoint_line_number()?;

        if let Some(&builtin) = BUILTINS_MAP.get("LED_R") {
            append_hex_string_array(&mut self.output_buffer, vec![hex_encode(builtin)])?;
        }
        if let Some(&builtin) = BUILTINS_MAP.get("WAIT_FOR_BUTTON_PRESS") {
            append_hex_string_array(&mut self.output_buffer, vec![hex_encode(builtin)])?;
        }
        if let Some(&builtin) = BUILTINS_MAP.get("LED_G") {
            append_hex_string_array(&mut self.output_buffer, vec![hex_encode(builtin)])?;
        }
        self.inject_character("ENTER")?;

        Ok(())
    }

    fn parse_hex_byte_lossy(&self, s: &str) -> u8 {
        u8::from_str_radix(s.trim(), 16).unwrap_or(0)
    }

    fn extract_single_hex_operand(&self, line: &str) -> String {
        let mut parts = line.trim().split_whitespace();
        let _cmd = parts.next();
        let arg = parts.next().unwrap_or("0x00");
        let arg = arg.trim_start_matches("0x").trim_start_matches("0X");
        let b = self.parse_hex_byte_lossy(arg);
        format!("{:02x}", b)
    }

    fn extract_two_hex_operands_js_style(&self, line: &str) -> (String, String) {
        let args = line.trim().splitn(2, ' ').nth(1).unwrap_or("");
        let cleaned = args.replace("0x", "").replace("0X", "");
        let b1 = cleaned.get(0..2).unwrap_or("");
        let b2 = cleaned.get(2..4).unwrap_or("");
        let v1 = self.parse_hex_byte_lossy(b1);
        let v2 = self.parse_hex_byte_lossy(b2);
        (format!("{:02x}", v1), format!("{:02x}", v2))
    }

    fn handle_key_down(&mut self, line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0xEA0A)])?;
        let op = self.extract_single_hex_operand(line);
        append_hex_string_array(&mut self.output_buffer, vec![op])?;
        Ok(())
    }

    fn handle_key_up(&mut self, line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0xEA0B)])?;
        let op = self.extract_single_hex_operand(line);
        append_hex_string_array(&mut self.output_buffer, vec![op])?;
        Ok(())
    }

    fn handle_mod_down(&mut self, line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0xEA0C)])?;
        let op = self.extract_single_hex_operand(line);
        append_hex_string_array(&mut self.output_buffer, vec![op])?;
        Ok(())
    }

    fn handle_mod_up(&mut self, line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0xEA0F)])?;
        let op = self.extract_single_hex_operand(line);
        append_hex_string_array(&mut self.output_buffer, vec![op])?;
        Ok(())
    }

    fn handle_mod_key_down(&mut self, line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0xEA0D)])?;
        let (b1, b2) = self.extract_two_hex_operands_js_style(line);
        append_hex_string_array(&mut self.output_buffer, vec![b1, b2])?;
        Ok(())
    }

    fn handle_mod_key_up(&mut self, line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0xEA0E)])?;
        let (b1, b2) = self.extract_two_hex_operands_js_style(line);
        append_hex_string_array(&mut self.output_buffer, vec![b1, b2])?;
        Ok(())
    }

    fn handle_delay_var(&mut self, line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() < 2 {
            return Ok(());
        }
        
        let var_name = parts[1];
        append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0xE9E7)])?;

        if let Some(addr) = self.state.get_var_address(var_name) {
            let hex_addr = dec_to_hex(addr);
            append_hex_string_array(&mut self.output_buffer, vec![hex_addr])?;
        } else if ParserState::is_reserved_var(var_name) {
            if let Some(&val) = RESERVED_VARIABLES.get(var_name) {
                append_hex_string_array(&mut self.output_buffer, vec![hex_encode(val)])?;
            } else {
                return Err(CompilerError::UnknownVariable(var_name.to_string()));
            }
        } else {
            return Err(CompilerError::UnknownVariable(var_name.to_string()));
        }
        Ok(())
    }

    fn handle_default_delay(&mut self, line: &str) -> CompilerResult<()> {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 2 {
            if let Ok(delay) = parts[1].parse::<u32>() {
                self.default_delay = delay;
                self.delay_override = true;
            }
        }
        Ok(())
    }

    fn handle_string_delay(&mut self, line: &str) -> CompilerResult<()> {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 2 {
            if let Ok(_delay) = parts[1].parse::<u32>() {
            }
        }
        Ok(())
    }

    fn get_args_from_line(&self, line: &str) -> String {
        let trimmed = line.trim_start();
        if let Some(first_space_idx) = trimmed.find(' ') {
            trimmed[first_space_idx + 1..].to_string()
        } else {
            String::new()
        }
    }

    fn strip_inline_end_marker_string(&self, mut args: String) -> String {
        if args.ends_with(" END_STRING") {
            args.truncate(args.len().saturating_sub(" END_STRING".len()));
        }
        args
    }

    fn strip_inline_end_marker_stringln(&self, mut args: String) -> String {
        if args.ends_with(" END_STRINGLN") {
            args.truncate(args.len().saturating_sub(" END_STRINGLN".len()));
        }
        args
    }

    fn handle_string(&mut self, line: &str) -> CompilerResult<()> {
        let args = self.get_args_from_line(line);
        let content = self.strip_inline_end_marker_string(args);
        
        for ch in content.chars() {
            self.inject_character(&ch.to_string())?;
        }
        Ok(())
    }

    fn handle_stringln(&mut self, line: &str) -> CompilerResult<()> {
        let args = self.get_args_from_line(line);
        let content = self.strip_inline_end_marker_stringln(args);
        
        for ch in content.chars() {
            self.inject_character(&ch.to_string())?;
        }
        self.inject_character("ENTER")?;
        Ok(())
    }

    fn inject_character(&mut self, ch: &str) -> CompilerResult<()> {
        if let Some(codes) = self.keyboard_layout.get_bytes_for_key(ch) {
            if codes.len() > 1 && codes.len() < 3 {
                // Two-element format: [modifier, keycode]
                append_hex_string_array(&mut self.output_buffer, vec![codes[0].clone(), codes[1].clone()])?;
            } else if codes.len() >= 3 {
                // Three-element format: [modifier, unused, keycode]
                let modifier = &codes[0];
                let keycode = &codes[2];
                
                if keycode != "00" {
                    if modifier != "00" {
                        // Both modifier and keycode
                        append_hex_string_array(&mut self.output_buffer, vec![keycode.clone(), modifier.clone()])?;
                    } else {
                        // Keycode only, add release
                        append_hex_string_array(&mut self.output_buffer, vec![keycode.clone(), "00".to_string()])?;
                    }
                } else {
                    // Modifier only: encode as [keycode=00, modifier]
                    append_hex_string_array(&mut self.output_buffer, vec!["00".to_string(), modifier.clone()])?;
                }
            } else if codes.len() == 1 {
                // Single byte
                append_hex_string_array(&mut self.output_buffer, vec![codes[0].clone(), "00".to_string()])?;
            }
        }
        Ok(())
    }

    fn handle_if(&mut self, line: &str, chain: bool) -> CompilerResult<()> {
        self.mark_ds3();
        self.state.next_block_address += 1;
        self.state.current_block = self.state.next_block_address;
        
        if !chain {
            self.awaiting_future_addr.push(Vec::new());
        }

        let lexemes = split_syntax_line(line);
        let mut stack: Vec<String> = Vec::new();
        let mut output_stack: Vec<Vec<String>> = Vec::new();
        let mut lp_count = 0;
        let mut rp_count = 0;

        for word in lexemes {
            if word == "IF" || word == "THEN" || word == "{" {
                continue;
            } else if ParserState::contains_function_call(&word) {
                self.state.next_register();
                let return_reg = self.state.current_register();
                stack.push(return_reg.clone());
                self.handle_function_call(&word)?;
                self.handle_assignment(&format!("{} = $_f_ret", return_reg))?;
                self.state.free_eval_registers = self.state.free_eval_registers.saturating_sub(1);
                continue;
            } else if word == "(" {
                lp_count += 1;
            } else if word == ")" {
                rp_count += 1;
                let mut temp = Vec::new();
                self.state.next_register();
                
                if stack.len() > 1 {
                    let var2 = stack.pop().unwrap();
                    let op = stack.pop().unwrap();
                    let var1 = stack.pop().unwrap();
                    let current_reg = self.state.current_register();
                    stack.push(current_reg.clone());
                    temp.push("=".to_string());
                    temp.push(current_reg);
                    temp.push(var1);
                    temp.push(var2);
                    temp.push(op);
                } else if stack.is_empty() {
                    return Err(CompilerError::EmptyExpression);
                } else {
                    let var1 = stack.pop().unwrap();
                    let current_reg = self.state.current_register();
                    stack.push(current_reg.clone());
                    temp.push("=".to_string());
                    temp.push(current_reg);
                    temp.push(var1);
                    temp.push("0000".to_string());
                }
                
                self.state.free_eval_registers = self.state.free_eval_registers.saturating_sub(1);
                output_stack.push(temp);
            } else {
                stack.push(word);
            }
        }

        if lp_count != rp_count {
            return Err(CompilerError::MismatchedParentheses {
                left: lp_count,
                right: rp_count,
            });
        }

        if stack.len() == 1 {
            output_stack.push(vec!["IF".to_string(), stack[0].clone()]);
        } else {
            output_stack.push(vec!["IF".to_string(), format!("$__r{}", self.state.total_eval_registers.saturating_sub(1))]);
        }

        let encoded = self.encode_expression_stack(&output_stack)?;
        let block_hex = dec_to_hex(self.state.current_block as usize);
        let mut final_encoded = encoded;
        final_encoded.push(block_hex);
        
        append_hex_string_array(&mut self.output_buffer, final_encoded)?;
        self.state.block_stack.push(self.state.current_block);
        
        Ok(())
    }

    fn encode_expression_stack(&mut self, output_stack: &[Vec<String>]) -> CompilerResult<Vec<String>> {
        let mut result = Vec::new();
        
        for expr in output_stack {
            for word in expr {
                if word == "0000" {
                    result.push("0000".to_string());
                } else if word == "IF" {
                    result.push(hex_encode(0xEFEF));
                } else if ParserState::is_reserved_var(word) {
                    if let Some(&val) = RESERVED_VARIABLES.get(word.as_str()) {
                        result.push(hex_encode(val));
                    }
                } else if ParserState::is_reserved_constant(word) {
                    if let Some(&val) = RESERVED_CONSTANTS.get(word.as_str()) {
                        result.push(hex_encode(val));
                    }
                } else if ParserState::is_operator(word) {
                    if let Some(&val) = OPERATOR_MAP.get(word.as_str()) {
                        result.push(hex_encode(val));
                    }
                } else if ParserState::is_var(word) {
                    if let Some(addr) = self.state.get_var_address(word) {
                        result.push(dec_to_hex(addr));
                    } else {
                        let addr = self.state.allocate_var(word);
                        self.state.assign_value(word, "0000".to_string())?;
                        result.push(dec_to_hex(addr));
                    }
                } else if word == "=" {
                    result.push(hex_encode(0x01E8));
                } else if ParserState::is_hex(word) {
                    if let Some(addr) = self.state.get_var_address(word) {
                        result.push(dec_to_hex(addr));
                    } else {
                        let addr = self.state.allocate_var(word);
                        let formatted = format_hex(word);
                        let swapped = swap_hex(&formatted);
                        self.state.assign_value(word, swapped)?;
                        result.push(dec_to_hex(addr));
                    }
                } else if ParserState::is_numeric(word) {
                    if let Some(addr) = self.state.get_var_address(word) {
                        result.push(dec_to_hex(addr));
                    } else {
                        let addr = self.state.allocate_var(word);
                        if let Ok(num) = word.parse::<u16>() {
                            self.state.assign_value(word, hex_encode(num))?;
                        }
                        result.push(dec_to_hex(addr));
                    }
                }
            }
        }
        
        Ok(result)
    }

    fn handle_else_if(&mut self, line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        append_hex_string_array(&mut self.output_buffer, vec!["F8".to_string(), "F8".to_string(), "XX".to_string(), "XX".to_string()])?;
        self.mark_future_address();
        self.handle_end_if("END_IF", false)?;
        
        let converted = line.replace("ELSE IF", "IF");
        self.handle_if(&converted, true)
    }

    fn handle_else(&mut self, _line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        append_hex_string_array(&mut self.output_buffer, vec!["F8".to_string(), "F8".to_string(), "XX".to_string(), "XX".to_string()])?;
        self.mark_future_address();
        self.handle_end_if("END_IF", false)?;
        self.handle_if("IF TRUE THEN", true)
    }

    fn mark_future_address(&mut self) {
        if let Some(chain) = self.awaiting_future_addr.last_mut() {
            chain.push((
                self.output_buffer.len() - 2,
                self.current_line.clone(),
                self.current_line_index,
            ));
        }
    }

    fn handle_end_if(&mut self, _line: &str, close_chain: bool) -> CompilerResult<()> {
        self.mark_ds3();
        if let Some(block) = self.state.block_stack.pop() {
            let block_hex = dec_to_hex(block as usize);
            append_hex_string_array(&mut self.output_buffer, vec!["1F".to_string(), "F4".to_string(), block_hex])?;
        }
        
        if close_chain {
            self.close_if_chain()?;
        }
        Ok(())
    }

    fn close_if_chain(&mut self) -> CompilerResult<()> {
        if let Some(chain) = self.awaiting_future_addr.pop() {
            for (addr, _line, _index) in chain {
                let dest_addr = self.output_buffer.len() / 2;
                let hex_addr = dec_to_hex(dest_addr);
                if addr < self.output_buffer.len() {
                    self.output_buffer[addr] = hex_addr[0..2].to_string();
                    if addr + 1 < self.output_buffer.len() {
                        self.output_buffer[addr + 1] = hex_addr[2..4].to_string();
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_while(&mut self, line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        let label = format!("generated_while_{}", self.state.next_block_address + 1);
        let addr = self.output_buffer.len();
        self.state.create_label(&label, addr)?;
        let converted = line.replace("WHILE", "IF");
        self.handle_if(&converted, false)
    }

    fn handle_end_while(&mut self, _line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        if let Some(&block) = self.state.block_stack.last() {
            let label = format!("generated_while_{}", block);
            if let Some(&addr) = self.state.label_map.get(&label) {
                let hex_addr = dec_to_hex(addr / 2);
                append_hex_string_array(&mut self.output_buffer, vec!["F8".to_string(), "F8".to_string(), hex_addr])?;
            }
        }
        self.handle_end_if("END_IF", true)
    }

    fn handle_assignment(&mut self, line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        let lexemes = split_syntax_line(line);
        let mut stack: Vec<String> = Vec::new();
        let mut output_stack: Vec<Vec<String>> = Vec::new();
        let mut lp_count = 0;
        let mut rp_count = 0;

        for word in lexemes {
            if word == "VAR" {
                continue;
            } else if ParserState::contains_function_call(&word) {
                self.state.next_register();
                let return_reg = self.state.current_register();
                stack.push(return_reg.clone());
                self.handle_function_call(&word)?;
                self.handle_assignment(&format!("{} = $_f_ret", return_reg))?;
                self.state.free_eval_registers = self.state.free_eval_registers.saturating_sub(1);
                continue;
            } else if word == "=" {
                stack.insert(0, word);
            } else if word == "(" {
                lp_count += 1;
            } else if word == ")" {
                rp_count += 1;
                let mut temp = Vec::new();
                if stack.len() > 3 {
                    self.state.next_register();
                    let var2 = stack.pop().unwrap();
                    let op = stack.pop().unwrap();
                    let var1 = stack.pop().unwrap();
                    let current_reg = self.state.current_register();
                    stack.push(current_reg.clone());
                    temp.push("=".to_string());
                    temp.push(current_reg);
                    temp.push(var1);
                    temp.push(var2);
                    temp.push(op);
                } else if !stack.is_empty() {
                    self.state.next_register();
                    let var1 = stack.pop().unwrap();
                    let current_reg = self.state.current_register();
                    stack.push(current_reg.clone());
                    temp.push("=".to_string());
                    temp.push(current_reg);
                    temp.push(var1);
                }
                self.state.free_eval_registers = self.state.free_eval_registers.saturating_sub(1);
                output_stack.push(temp);
            } else {
                stack.push(word);
            }
        }

        if lp_count != rp_count {
            return Err(CompilerError::MismatchedParentheses {
                left: lp_count,
                right: rp_count,
            });
        }

        output_stack.push(stack);
        
        let mut encoded = self.encode_expression_stack(&output_stack)?;
        
        if let Some(last_expr) = output_stack.last() {
            if let Some(last_token) = last_expr.last() {
                if !ParserState::is_operator(last_token) {
                    encoded.push("0000".to_string());
                }
            }
        }
        
        append_hex_string_array(&mut self.output_buffer, encoded)?;
        Ok(())
    }

    fn handle_function_def(&mut self, line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        if self.defining_function {
            return Err(CompilerError::SyntaxError {
                line: self.current_line_index,
                message: "Nested functions not allowed".to_string(),
            });
        }
        self.defining_function = true;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 2 {
            let func_name = parts[1].trim_end_matches("()");
            let addr = self.output_buffer.len();
            self.state.create_label(&format!("function_{}", func_name), addr)?;
            self.handle_if("IF FALSE THEN", false)?;
        }
        Ok(())
    }

    fn handle_end_function(&mut self, _line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        if !self.return_defined {
            self.handle_return("RETURN 0")?;
        }
        self.return_defined = false;
        self.defining_function = false;
        self.handle_end_if("END_IF", true)
    }

    fn handle_function_call(&mut self, line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        let func_name = line.trim().trim_end_matches("()");
        let label = format!("function_{}", func_name);
        
        if let Some(&addr) = self.state.label_map.get(&label) {
            let adjusted_addr = addr + 6;
            let hex_addr = dec_to_hex(adjusted_addr / 2);
            append_hex_string_array(&mut self.output_buffer, vec!["F7".to_string(), "F7".to_string(), hex_addr])?;
        } else {
            append_hex_string_array(&mut self.output_buffer, vec!["F7".to_string(), "F7".to_string(), "XX".to_string(), "XX".to_string()])?;
            self.goto_awaiting_label
                .entry(label)
                .or_insert_with(Vec::new)
                .push(self.output_buffer.len() - 2);
        }
        Ok(())
    }

    fn handle_return(&mut self, line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        self.return_defined = true;
        let converted = line.replace("RETURN", "VAR $_f_ret =");
        self.handle_assignment(&converted)?;
        append_hex_string_array(&mut self.output_buffer, vec!["FD".to_string(), "FD".to_string()])?;
        Ok(())
    }

    fn handle_enter(&mut self, _line: &str) -> CompilerResult<()> {
        self.inject_character("ENTER")
    }

    fn handle_repeat(&mut self, line: &str) -> CompilerResult<()> {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() < 3 {
            return Ok(());
        }
        
        if let Ok(count) = parts[1].parse::<usize>() {
            let remainder = parts[2..].join(" ");
            for _ in 0..count {
                self.process_line(&remainder)?;
            }
        }
        Ok(())
    }

    fn handle_hold(&mut self, line: &str) -> CompilerResult<()> {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 2 {
            append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0xF8FF)])?;
            for key in &parts[1..] {
                self.inject_character(key)?;
            }
        }
        Ok(())
    }

    fn handle_release(&mut self, line: &str) -> CompilerResult<()> {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 2 {
            append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0xE8EE)])?;
            for key in &parts[1..] {
                self.inject_character(key)?;
            }
        } else {
            append_hex_string_array(&mut self.output_buffer, vec!["00".to_string(), "00".to_string()])?;
        }
        Ok(())
    }

    fn handle_inject(&mut self, line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 2 {
            let param = parts[1].trim_start_matches("0x").trim_start_matches("0X");
            append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0xE8E9)])?;
            append_hex_string_array(&mut self.output_buffer, vec![param.to_string()])?;
        }
        Ok(())
    }

    fn handle_inject_mod(&mut self, line: &str) -> CompilerResult<()> {
        if !self.modifier_queue.is_empty() {
            append_hex_string_array(&mut self.output_buffer, self.modifier_queue.clone())?;
            self.modifier_queue.clear();
        }
        append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0xE9E6)])?;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 2 {
            let param = parts[1..].join(" ");
            let mod_bytes = self.keyboard_layout.get_bytes_for_key(&param);
            let key_bytes: Option<Vec<String>> = None;
            
            let mod_hex = if let Some(mod_codes) = mod_bytes.as_ref() {
                if mod_codes.len() >= 1 {
                    mod_codes[0].clone()
                } else {
                    "00".to_string()
                }
            } else {
                "00".to_string()
            };
            
            let key_hex = if let Some(key_codes) = key_bytes.as_ref() {
                if key_codes.len() >= 1 {
                    key_codes[0].clone()
                } else {
                    "00".to_string()
                }
            } else {
                "00".to_string()
            };
            
            append_hex_string_array(&mut self.output_buffer, vec![key_hex, mod_hex])?;
        }
        Ok(())
    }

    fn handle_inject_var(&mut self, line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 2 {
            let var_name = parts[1];
            append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0xE9E9)])?;

            if let Some(addr) = self.state.get_var_address(var_name) {
                let hex_addr = dec_to_hex(addr);
                append_hex_string_array(&mut self.output_buffer, vec![hex_addr])?;
            } else if ParserState::is_reserved_var(var_name) {
                if let Some(&val) = RESERVED_VARIABLES.get(var_name) {
                    append_hex_string_array(&mut self.output_buffer, vec![hex_encode(val)])?;
                } else {
                    return Err(CompilerError::UnknownVariable(var_name.to_string()));
                }
            } else {
                return Err(CompilerError::UnknownVariable(var_name.to_string()));
            }
        }
        Ok(())
    }

    fn handle_keycode(&mut self, line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 2 {
            let param = parts[1].trim_start_matches("0x").trim_start_matches("0X");
            append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0xE8E9)])?;
            append_hex_string_array(&mut self.output_buffer, vec![param.to_string()])?;
        }
        Ok(())
    }

    fn handle_exfil_var(&mut self, line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 2 {
            let var_name = parts[1];
            append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0xE9F6)])?;
            if let Some(addr) = self.state.get_var_address(var_name) {
                append_hex_string_array(&mut self.output_buffer, vec![dec_to_hex(addr)])?;
            } else {
                return Err(CompilerError::UnknownVariable(var_name.to_string()));
            }
        }
        Ok(())
    }

    fn handle_modifier(&mut self, line: &str) -> CompilerResult<()> {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }
        
        let modifier = parts[0];
        let key = if parts.len() > 1 { parts[1] } else { "" };
        
        let mod_bytes = self.keyboard_layout.get_bytes_for_key(modifier);
        let key_bytes = if !key.is_empty() {
            self.keyboard_layout.get_bytes_for_key(key)
        } else {
            None
        };
        
        let combined_value = if let (Some(mod_codes), Some(key_codes)) = (mod_bytes.as_ref(), key_bytes.as_ref()) {
            if key_codes.len() >= 1 && mod_codes.len() >= 1 {
                format!("{}{}", key_codes[0], mod_codes[0])
            } else {
                "0000".to_string()
            }
        } else if let Some(mod_codes) = mod_bytes.as_ref() {
            if mod_codes.len() >= 1 {
                format!("{}00", mod_codes[0])
            } else {
                "0000".to_string()
            }
        } else {
            "0000".to_string()
        };
        
        let var_name = format!("__MOD_{}_{}", modifier, key);
        let addr = if let Some(existing_addr) = self.state.get_var_address(&var_name) {
            existing_addr
        } else {
            let addr = self.state.allocate_var(&var_name);
            self.state.assign_value(&var_name, combined_value)?;
            addr
        };
        
        self.modifier_queue.push(dec_to_hex(addr));
        Ok(())
    }

    fn handle_button_def(&mut self, _line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        self.defining_button += 1;
        self.button_block_stack.push(self.defining_button);
        append_hex_string_array(
            &mut self.output_buffer,
            vec![
                "EA".to_string(),
                "EE".to_string(),
                dec_to_hex(self.defining_button as usize),
            ],
        )?;
        Ok(())
    }

    fn handle_end_button_def(&mut self, _line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        let addr = self
            .button_block_stack
            .pop()
            .unwrap_or(self.defining_button.max(1));
        append_hex_string_array(
            &mut self.output_buffer,
            vec!["EB".to_string(), "F4".to_string(), dec_to_hex(addr as usize)],
        )?;
        self.defining_button = self.defining_button.saturating_sub(1);
        Ok(())
    }

    fn handle_attackmode(&mut self, line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        
        if parts.len() < 2 {
            return Ok(());
        }
        
        let mut encoded_line: Vec<String> = Vec::new();
        let mut mode_hex = 0x0000u16;
        let mut mode_defined = false;
        let mut vid_defined = false;
        let mut pid_defined = false;
        let mut man_defined = false;
        let mut prod_defined = false;
        let mut serial_defined = false;

        let mut i = 0;
        while i < parts.len() {
            let word = parts[i];

            if word.eq_ignore_ascii_case("ATTACKMODE") {
                i += 1;
                if i >= parts.len() {
                    break;
                }
                let mode = parts[i];
                if mode.eq_ignore_ascii_case("OFF") {
                    mode_hex = 0xF0F0;
                    mode_defined = true;
                    encoded_line.push(hex_encode(mode_hex));
                    break;
                }
                if mode.eq_ignore_ascii_case("HID") {
                    mode_defined = true;
                    if i + 1 < parts.len() && parts[i + 1].eq_ignore_ascii_case("STORAGE") {
                        mode_hex = 0xF3F3;
                        encoded_line.push(hex_encode(mode_hex));
                        i += 1;
                    } else {
                        mode_hex = 0xF1F1;
                        encoded_line.push(hex_encode(mode_hex));
                    }
                    i += 1;
                    continue;
                }
                if mode.eq_ignore_ascii_case("STORAGE") {
                    mode_defined = true;
                    if i + 1 < parts.len() && parts[i + 1].eq_ignore_ascii_case("HID") {
                        mode_hex = 0xF3F3;
                        encoded_line.push(hex_encode(mode_hex));
                        i += 1;
                    } else {
                        mode_hex = 0xF2F2;
                        encoded_line.push(hex_encode(mode_hex));
                    }
                    i += 1;
                    continue;
                }
            } else if word.eq_ignore_ascii_case("VID_RANDOM") {
                encoded_line.push(hex_encode(0xF5F5));
                encoded_line.push(hex_encode(0x42F3));
                vid_defined = true;
            } else if word.to_ascii_uppercase().starts_with("VID_$") {
                encoded_line.push(hex_encode(0xF5F5));
                vid_defined = true;
                let var = &word[4..];
                if let Some(addr) = self.state.get_var_address(var) {
                    encoded_line.push(dec_to_hex(addr));
                } else {
                    return Err(CompilerError::UnknownVariable(var.to_string()));
                }
            } else if word.to_ascii_uppercase().starts_with("VID_") {
                let arg = &word[4..];
                if arg.is_empty() {
                    return Err(CompilerError::SyntaxError {
                        line: self.current_line_index,
                        message: "Invalid VID".to_string(),
                    });
                }
                let formatted = format_hex(arg);
                let addr = if let Some(existing) = self.state.get_var_address(&formatted) {
                    existing
                } else {
                    let a = self.state.allocate_var(&formatted);
                    self.state.assign_value(&formatted, formatted.clone())?;
                    a
                };
                encoded_line.push(hex_encode(0xF5F5));
                encoded_line.push(dec_to_hex(addr));
                vid_defined = true;
            } else if word.eq_ignore_ascii_case("PID_RANDOM") {
                encoded_line.push(hex_encode(0xF6F6));
                encoded_line.push(hex_encode(0x42F3));
                pid_defined = true;
            } else if word.to_ascii_uppercase().starts_with("PID_$") {
                encoded_line.push(hex_encode(0xF6F6));
                pid_defined = true;
                let var = &word[4..];
                if let Some(addr) = self.state.get_var_address(var) {
                    encoded_line.push(dec_to_hex(addr));
                } else {
                    return Err(CompilerError::UnknownVariable(var.to_string()));
                }
            } else if word.to_ascii_uppercase().starts_with("PID_") {
                let arg = &word[4..];
                if arg.is_empty() {
                    return Err(CompilerError::SyntaxError {
                        line: self.current_line_index,
                        message: "Invalid PID".to_string(),
                    });
                }
                let formatted = format_hex(arg);
                let addr = if let Some(existing) = self.state.get_var_address(&formatted) {
                    existing
                } else {
                    let a = self.state.allocate_var(&formatted);
                    self.state.assign_value(&formatted, formatted.clone())?;
                    a
                };
                encoded_line.push(hex_encode(0xF6F6));
                encoded_line.push(dec_to_hex(addr));
                pid_defined = true;
            } else if word.eq_ignore_ascii_case("MAN_RANDOM") {
                encoded_line.push(hex_encode(0xF9F9));
                for _ in 0..12 {
                    encoded_line.push(hex_encode(0x42F5));
                }
                encoded_line.push(hex_encode(0xF9F9));
                man_defined = true;
            } else if word.to_ascii_uppercase().starts_with("MAN_") {
                let arg = &word[4..];
                if arg.is_empty() || arg.len() > 32 {
                    return Err(CompilerError::SyntaxError {
                        line: self.current_line_index,
                        message: "Invalid Manufacturer".to_string(),
                    });
                }
                encoded_line.push(hex_encode(0xF9F9));
                for ch in arg.chars() {
                    let v = hex_encode(ch as u16);
                    let name = format!("__CHAR_{}", v);
                    let addr = if let Some(existing) = self.state.get_var_address(&name) {
                        existing
                    } else {
                        let a = self.state.allocate_var(&name);
                        self.state.assign_value(&name, v)?;
                        a
                    };
                    encoded_line.push(dec_to_hex(addr));
                }
                encoded_line.push(hex_encode(0xF9F9));
                man_defined = true;
            } else if word.eq_ignore_ascii_case("PROD_RANDOM") {
                encoded_line.push(hex_encode(0xFAFA));
                for _ in 0..12 {
                    encoded_line.push(hex_encode(0x42F5));
                }
                encoded_line.push(hex_encode(0xFAFA));
                prod_defined = true;
            } else if word.to_ascii_uppercase().starts_with("PROD_") {
                let arg = &word[5..];
                if arg.is_empty() || arg.len() > 32 {
                    return Err(CompilerError::SyntaxError {
                        line: self.current_line_index,
                        message: "Invalid Product".to_string(),
                    });
                }
                encoded_line.push(hex_encode(0xFAFA));
                for ch in arg.chars() {
                    let v = hex_encode(ch as u16);
                    let name = format!("__CHAR_{}", v);
                    let addr = if let Some(existing) = self.state.get_var_address(&name) {
                        existing
                    } else {
                        let a = self.state.allocate_var(&name);
                        self.state.assign_value(&name, v)?;
                        a
                    };
                    encoded_line.push(dec_to_hex(addr));
                }
                encoded_line.push(hex_encode(0xFAFA));
                prod_defined = true;
            } else if word.eq_ignore_ascii_case("SERIAL_RANDOM") {
                encoded_line.push(hex_encode(0xFBFB));
                for _ in 0..12 {
                    encoded_line.push(hex_encode(0x42F7));
                }
                encoded_line.push(hex_encode(0xFBFB));
                serial_defined = true;
            } else if word.to_ascii_uppercase().starts_with("SERIAL_") {
                let arg = &word[7..];
                if arg.is_empty() || arg.len() > 12 {
                    return Err(CompilerError::SyntaxError {
                        line: self.current_line_index,
                        message: "Invalid Serial".to_string(),
                    });
                }
                if !arg.chars().all(|c| c.is_ascii_digit()) {
                    return Err(CompilerError::SyntaxError {
                        line: self.current_line_index,
                        message: "Invalid Serial".to_string(),
                    });
                }
                encoded_line.push(hex_encode(0xFBFB));
                for ch in arg.chars() {
                    let v = hex_encode(ch as u16);
                    let name = format!("__CHAR_{}", v);
                    let addr = if let Some(existing) = self.state.get_var_address(&name) {
                        existing
                    } else {
                        let a = self.state.allocate_var(&name);
                        self.state.assign_value(&name, v)?;
                        a
                    };
                    encoded_line.push(dec_to_hex(addr));
                }
                encoded_line.push(hex_encode(0xFBFB));
                serial_defined = true;
            }

            i += 1;
        }
        
        if mode_defined {
            encoded_line.push(hex_encode(mode_hex));
        }

        if vid_defined || pid_defined {
            if !(vid_defined && pid_defined) {
                return Err(CompilerError::SyntaxError {
                    line: self.current_line_index,
                    message: "VID + PID must both be defined".to_string(),
                });
            }
        }
        if man_defined || prod_defined || serial_defined {
            if !(man_defined && prod_defined && serial_defined) {
                return Err(CompilerError::SyntaxError {
                    line: self.current_line_index,
                    message: "MAN + PROD + SERIAL must all be defined".to_string(),
                });
            }
        }
        
        append_hex_string_array(&mut self.output_buffer, encoded_line)?;
        Ok(())
    }

    fn handle_random_lowercase(&mut self, _line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        self.state.requires_lang_pack = true;
        append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0xE9E9)])?;
        append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0x42FA)])?;
        Ok(())
    }

    fn handle_random_uppercase(&mut self, _line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        self.state.requires_lang_pack = true;
        append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0xE9E9)])?;
        append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0x42FB)])?;
        Ok(())
    }

    fn handle_random_number(&mut self, _line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        self.state.requires_lang_pack = true;
        append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0xE9E9)])?;
        append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0x42FC)])?;
        Ok(())
    }

    fn handle_random_letter(&mut self, _line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        self.state.requires_lang_pack = true;
        append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0xE9E9)])?;
        append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0x42FF)])?;
        Ok(())
    }

    fn handle_random_special(&mut self, _line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        self.state.requires_lang_pack = true;
        append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0xE9E9)])?;
        append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0x42FD)])?;
        Ok(())
    }

    fn handle_random_char(&mut self, _line: &str) -> CompilerResult<()> {
        self.mark_ds3();
        self.state.requires_lang_pack = true;
        append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0xE9E9)])?;
        append_hex_string_array(&mut self.output_buffer, vec![hex_encode(0x42FE)])?;
        Ok(())
    }

    fn handle_unknown(&mut self, line: &str) -> CompilerResult<()> {
        let trimmed = line.trim();

        if trimmed.eq_ignore_ascii_case("RESTORE_HOST_KEYBOARD_LOCK_STATE") {
            if let Some(&builtin) = BUILTINS_MAP.get("RESTORE_HOST_KEYBOARD_LOCK_STATE") {
                append_hex_string_array(&mut self.output_buffer, vec![hex_encode(builtin)])?;
                append_hex_string_array(
                    &mut self.output_buffer,
                    vec!["0000".to_string(), "0000".to_string(), "0000".to_string()],
                )?;
                return Ok(());
            }
        }
        
        if let Some(&builtin) = BUILTINS_MAP.get(trimmed) {
            append_hex_string_array(&mut self.output_buffer, vec![hex_encode(builtin)])?;
            return Ok(());
        }
        
        let parts: Vec<&str> = trimmed.split_whitespace().filter(|s| *s != "-").collect();
        
        if parts.len() > 1 {
            let mut key_sum = 0u8;
            let mut mod_sum = 0u8;
            
            for part in &parts {
                if let Some(codes) = self.keyboard_layout.get_bytes_for_key(part) {
                    if codes.len() >= 3 {
                        let modifier = u8::from_str_radix(&codes[0], 16).unwrap_or(0);
                        let keycode = u8::from_str_radix(&codes[2], 16).unwrap_or(0);
                        
                        if keycode != 0 {
                            key_sum += keycode;
                            if modifier != 0 {
                                mod_sum += modifier;
                            }
                        } else {
                            mod_sum += modifier;
                        }
                    } else if codes.len() >= 1 {
                        let byte_val = u8::from_str_radix(&codes[0], 16).unwrap_or(0);
                        key_sum += byte_val;
                    }
                }
            }
            
            append_hex_string_array(&mut self.output_buffer, vec![format!("{:02x}", key_sum), format!("{:02x}", mod_sum)])?;
            return Ok(());
        }
        
        self.inject_character(trimmed)?;
        Ok(())
    }

    fn check_delay(&self) -> Option<Vec<String>> {
        if !self.delay_override && self.default_delay > 0 {
            Some(build_delay_bytes(self.default_delay))
        } else {
            None
        }
    }

    fn post_process(&mut self) -> CompilerResult<()> {
        if !self.state.block_stack.is_empty() {
            return Err(CompilerError::MissingEnd("IF/WHILE".to_string()));
        }
        
        if self.inside_string_block {
            return Err(CompilerError::MissingEnd("STRING".to_string()));
        }
        
        if self.inside_stringln_block {
            return Err(CompilerError::MissingEnd("STRINGLN".to_string()));
        }
        
        if self.defining_button > 0 {
            return Err(CompilerError::MissingEnd("BUTTON".to_string()));
        }
        
        Ok(())
    }

    fn finalize(&mut self) -> CompilerResult<Vec<u8>> {
        let mut added_bytes = 0;
        
        if self.state.requires_lang_pack {
            self.generate_lang_pack()?;
        }
        
        if self.state.var_values.len() > 1 {
            let mut temp_buffer = Vec::new();
            temp_buffer.push("E8E8".to_string());
            added_bytes += 2;
            
            for i in 1..self.state.var_values.len() {
                let val = &self.state.var_values[i];
                let formatted = if val.len() == 4 {
                    val.clone()
                } else if val.len() == 2 {
                    format!("00{}", val)
                } else if val.len() == 1 {
                    format!("000{}", val)
                } else if val.len() > 4 {
                    val[0..4].to_string()
                } else {
                    format!("{:0<4}", val)
                };
                
                temp_buffer.push(formatted);
                added_bytes += 2;
            }
            
            temp_buffer.push("E8E8".to_string());
            added_bytes += 2;
            
            temp_buffer.extend(self.output_buffer.clone());
            self.output_buffer = temp_buffer;
        }
        
        let mut ui16_buffer = Vec::new();
        let joined = self.output_buffer.join("");
        for i in (0..joined.len()).step_by(4) {
            if i + 4 <= joined.len() {
                ui16_buffer.push(joined[i..i + 4].to_string());
            } else if i + 2 <= joined.len() {
                ui16_buffer.push(format!("{:0<4}", &joined[i..]));
            }
        }
        
        for i in 0..ui16_buffer.len() {
            if ui16_buffer[i] == "F8F8" || ui16_buffer[i] == "F7F7" {
                if i + 1 < ui16_buffer.len() {
                    if let Ok(current_addr) = usize::from_str_radix(&ui16_buffer[i + 1], 16) {
                        let shifted_addr = current_addr + (added_bytes / 2);
                        ui16_buffer[i + 1] = hex_encode(shifted_addr as u16);
                    }
                }
            }
        }
        
        let final_hex = ui16_buffer.join("");
        Ok(hex_to_byte_array(&final_hex))
    }

    fn generate_lang_pack(&mut self) -> CompilerResult<()> {
        let shift_codes = self.keyboard_layout.get_bytes_for_key("SHIFT")
            .unwrap_or_else(|| vec!["02".to_string(), "00".to_string(), "00".to_string()]);
        let shift = &shift_codes[0];
        
        let to_pack = vec![
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m",
            "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "0"
        ];
        
        let _marker_addr = self.state.allocate_var("LANG_MARKER");
        self.state.assign_value("LANG_MARKER", hex_encode(0xEEEE))?;
        
        for ch in to_pack {
            if let Some(codes) = self.keyboard_layout.get_bytes_for_key(ch) {
                if codes.len() >= 3 {
                    let keycode = &codes[2];
                    if let (Ok(shift_val), Ok(key_val)) = (u8::from_str_radix(shift, 16), u8::from_str_radix(keycode, 16)) {
                        let combined_u16 = ((shift_val as u16) << 8) | (key_val as u16);
                        let var_name = format!("LANG_{}", ch);
                        let _addr = self.state.allocate_var(&var_name);
                        self.state.assign_value(&var_name, hex_encode(combined_u16))?;
                    }
                }
            }
        }
        
        Ok(())
    }
}
