#![recursion_limit = "256"]

pub mod compiler;
pub mod constants;
pub mod encoder;
pub mod errors;
pub mod language;
pub mod lexer;
pub mod parser;
pub mod preprocessor;

pub use compiler::DuckyCompiler;
pub use errors::{CompilerError, CompilerResult};
pub use language::KeyboardLayout;
