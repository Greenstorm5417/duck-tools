use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum CompilerError {
    #[error("Syntax error at line {line}: {message}")]
    SyntaxError { line: usize, message: String },
    
    #[error("Unknown variable: {0}")]
    UnknownVariable(String),
    
    #[error("Unknown label: {0}")]
    UnknownLabel(String),
    
    #[error("Duplicate definition: {0}")]
    DuplicateDefinition(String),
    
    #[error("Payload too large: {size} bytes (max 16384)")]
    PayloadTooLarge { size: usize },
    
    #[error("Mismatched parentheses: {left} '(' vs {right} ')'")]
    MismatchedParentheses { left: usize, right: usize },
    
    #[error("Missing END_{0}")]
    MissingEnd(String),
    
    #[error("Key not found in language file: {0}")]
    KeyNotFound(String),
    
    #[error("Empty expression")]
    EmptyExpression,
    
    #[error("Unexpected symbol: {0}")]
    UnexpectedSymbol(String),
    
    #[error("IO error: {0}")]
    IoError(String),
    
    #[error("Invalid hex value: {0}")]
    InvalidHex(String),
    
    #[error("Incompatible DuckyScript 1.0 syntax")]
    IncompatibleDS1,
}

#[derive(Debug, Clone)]
pub struct CompilerWarning {
    pub line: usize,
    pub message: String,
}

pub type CompilerResult<T> = Result<T, CompilerError>;
