#![recursion_limit = "256"]

pub mod lexer;
pub mod parser;
pub mod compiler;
pub mod encoder;
pub mod preprocessor;
pub mod language;
pub mod errors;
pub mod constants;

pub use compiler::DuckyCompiler;
pub use errors::{CompilerError, CompilerResult};
pub use language::KeyboardLayout;
