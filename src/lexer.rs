use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DEFINE_REGEX: Regex = Regex::new(r"^\s*DEFINE .*").unwrap();
    pub static ref IFDEF_REGEX: Regex = Regex::new(r"^\s*IF_DEFINED_TRUE .*").unwrap();
    pub static ref IFNOTDEF_REGEX: Regex = Regex::new(r"^\s*IF_NOT_DEFINED_TRUE .*").unwrap();
    pub static ref ENDIFDEF_REGEX: Regex = Regex::new(r"^\s*END_IF_DEFINED\s*$").unwrap();
    pub static ref ELSEDEF_REGEX: Regex = Regex::new(r"^\s*ELSE_DEFINED\s*$").unwrap();
    
    pub static ref STRINGLN_REGEX: Regex = Regex::new(r"^\s*STRINGLN\s+.*$").unwrap();
    pub static ref STRING_REGEX: Regex = Regex::new(r"^\s*STRING\s+.*$").unwrap();
    pub static ref STRING_INLINE_REGEX: Regex = Regex::new(r"^\s*STRING\s+.*\s+END_STRING$").unwrap();
    pub static ref STRINGLN_INLINE_REGEX: Regex = Regex::new(r"^\s*STRINGLN\s+.*\s+END_STRING$").unwrap();
    pub static ref END_STRING_REGEX: Regex = Regex::new(r"^\s*END_STRING(LN)?\s*$").unwrap();
    pub static ref INLINE_STRINGLN_REGEX: Regex = Regex::new(r"^\s*STRINGLN(_(POWERSHELL|BASH|BATCH|PYTHON|HTML|RUBY|JAVASCRIPT))?\s+.*\s+END_STRINGLN$").unwrap();
    pub static ref INLINE_STRING_REGEX: Regex = Regex::new(r"^\s*STRING(_(POWERSHELL|BASH|BATCH|PYTHON|HTML|RUBY|JAVASCRIPT))?\s+.*\s+END_STRING$").unwrap();
    pub static ref STRINGLN_BLOCK_REGEX: Regex = Regex::new(r"^\s*STRINGLN(_(POWERSHELL|BASH|BATCH|PYTHON|HTML|RUBY|JAVASCRIPT))?$").unwrap();
    pub static ref STRING_BLOCK_REGEX: Regex = Regex::new(r"^\s*STRING(_(POWERSHELL|BASH|BATCH|PYTHON|HTML|RUBY|JAVASCRIPT))?$").unwrap();
    
    pub static ref REM_REGEX: Regex = Regex::new(r"^\s*REM.*$").unwrap();
    pub static ref REM_BLOCK_REGEX: Regex = Regex::new(r"^\s*REM_BLOCK.*$").unwrap();
    pub static ref END_REM_BLOCK_REGEX: Regex = Regex::new(r"^\s*END_REM.*$").unwrap();
    pub static ref COMMENT_REGEX: Regex = Regex::new(r"^// .*$").unwrap();
    
    pub static ref PREPROCESSOR_DISABLED: Regex = Regex::new(r"^\s*PREPROCESSOR_DISABLED.*$").unwrap();
    
    pub static ref INJECT_VAR_REGEX: Regex = Regex::new(r"^\s*INJECT_VAR \$.*$").unwrap();
    pub static ref DELAY_VAR_REGEX: Regex = Regex::new(r"^\s*DELAY \$.*$").unwrap();
    pub static ref DELAY_REGEX: Regex = Regex::new(r"^\s*DELAY .*").unwrap();
    
    pub static ref HOLD_REGEX: Regex = Regex::new(r"^\s*HOLD .*$").unwrap();
    pub static ref RELEASE_REGEX: Regex = Regex::new(r"^\s*RELEASE .*$").unwrap();
    pub static ref REPEAT_REGEX: Regex = Regex::new(r"^\s*REPEAT (?:[2-9]|\d\d\d*).*$").unwrap();
    pub static ref ENTER_REGEX: Regex = Regex::new(r"^\s*ENTER.*$").unwrap();
    
    pub static ref DEFAULTDELAY_REGEX: Regex = Regex::new(r"^\s*DEFAULTDELAY (?:[2-9]|\d\d\d*)$").unwrap();
    pub static ref DEFAULT_DELAY_REGEX: Regex = Regex::new(r"^\s*DEFAULT_DELAY (?:[2-9]|\d\d\d*)$").unwrap();
    pub static ref STRINGDELAY_REGEX: Regex = Regex::new(r"^\s*STRINGDELAY (?:[2-9]|\d\d\d*).*$").unwrap();
    pub static ref STRING_DELAY_REGEX: Regex = Regex::new(r"^\s*STRING_DELAY (?:[2-9]|\d\d\d*).*$").unwrap();
    
    pub static ref IF_REGEX: Regex = Regex::new(r"^\s*IF .*").unwrap();
    pub static ref END_B_REGEX: Regex = Regex::new(r"^}.*").unwrap();
    pub static ref END_IF_REGEX: Regex = Regex::new(r"^\s*END_IF.*").unwrap();
    pub static ref ELSE_IF_REGEX: Regex = Regex::new(r"^\s*ELSE IF .*").unwrap();
    pub static ref ELSE_REGEX: Regex = Regex::new(r"^\s*ELSE.*").unwrap();
    
    pub static ref WHILE_REGEX: Regex = Regex::new(r"^\s*WHILE .*").unwrap();
    pub static ref END_WHILE_REGEX: Regex = Regex::new(r"^\s*END_WHILE.*").unwrap();
    
    pub static ref STAGE_DEF_REGEX: Regex = Regex::new(r"^\s*STAGE .*$").unwrap();
    pub static ref END_STAGE_REGEX: Regex = Regex::new(r"^\s*END_STAGE.*").unwrap();
    
    pub static ref EXTENSION_DEF_REGEX: Regex = Regex::new(r"^\s*EXTENSION .*$").unwrap();
    pub static ref END_EXTENSION_REGEX: Regex = Regex::new(r"^\s*END_EXTENSION.*").unwrap();
    
    pub static ref FUNCTION_DEF_REGEX: Regex = Regex::new(r"^\s*FUNCTION .*\(\).*").unwrap();
    pub static ref END_FUNCTION_REGEX: Regex = Regex::new(r"^\s*END_FUNCTION.*").unwrap();
    
    pub static ref ATTACKMODE_REGEX: Regex = Regex::new(r"^\s*ATTACKMODE .*").unwrap();
    pub static ref FUNCTION_CALL_RETURN_REGEX: Regex = Regex::new(r"^\s*RETURN .*").unwrap();
    
    pub static ref ASSIGNMENT_REGEX: Regex = Regex::new(r"\$.*= .*").unwrap();
    pub static ref DECLARATION_REGEX: Regex = Regex::new(r"^\s*VAR \$.*= .*").unwrap();
    pub static ref FUNCTION_CALL_NO_RETURN_REGEX: Regex = Regex::new(r"^\s*[\w]+\(\)").unwrap();
    
    pub static ref BUTTON_DEF_REGEX: Regex = Regex::new(r"^\s*BUTTON_DEF.*").unwrap();
    pub static ref END_BUTTON_DEF_REGEX: Regex = Regex::new(r"^\s*END_BUTTON.*").unwrap();
    
    pub static ref DEBUGGER_BREAKPOINT_REGEX: Regex = Regex::new(r"^\s*DEBUGGER_BREAKPOINT.*").unwrap();
    pub static ref INJECT_BREAKPOINT_LINENUMBER_REGEX: Regex = Regex::new(r"^\s*INJECT_BREAKPOINT_LINE_NUMBER.*").unwrap();
    
    pub static ref RANDOM_LOWERCASE_LETTER_REGEX: Regex = Regex::new(r"^\s*RANDOM_LOWERCASE_LETTER.*").unwrap();
    pub static ref RANDOM_UPPERCASE_LETTER_REGEX: Regex = Regex::new(r"^\s*RANDOM_UPPERCASE_LETTER.*").unwrap();
    pub static ref RANDOM_NUMBER_REGEX: Regex = Regex::new(r"^\s*RANDOM_NUMBER.*").unwrap();
    pub static ref RANDOM_LETTER_REGEX: Regex = Regex::new(r"^\s*RANDOM_LETTER.*").unwrap();
    pub static ref RANDOM_SPECIAL_REGEX: Regex = Regex::new(r"^\s*RANDOM_SPECIAL.*").unwrap();
    pub static ref RANDOM_CHAR_REGEX: Regex = Regex::new(r"^\s*RANDOM_CHAR.*").unwrap();
    
    pub static ref RESTORE_HOST_KEYBOARD_LOCK_STATE_REGEX: Regex = Regex::new(r"^\s*RESTORE_HOST_KEYBOARD_LOCK_STATE.*").unwrap();
    
    pub static ref INJECTMOD_REGEX: Regex = Regex::new(r"^\s*INJECT_MOD\s*$").unwrap();
    pub static ref INJECTMOD_PARAM_REGEX: Regex = Regex::new(r"^\s*INJECT_MOD .*$").unwrap();
    pub static ref INJECT_REGEX: Regex = Regex::new(r"^\s*INJECT .*").unwrap();
    pub static ref KEYCODE_REGEX: Regex = Regex::new(r"^\s*KEYCODE .*$").unwrap();
    
    pub static ref EXFIL_VAR_REGEX: Regex = Regex::new(r"^\s*EXFIL \$.*").unwrap();
    
    pub static ref MOD_KEY_DOWN_REGEX: Regex = Regex::new(r"^\s*MOD_KEY_DOWN .*").unwrap();
    pub static ref MOD_KEY_UP_REGEX: Regex = Regex::new(r"^\s*MOD_KEY_UP .*").unwrap();
    pub static ref MOD_UP_REGEX: Regex = Regex::new(r"^\s*MOD_UP .*").unwrap();
    pub static ref KEY_UP_REGEX: Regex = Regex::new(r"^\s*KEY_UP .*").unwrap();
    pub static ref KEY_DOWN_REGEX: Regex = Regex::new(r"^\s*KEY_DOWN .*").unwrap();
    pub static ref MOD_DOWN_REGEX: Regex = Regex::new(r"^\s*MOD_DOWN .*").unwrap();
    
    pub static ref US_MODIFIERS_REGEX: Regex = Regex::new(r"^\s*(CTRL|CONTROL|SHIFT|ALT|GUI|WINDOWS|CTRL-ALT|COMMAND)$").unwrap();
    
    pub static ref VAR_REGEX: Regex = Regex::new(r"\$.*").unwrap();
    pub static ref FUNCTION_CALL_REGEX: Regex = Regex::new(r"[\w]+\(\)").unwrap();
    pub static ref HEX_REGEX: Regex = Regex::new(r"0x.*").unwrap();
    pub static ref DECIMAL_REGEX: Regex = Regex::new(r"(?:[0-9]|\d\d\d*)").unwrap();
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Define,
    IfDefined,
    IfNotDefined,
    EndIfDefined,
    ElseDefined,
    
    String,
    StringLn,
    StringBlock,
    StringLnBlock,
    EndString,
    
    Rem,
    RemBlock,
    EndRemBlock,
    Comment,
    
    Delay,
    DelayVar,
    DefaultDelay,
    StringDelay,
    
    Hold,
    Release,
    Repeat,
    Enter,
    
    If,
    ElseIf,
    Else,
    EndIf,
    
    While,
    EndWhile,
    
    Function,
    EndFunction,
    FunctionCall,
    Return,
    
    Assignment,
    Declaration,
    
    ButtonDef,
    EndButtonDef,
    
    Attackmode,
    
    InjectVar,
    Inject,
    InjectMod,
    Keycode,
    
    ExfilVar,
    
    ModKeyDown,
    ModKeyUp,
    KeyDown,
    KeyUp,
    ModDown,
    ModUp,
    
    RandomLowercaseLetter,
    RandomUppercaseLetter,
    RandomNumber,
    RandomLetter,
    RandomSpecial,
    RandomChar,
    
    Modifier,
    
    Stage,
    EndStage,
    Extension,
    EndExtension,
    
    Breakpoint,
    InjectBreakpointLineNumber,
    
    PreprocessorDisabled,
    Unknown,
}

pub fn tokenize_line(line: &str) -> TokenType {
    let trimmed = line.trim();
    
    if PREPROCESSOR_DISABLED.is_match(trimmed) { return TokenType::PreprocessorDisabled; }
    if REM_BLOCK_REGEX.is_match(trimmed) { return TokenType::RemBlock; }
    if END_REM_BLOCK_REGEX.is_match(trimmed) { return TokenType::EndRemBlock; }
    if REM_REGEX.is_match(trimmed) { return TokenType::Rem; }
    if COMMENT_REGEX.is_match(trimmed) { return TokenType::Comment; }
    
    if DEFINE_REGEX.is_match(trimmed) { return TokenType::Define; }
    if IFDEF_REGEX.is_match(trimmed) { return TokenType::IfDefined; }
    if IFNOTDEF_REGEX.is_match(trimmed) { return TokenType::IfNotDefined; }
    if ELSEDEF_REGEX.is_match(trimmed) { return TokenType::ElseDefined; }
    if ENDIFDEF_REGEX.is_match(trimmed) { return TokenType::EndIfDefined; }
    
    if INJECT_VAR_REGEX.is_match(trimmed) { return TokenType::InjectVar; }
    if DELAY_VAR_REGEX.is_match(trimmed) { return TokenType::DelayVar; }
    if DELAY_REGEX.is_match(trimmed) { return TokenType::Delay; }
    
    if END_STRING_REGEX.is_match(trimmed) { return TokenType::EndString; }
    if STRINGLN_INLINE_REGEX.is_match(trimmed) { return TokenType::StringLn; }
    if STRING_INLINE_REGEX.is_match(trimmed) { return TokenType::String; }
    if INLINE_STRINGLN_REGEX.is_match(trimmed) { return TokenType::StringLn; }
    if INLINE_STRING_REGEX.is_match(trimmed) { return TokenType::String; }
    if STRINGLN_BLOCK_REGEX.is_match(trimmed) { return TokenType::StringLnBlock; }
    if STRING_BLOCK_REGEX.is_match(trimmed) { return TokenType::StringBlock; }
    if STRINGLN_REGEX.is_match(trimmed) { return TokenType::StringLn; }
    if STRING_REGEX.is_match(trimmed) { return TokenType::String; }
    
    if HOLD_REGEX.is_match(trimmed) { return TokenType::Hold; }
    if RELEASE_REGEX.is_match(trimmed) { return TokenType::Release; }
    if REPEAT_REGEX.is_match(trimmed) { return TokenType::Repeat; }
    if ENTER_REGEX.is_match(trimmed) { return TokenType::Enter; }
    
    if DEFAULTDELAY_REGEX.is_match(trimmed) { return TokenType::DefaultDelay; }
    if DEFAULT_DELAY_REGEX.is_match(trimmed) { return TokenType::DefaultDelay; }
    if STRINGDELAY_REGEX.is_match(trimmed) { return TokenType::StringDelay; }
    if STRING_DELAY_REGEX.is_match(trimmed) { return TokenType::StringDelay; }
    
    if IF_REGEX.is_match(trimmed) { return TokenType::If; }
    if END_B_REGEX.is_match(trimmed) { return TokenType::EndIf; }
    if END_IF_REGEX.is_match(trimmed) { return TokenType::EndIf; }
    if ELSE_IF_REGEX.is_match(trimmed) { return TokenType::ElseIf; }
    if ELSE_REGEX.is_match(trimmed) { return TokenType::Else; }
    
    if WHILE_REGEX.is_match(trimmed) { return TokenType::While; }
    if END_WHILE_REGEX.is_match(trimmed) { return TokenType::EndWhile; }
    
    if STAGE_DEF_REGEX.is_match(trimmed) { return TokenType::Stage; }
    if END_STAGE_REGEX.is_match(trimmed) { return TokenType::EndStage; }
    
    if EXTENSION_DEF_REGEX.is_match(trimmed) { return TokenType::Extension; }
    if END_EXTENSION_REGEX.is_match(trimmed) { return TokenType::EndExtension; }
    
    if FUNCTION_DEF_REGEX.is_match(trimmed) { return TokenType::Function; }
    if END_FUNCTION_REGEX.is_match(trimmed) { return TokenType::EndFunction; }
    
    if ATTACKMODE_REGEX.is_match(trimmed) { return TokenType::Attackmode; }
    if FUNCTION_CALL_RETURN_REGEX.is_match(trimmed) { return TokenType::Return; }
    
    if ASSIGNMENT_REGEX.is_match(trimmed) { return TokenType::Assignment; }
    if DECLARATION_REGEX.is_match(trimmed) { return TokenType::Declaration; }
    if FUNCTION_CALL_NO_RETURN_REGEX.is_match(trimmed) { return TokenType::FunctionCall; }
    
    if BUTTON_DEF_REGEX.is_match(trimmed) { return TokenType::ButtonDef; }
    if END_BUTTON_DEF_REGEX.is_match(trimmed) { return TokenType::EndButtonDef; }
    
    if DEBUGGER_BREAKPOINT_REGEX.is_match(trimmed) { return TokenType::Breakpoint; }
    if INJECT_BREAKPOINT_LINENUMBER_REGEX.is_match(trimmed) { return TokenType::InjectBreakpointLineNumber; }
    
    if RANDOM_LOWERCASE_LETTER_REGEX.is_match(trimmed) { return TokenType::RandomLowercaseLetter; }
    if RANDOM_UPPERCASE_LETTER_REGEX.is_match(trimmed) { return TokenType::RandomUppercaseLetter; }
    if RANDOM_NUMBER_REGEX.is_match(trimmed) { return TokenType::RandomNumber; }
    if RANDOM_LETTER_REGEX.is_match(trimmed) { return TokenType::RandomLetter; }
    if RANDOM_SPECIAL_REGEX.is_match(trimmed) { return TokenType::RandomSpecial; }
    if RANDOM_CHAR_REGEX.is_match(trimmed) { return TokenType::RandomChar; }
    
    if RESTORE_HOST_KEYBOARD_LOCK_STATE_REGEX.is_match(trimmed) { return TokenType::Unknown; }
    
    if INJECTMOD_REGEX.is_match(trimmed) { return TokenType::InjectMod; }
    if INJECTMOD_PARAM_REGEX.is_match(trimmed) { return TokenType::InjectMod; }
    if INJECT_REGEX.is_match(trimmed) { return TokenType::Inject; }
    if KEYCODE_REGEX.is_match(trimmed) { return TokenType::Keycode; }
    
    if EXFIL_VAR_REGEX.is_match(trimmed) { return TokenType::ExfilVar; }
    
    if MOD_KEY_DOWN_REGEX.is_match(trimmed) { return TokenType::ModKeyDown; }
    if MOD_KEY_UP_REGEX.is_match(trimmed) { return TokenType::ModKeyUp; }
    if MOD_UP_REGEX.is_match(trimmed) { return TokenType::ModUp; }
    if KEY_UP_REGEX.is_match(trimmed) { return TokenType::KeyUp; }
    if KEY_DOWN_REGEX.is_match(trimmed) { return TokenType::KeyDown; }
    if MOD_DOWN_REGEX.is_match(trimmed) { return TokenType::ModDown; }
    
    if US_MODIFIERS_REGEX.is_match(trimmed) { return TokenType::Modifier; }
    
    TokenType::Unknown
}
