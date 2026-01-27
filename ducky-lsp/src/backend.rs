use crate::diagnostics::compiler_to_diagnostics;
use crate::document::DocumentStore;
use ducky_core::DuckyCompiler;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

pub struct Backend {
    pub client: Client,
    pub documents: DocumentStore,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: DocumentStore::new(),
        }
    }

    /// Compile document and publish diagnostics
    async fn validate_document(&self, uri: Url) {
        if let Some(content) = self.documents.get(&uri) {
            let mut compiler = DuckyCompiler::new(None);
            
            // Compile (errors are collected internally)
            let _ = compiler.compile(&content);
            
            // Convert to LSP diagnostics
            let diagnostics = compiler_to_diagnostics(&compiler, &uri);
            
            // Publish to client
            self.client
                .publish_diagnostics(uri, diagnostics, None)
                .await;
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec!["$".to_string(), "_".to_string()]),
                    ..Default::default()
                }),
                document_formatting_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "DuckyScript Language Server".to_string(),
                version: Some("0.1.0".to_string()),
            }),
        })
    }

    async fn initialized(&self, _params: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "DuckyScript LSP initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let content = params.text_document.text;

        self.documents.insert(uri.clone(), content);
        self.validate_document(uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        
        // FULL sync - take the entire new content
        if let Some(change) = params.content_changes.first() {
            self.documents.insert(uri.clone(), change.text.clone());
            self.validate_document(uri).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        self.documents.remove(&uri);
        
        // Clear diagnostics
        self.client.publish_diagnostics(uri, vec![], None).await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        // Re-validate on save (optional, already validated on change)
        self.validate_document(params.text_document.uri).await;
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let _uri = params.text_document_position_params.text_document.uri;
        let _position = params.text_document_position_params.position;

        // TODO: Implement hover for variables, functions, commands
        // For now, just show basic info
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String(
                "DuckyScript command".to_string(),
            )),
            range: None,
        }))
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let _uri = params.text_document_position.text_document.uri;
        let _position = params.text_document_position.position;

        // TODO: Implement smart completion
        // - DuckyScript commands (STRING, DELAY, etc.)
        // - Variables ($var)
        // - Reserved variables ($_RANDOM_INT, etc.)
        // - Functions

        let mut items = vec![];

        // Basic command completions
        for command in &[
            "STRING",
            "STRINGLN",
            "DELAY",
            "ENTER",
            "ATTACKMODE",
            "VAR",
            "IF",
            "WHILE",
            "FUNCTION",
        ] {
            items.push(CompletionItem {
                label: command.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..Default::default()
            });
        }

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri;

        if let Some(_content) = self.documents.get(&uri) {
            // TODO: Implement formatter
            // For now, just return the content unchanged
            // Future: normalize indentation, align commands, etc.
            
            Ok(None) // No formatting changes
        } else {
            Ok(None)
        }
    }
}
