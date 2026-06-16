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

const LANGS: [&str; 7] = [
    "_POWERSHELL",
    "_BASH",
    "_BATCH",
    "_PYTHON",
    "_HTML",
    "_RUBY",
    "_JAVASCRIPT",
];

const US_MODIFIERS: [&str; 8] = [
    "CTRL", "CONTROL", "SHIFT", "ALT", "GUI", "WINDOWS", "CTRL-ALT", "COMMAND",
];

fn first_char_whitespace(s: &str) -> bool {
    s.chars().next().is_some_and(|c| c.is_whitespace())
}

fn repeat_number_prefix(s: &str) -> bool {
    let b = s.as_bytes();
    if b.is_empty() || !b[0].is_ascii_digit() {
        return false;
    }
    b[0] >= b'2' || (b.len() >= 2 && b[1].is_ascii_digit())
}

fn exact_number_2plus(s: &str) -> bool {
    let b = s.as_bytes();
    if b.is_empty() || !b.iter().all(|c| c.is_ascii_digit()) {
        return false;
    }
    b.len() >= 2 || b[0] >= b'2'
}

fn inline_body(t: &str, head_len: usize, tail: &str) -> bool {
    if !t.ends_with(tail) || t.len() < head_len + tail.len() {
        return false;
    }
    let mid = &t[head_len..t.len() - tail.len()];
    let mut count = 0;
    let mut first = None;
    let mut last = None;
    for c in mid.chars() {
        if first.is_none() {
            first = Some(c);
        }
        last = Some(c);
        count += 1;
    }
    count >= 2
        && first.is_some_and(|c| c.is_whitespace())
        && last.is_some_and(|c| c.is_whitespace())
}

fn matches_inline(t: &str, base: &str, tail: &str, allow_lang: bool) -> bool {
    if !t.starts_with(base) {
        return false;
    }
    if inline_body(t, base.len(), tail) {
        return true;
    }
    if allow_lang {
        let rest = &t[base.len()..];
        for lang in LANGS {
            if rest.starts_with(lang) && inline_body(t, base.len() + lang.len(), tail) {
                return true;
            }
        }
    }
    false
}

fn matches_block(t: &str, base: &str) -> bool {
    if t == base {
        return true;
    }
    if let Some(rest) = t.strip_prefix(base) {
        return LANGS.contains(&rest);
    }
    false
}

fn leading_word_chars(t: &str) -> usize {
    t.as_bytes()
        .iter()
        .take_while(|b| b.is_ascii_alphanumeric() || **b == b'_')
        .count()
}

pub fn is_preprocessor_disabled(t: &str) -> bool {
    t.starts_with("PREPROCESSOR_DISABLED")
}

pub fn is_rem(t: &str) -> bool {
    t.starts_with("REM")
}

pub fn is_end_rem_block(t: &str) -> bool {
    t.starts_with("END_REM")
}

pub fn is_define(t: &str) -> bool {
    t.starts_with("DEFINE ")
}

pub fn is_ifdef(t: &str) -> bool {
    t.starts_with("IF_DEFINED_TRUE ")
}

pub fn is_ifnotdef(t: &str) -> bool {
    t.starts_with("IF_NOT_DEFINED_TRUE ")
}

pub fn is_elsedef(t: &str) -> bool {
    t == "ELSE_DEFINED"
}

pub fn is_endifdef(t: &str) -> bool {
    t == "END_IF_DEFINED"
}

pub fn is_end_string(t: &str) -> bool {
    t == "END_STRING" || t == "END_STRINGLN"
}

pub fn is_string_block(t: &str) -> bool {
    matches_block(t, "STRING")
}

pub fn is_stringln_block(t: &str) -> bool {
    matches_block(t, "STRINGLN")
}

pub fn tokenize_line(line: &str) -> TokenType {
    tokenize_trimmed(line.trim())
}

pub fn tokenize_trimmed(t: &str) -> TokenType {
    if is_preprocessor_disabled(t) {
        return TokenType::PreprocessorDisabled;
    }
    if t.starts_with("REM_BLOCK") {
        return TokenType::RemBlock;
    }
    if is_end_rem_block(t) {
        return TokenType::EndRemBlock;
    }
    if is_rem(t) {
        return TokenType::Rem;
    }
    if t.starts_with("// ") {
        return TokenType::Comment;
    }

    if is_define(t) {
        return TokenType::Define;
    }
    if is_ifdef(t) {
        return TokenType::IfDefined;
    }
    if is_ifnotdef(t) {
        return TokenType::IfNotDefined;
    }
    if is_elsedef(t) {
        return TokenType::ElseDefined;
    }
    if is_endifdef(t) {
        return TokenType::EndIfDefined;
    }

    if t.starts_with("INJECT_VAR $") {
        return TokenType::InjectVar;
    }
    if t.starts_with("DELAY $") {
        return TokenType::DelayVar;
    }
    if t.starts_with("DELAY ") {
        return TokenType::Delay;
    }

    if is_end_string(t) {
        return TokenType::EndString;
    }
    if matches_inline(t, "STRINGLN", "END_STRING", false) {
        return TokenType::StringLn;
    }
    if matches_inline(t, "STRING", "END_STRING", false) {
        return TokenType::String;
    }
    if matches_inline(t, "STRINGLN", "END_STRINGLN", true) {
        return TokenType::StringLn;
    }
    if matches_inline(t, "STRING", "END_STRING", true) {
        return TokenType::String;
    }
    if is_stringln_block(t) {
        return TokenType::StringLnBlock;
    }
    if is_string_block(t) {
        return TokenType::StringBlock;
    }
    if t.starts_with("STRINGLN") && first_char_whitespace(&t["STRINGLN".len()..]) {
        return TokenType::StringLn;
    }
    if t.starts_with("STRING") && first_char_whitespace(&t["STRING".len()..]) {
        return TokenType::String;
    }

    if t.starts_with("HOLD ") {
        return TokenType::Hold;
    }
    if t.starts_with("RELEASE ") {
        return TokenType::Release;
    }
    if t.starts_with("REPEAT ") && repeat_number_prefix(&t["REPEAT ".len()..]) {
        return TokenType::Repeat;
    }
    if t.starts_with("ENTER") {
        return TokenType::Enter;
    }

    if t.starts_with("DEFAULTDELAY ") && exact_number_2plus(&t["DEFAULTDELAY ".len()..]) {
        return TokenType::DefaultDelay;
    }
    if t.starts_with("DEFAULT_DELAY ") && exact_number_2plus(&t["DEFAULT_DELAY ".len()..]) {
        return TokenType::DefaultDelay;
    }
    if t.starts_with("STRINGDELAY ") && repeat_number_prefix(&t["STRINGDELAY ".len()..]) {
        return TokenType::StringDelay;
    }
    if t.starts_with("STRING_DELAY ") && repeat_number_prefix(&t["STRING_DELAY ".len()..]) {
        return TokenType::StringDelay;
    }

    if t.starts_with("IF ") {
        return TokenType::If;
    }
    if t.starts_with('}') {
        return TokenType::EndIf;
    }
    if t.starts_with("END_IF") {
        return TokenType::EndIf;
    }
    if t.starts_with("ELSE IF ") {
        return TokenType::ElseIf;
    }
    if t.starts_with("ELSE") {
        return TokenType::Else;
    }

    if t.starts_with("WHILE ") {
        return TokenType::While;
    }
    if t.starts_with("END_WHILE") {
        return TokenType::EndWhile;
    }

    if t.starts_with("STAGE ") {
        return TokenType::Stage;
    }
    if t.starts_with("END_STAGE") {
        return TokenType::EndStage;
    }

    if t.starts_with("EXTENSION ") {
        return TokenType::Extension;
    }
    if t.starts_with("END_EXTENSION") {
        return TokenType::EndExtension;
    }

    if t.starts_with("FUNCTION ") && t["FUNCTION ".len()..].contains("()") {
        return TokenType::Function;
    }
    if t.starts_with("END_FUNCTION") {
        return TokenType::EndFunction;
    }

    if t.starts_with("ATTACKMODE ") {
        return TokenType::Attackmode;
    }
    if t.starts_with("RETURN ") {
        return TokenType::Return;
    }

    if is_assignment(t) {
        return TokenType::Assignment;
    }
    if t.starts_with("VAR $") && t["VAR $".len()..].contains("= ") {
        return TokenType::Declaration;
    }
    if is_function_call_no_return(t) {
        return TokenType::FunctionCall;
    }

    if t.starts_with("BUTTON_DEF") {
        return TokenType::ButtonDef;
    }
    if t.starts_with("END_BUTTON") {
        return TokenType::EndButtonDef;
    }

    if t.starts_with("DEBUGGER_BREAKPOINT") {
        return TokenType::Breakpoint;
    }
    if t.starts_with("INJECT_BREAKPOINT_LINE_NUMBER") {
        return TokenType::InjectBreakpointLineNumber;
    }

    if t.starts_with("RANDOM_LOWERCASE_LETTER") {
        return TokenType::RandomLowercaseLetter;
    }
    if t.starts_with("RANDOM_UPPERCASE_LETTER") {
        return TokenType::RandomUppercaseLetter;
    }
    if t.starts_with("RANDOM_NUMBER") {
        return TokenType::RandomNumber;
    }
    if t.starts_with("RANDOM_LETTER") {
        return TokenType::RandomLetter;
    }
    if t.starts_with("RANDOM_SPECIAL") {
        return TokenType::RandomSpecial;
    }
    if t.starts_with("RANDOM_CHAR") {
        return TokenType::RandomChar;
    }

    if t.starts_with("RESTORE_HOST_KEYBOARD_LOCK_STATE") {
        return TokenType::Unknown;
    }

    if t == "INJECT_MOD" {
        return TokenType::InjectMod;
    }
    if t.starts_with("INJECT_MOD ") {
        return TokenType::InjectMod;
    }
    if t.starts_with("INJECT ") {
        return TokenType::Inject;
    }
    if t.starts_with("KEYCODE ") {
        return TokenType::Keycode;
    }

    if t.starts_with("EXFIL $") {
        return TokenType::ExfilVar;
    }

    if t.starts_with("MOD_KEY_DOWN ") {
        return TokenType::ModKeyDown;
    }
    if t.starts_with("MOD_KEY_UP ") {
        return TokenType::ModKeyUp;
    }
    if t.starts_with("MOD_UP ") {
        return TokenType::ModUp;
    }
    if t.starts_with("KEY_UP ") {
        return TokenType::KeyUp;
    }
    if t.starts_with("KEY_DOWN ") {
        return TokenType::KeyDown;
    }
    if t.starts_with("MOD_DOWN ") {
        return TokenType::ModDown;
    }

    if US_MODIFIERS.contains(&t) {
        return TokenType::Modifier;
    }

    TokenType::Unknown
}

fn is_assignment(t: &str) -> bool {
    match t.find('$') {
        Some(p) => t[p + 1..].contains("= "),
        None => false,
    }
}

fn is_function_call_no_return(t: &str) -> bool {
    let n = leading_word_chars(t);
    n >= 1 && t[n..].starts_with("()")
}
