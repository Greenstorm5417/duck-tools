use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum CompilerError {
    // Syntax Errors (E001-E099)
    #[error("Syntax error at line {line}: {message}")]
    SyntaxError {
        line: usize,
        message: String,
        column: Option<usize>,
        length: Option<usize>,
        suggestion: Option<String>,
    },

    #[error("Unknown command: {command}")]
    UnknownCommand { line: usize, command: String },

    #[error("Invalid expression: {message}")]
    InvalidExpression { line: usize, message: String },

    #[error("Empty expression")]
    EmptyExpression { line: usize },

    #[error("Unexpected symbol: {symbol}")]
    UnexpectedSymbol { line: usize, symbol: String },

    // Block Structure Errors (E100-E199)
    #[error("Missing END_{block_type}")]
    MissingEnd { line: usize, block_type: String },

    #[error("Unexpected END_{block_type} without matching {block_type}")]
    UnexpectedEnd { line: usize, block_type: String },

    #[error("Mismatched block end: expected END_{expected}, found END_{found}")]
    MismatchedBlockEnd {
        line: usize,
        expected: String,
        found: String,
    },

    #[error("Unclosed block: {block_type} started at line {start_line}")]
    UnclosedBlock {
        line: usize,
        block_type: String,
        start_line: usize,
    },

    // Variable/Function Errors (E200-E299)
    #[error("Unknown variable: {name}")]
    UnknownVariable { line: usize, name: String },

    #[error("Variable redefinition: {name}")]
    VariableRedefinition { line: usize, name: String },

    #[error("Invalid variable name: {name}")]
    InvalidVariableName { line: usize, name: String },

    #[error("Undefined function: {name}")]
    UndefinedFunction { line: usize, name: String },

    #[error("Function redefinition: {name}")]
    FunctionRedefinition { line: usize, name: String },

    #[error("RETURN statement outside function")]
    ReturnOutsideFunction { line: usize },

    // Label/Definition Errors (E300-E399)
    #[error("Unknown label: {label}")]
    UnknownLabel { line: usize, label: String },

    #[error("Duplicate definition: {name}")]
    DuplicateDefinition { line: usize, name: String },

    // Expression/Operator Errors (E400-E499)
    #[error("Division by zero")]
    DivisionByZero { line: usize },

    #[error("Type mismatch: expected {expected}, got {got}")]
    TypeMismatch {
        line: usize,
        expected: String,
        got: String,
    },

    #[error("Invalid operator: {operator}")]
    InvalidOperator { line: usize, operator: String },

    #[error("Mismatched parentheses: {left} '(' vs {right} ')')")]
    MismatchedParentheses {
        line: usize,
        left: usize,
        right: usize,
    },

    // Command Argument Errors (E500-E599)
    #[error("Invalid delay value: {value}")]
    InvalidDelay { line: usize, value: String },

    #[error("Invalid repeat count: {value}")]
    InvalidRepeat { line: usize, value: String },

    #[error("Invalid key code: {code}")]
    InvalidKeyCode { line: usize, code: String },

    #[error("Invalid modifier: {modifier}")]
    InvalidModifier { line: usize, modifier: String },

    #[error("Invalid hex value: {value}")]
    InvalidHex { line: usize, value: String },

    // Preprocessor Errors (E600-E699)
    #[error("Unclosed IF_DEFINED block")]
    UnclosedIfDef { line: usize },

    #[error("DEFINE redefinition: {name}")]
    DefineRedefinition { line: usize, name: String },

    #[error("Circular DEFINE reference: {name}")]
    CircularDefine { line: usize, name: String },

    // System Errors (E700-E799)
    #[error("Payload too large: {size} bytes (max 16384)")]
    PayloadTooLarge { size: usize },

    #[error("Key not found in language file: {key}")]
    KeyNotFound { line: usize, key: String },

    #[error("IO error: {message}")]
    IoError { message: String },

    #[error("Incompatible DuckyScript 1.0 syntax")]
    IncompatibleDS1 { line: usize },
}

impl CompilerError {
    /// Get the line number for all errors
    pub fn line(&self) -> Option<usize> {
        match self {
            CompilerError::SyntaxError { line, .. } => Some(*line),
            CompilerError::UnknownCommand { line, .. } => Some(*line),
            CompilerError::InvalidExpression { line, .. } => Some(*line),
            CompilerError::EmptyExpression { line } => Some(*line),
            CompilerError::UnexpectedSymbol { line, .. } => Some(*line),
            CompilerError::MissingEnd { line, .. } => Some(*line),
            CompilerError::UnexpectedEnd { line, .. } => Some(*line),
            CompilerError::MismatchedBlockEnd { line, .. } => Some(*line),
            CompilerError::UnclosedBlock { line, .. } => Some(*line),
            CompilerError::UnknownVariable { line, .. } => Some(*line),
            CompilerError::VariableRedefinition { line, .. } => Some(*line),
            CompilerError::InvalidVariableName { line, .. } => Some(*line),
            CompilerError::UndefinedFunction { line, .. } => Some(*line),
            CompilerError::FunctionRedefinition { line, .. } => Some(*line),
            CompilerError::ReturnOutsideFunction { line } => Some(*line),
            CompilerError::UnknownLabel { line, .. } => Some(*line),
            CompilerError::DuplicateDefinition { line, .. } => Some(*line),
            CompilerError::DivisionByZero { line } => Some(*line),
            CompilerError::TypeMismatch { line, .. } => Some(*line),
            CompilerError::InvalidOperator { line, .. } => Some(*line),
            CompilerError::MismatchedParentheses { line, .. } => Some(*line),
            CompilerError::InvalidDelay { line, .. } => Some(*line),
            CompilerError::InvalidRepeat { line, .. } => Some(*line),
            CompilerError::InvalidKeyCode { line, .. } => Some(*line),
            CompilerError::InvalidModifier { line, .. } => Some(*line),
            CompilerError::InvalidHex { line, .. } => Some(*line),
            CompilerError::UnclosedIfDef { line } => Some(*line),
            CompilerError::DefineRedefinition { line, .. } => Some(*line),
            CompilerError::CircularDefine { line, .. } => Some(*line),
            CompilerError::KeyNotFound { line, .. } => Some(*line),
            CompilerError::IncompatibleDS1 { line } => Some(*line),
            CompilerError::PayloadTooLarge { .. } => None,
            CompilerError::IoError { .. } => None,
        }
    }

    /// Get the column number for errors that support it
    pub fn column(&self) -> Option<usize> {
        match self {
            CompilerError::SyntaxError { column, .. } => *column,
            _ => None,
        }
    }

    /// Get the error length for errors that support it
    pub fn length(&self) -> Option<usize> {
        match self {
            CompilerError::SyntaxError { length, .. } => *length,
            _ => None,
        }
    }

    /// Get the suggestion for fixing the error
    pub fn suggestion(&self) -> Option<&str> {
        match self {
            CompilerError::SyntaxError { suggestion, .. } => suggestion.as_deref(),
            _ => None,
        }
    }

    /// Get error code for LSP integration
    pub fn code(&self) -> &'static str {
        match self {
            CompilerError::SyntaxError { .. } => "E001",
            CompilerError::UnknownCommand { .. } => "E002",
            CompilerError::InvalidExpression { .. } => "E003",
            CompilerError::EmptyExpression { .. } => "E004",
            CompilerError::UnexpectedSymbol { .. } => "E005",
            CompilerError::MissingEnd { .. } => "E100",
            CompilerError::UnexpectedEnd { .. } => "E101",
            CompilerError::MismatchedBlockEnd { .. } => "E102",
            CompilerError::UnclosedBlock { .. } => "E103",
            CompilerError::UnknownVariable { .. } => "E200",
            CompilerError::VariableRedefinition { .. } => "E201",
            CompilerError::InvalidVariableName { .. } => "E202",
            CompilerError::UndefinedFunction { .. } => "E203",
            CompilerError::FunctionRedefinition { .. } => "E204",
            CompilerError::ReturnOutsideFunction { .. } => "E205",
            CompilerError::UnknownLabel { .. } => "E300",
            CompilerError::DuplicateDefinition { .. } => "E301",
            CompilerError::DivisionByZero { .. } => "E400",
            CompilerError::TypeMismatch { .. } => "E401",
            CompilerError::InvalidOperator { .. } => "E402",
            CompilerError::MismatchedParentheses { .. } => "E403",
            CompilerError::InvalidDelay { .. } => "E500",
            CompilerError::InvalidRepeat { .. } => "E501",
            CompilerError::InvalidKeyCode { .. } => "E502",
            CompilerError::InvalidModifier { .. } => "E503",
            CompilerError::InvalidHex { .. } => "E504",
            CompilerError::UnclosedIfDef { .. } => "E600",
            CompilerError::DefineRedefinition { .. } => "E601",
            CompilerError::CircularDefine { .. } => "E602",
            CompilerError::PayloadTooLarge { .. } => "E700",
            CompilerError::KeyNotFound { .. } => "E701",
            CompilerError::IoError { .. } => "E702",
            CompilerError::IncompatibleDS1 { .. } => "E703",
        }
    }
}

#[derive(Error, Debug, Clone)]
pub enum CompilerWarning {
    #[error("DEFINE {label} modifies this line replacing {old} with {new}")]
    DefineReplacement {
        line: usize,
        label: String,
        old: String,
        new: String,
    },

    #[error("Unused variable: {name}")]
    UnusedVariable { line: usize, name: String },

    #[error("Unused function: {name}")]
    UnusedFunction { line: usize, name: String },

    #[error("Deprecated syntax: use {new} instead of {old}")]
    DeprecatedSyntax {
        line: usize,
        old: String,
        new: String,
    },

    #[error("Suspicious delay value: {delay}ms (very long)")]
    SuspiciousDelay { line: usize, delay: u32 },

    #[error("Empty block: {block_type}")]
    EmptyBlock { line: usize, block_type: String },

    #[error("Unreachable code after RETURN")]
    UnreachableCode { line: usize },
}

impl CompilerWarning {
    /// Get the line number for all warnings
    pub fn line(&self) -> usize {
        match self {
            CompilerWarning::DefineReplacement { line, .. } => *line,
            CompilerWarning::UnusedVariable { line, .. } => *line,
            CompilerWarning::UnusedFunction { line, .. } => *line,
            CompilerWarning::DeprecatedSyntax { line, .. } => *line,
            CompilerWarning::SuspiciousDelay { line, .. } => *line,
            CompilerWarning::EmptyBlock { line, .. } => *line,
            CompilerWarning::UnreachableCode { line } => *line,
        }
    }

    /// Get warning code for LSP integration
    pub fn code(&self) -> &'static str {
        match self {
            CompilerWarning::DefineReplacement { .. } => "W001",
            CompilerWarning::UnusedVariable { .. } => "W002",
            CompilerWarning::UnusedFunction { .. } => "W003",
            CompilerWarning::DeprecatedSyntax { .. } => "W004",
            CompilerWarning::SuspiciousDelay { .. } => "W005",
            CompilerWarning::EmptyBlock { .. } => "W006",
            CompilerWarning::UnreachableCode { .. } => "W007",
        }
    }

    /// Get the message for display
    pub fn message(&self) -> String {
        format!("{}", self)
    }
}

pub type CompilerResult<T> = Result<T, CompilerError>;
