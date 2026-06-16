mod backend;
mod call_hierarchy;
mod code_actions;
mod code_lens;
mod completions;
mod definition;
mod diagnostics;
mod document;
mod document_highlight;
mod document_symbol;
mod folding_range;
mod formatter;
mod hover;
mod inlay_hints;
mod linked_editing;
mod on_type_formatting;
mod range_formatting;
mod references;
mod rename;
mod selection_range;
mod semantic_tokens;
mod signature_help;

use backend::Backend;
use tower_lsp::{LspService, Server};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    // Read from stdin, write to stdout (LSP protocol)
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    // Create LSP service
    let (service, socket) = LspService::new(Backend::new);

    // Start server
    Server::new(stdin, stdout, socket).serve(service).await;
}
