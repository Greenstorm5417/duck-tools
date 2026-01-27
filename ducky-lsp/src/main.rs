mod backend;
mod diagnostics;
mod document;

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
