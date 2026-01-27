mod backend;
mod diagnostics;
mod document;
mod semantic_tokens;
mod formatter;
mod hover;
mod completions;
mod definition;
mod references;
mod rename;
mod document_symbol;
mod document_highlight;
mod code_lens;
mod signature_help;
mod folding_range;
mod code_actions;
mod inlay_hints;
mod selection_range;
mod on_type_formatting;
mod range_formatting;
mod linked_editing;
mod call_hierarchy;

use backend::Backend;
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() {
    // Read from stdin, write to stdout (LSP protocol)
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    // Create LSP service
    let (service, socket) = LspService::new(|client| Backend::new(client));

    // Start server
    Server::new(stdin, stdout, socket).serve(service).await;
}
