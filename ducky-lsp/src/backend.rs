use crate::diagnostics::compiler_to_diagnostics;
use crate::document::DocumentStore;
use crate::semantic_tokens::{generate_semantic_tokens, semantic_token_types_legend, semantic_token_modifiers_legend};
use crate::formatter::format_document;
use crate::hover::{get_hover_info, get_variable_hover};
use crate::completions::{get_keyword_completions, get_modifier_completions, get_special_key_completions, get_reserved_variable_completions};
use crate::definition::find_definition;
use crate::references::find_references;
use crate::rename::{prepare_rename, rename_symbol};
use crate::document_symbol::get_document_symbols;
use crate::document_highlight::get_document_highlights;
use crate::code_lens::get_code_lenses;
use crate::signature_help::get_signature_help;
use crate::folding_range::get_folding_ranges;
use crate::code_actions::get_code_actions;
use crate::inlay_hints::get_inlay_hints;
use crate::selection_range::get_selection_ranges;
use crate::on_type_formatting::get_on_type_formatting;
use crate::range_formatting::format_range;
use crate::linked_editing::get_linked_editing_ranges;
use crate::call_hierarchy::{prepare_call_hierarchy, get_incoming_calls, get_outgoing_calls};
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
            
            // Compile - ignore Result, we want ALL errors collected
            // The compiler.errors vec will have all errors even if compile() returns Err
            let _ = compiler.compile(&content);
            
            // Convert to LSP diagnostics (reads from compiler.errors and compiler.warnings)
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
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            legend: SemanticTokensLegend {
                                token_types: semantic_token_types_legend(),
                                token_modifiers: semantic_token_modifiers_legend(),
                            },
                            full: Some(SemanticTokensFullOptions::Bool(true)),
                            range: Some(false),
                            ..Default::default()
                        }
                    )
                ),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Right(RenameOptions {
                    prepare_provider: Some(true),
                    work_done_progress_options: Default::default(),
                })),
                document_symbol_provider: Some(OneOf::Left(true)),
                document_highlight_provider: Some(OneOf::Left(true)),
                code_lens_provider: Some(CodeLensOptions {
                    resolve_provider: Some(false),
                }),
                signature_help_provider: Some(SignatureHelpOptions {
                    trigger_characters: Some(vec!["(".to_string(), " ".to_string()]),
                    retrigger_characters: None,
                    work_done_progress_options: Default::default(),
                }),
                folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                inlay_hint_provider: Some(OneOf::Left(true)),
                selection_range_provider: Some(SelectionRangeProviderCapability::Simple(true)),
                document_on_type_formatting_provider: Some(DocumentOnTypeFormattingOptions {
                    first_trigger_character: "\n".to_string(),
                    more_trigger_character: None,
                }),
                document_range_formatting_provider: Some(OneOf::Left(true)),
                linked_editing_range_provider: Some(LinkedEditingRangeServerCapabilities::Simple(true)),
                call_hierarchy_provider: Some(CallHierarchyServerCapability::Simple(true)),
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
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        if let Some(content) = self.documents.get(&uri) {
            let lines: Vec<&str> = content.lines().collect();
            
            if let Some(line) = lines.get(position.line as usize) {
                // Check if hovering over a variable
                let chars: Vec<char> = line.chars().collect();
                if let Some(ch) = chars.get(position.character as usize) {
                    if *ch == '$' || (position.character > 0 && chars.get((position.character - 1) as usize) == Some(&'$')) {
                        // Extract variable name
                        let var_start = if *ch == '$' { position.character } else { position.character - 1 };
                        let var_name: String = chars.iter()
                            .skip(var_start as usize)
                            .take_while(|c| c.is_alphanumeric() || **c == '_' || **c == '$')
                            .collect();
                        
                        if let Some(hover) = get_variable_hover(&var_name) {
                            return Ok(Some(hover));
                        }
                    }
                }
                
                // Get hover for the command on this line
                return Ok(get_hover_info(line, position));
            }
        }

        Ok(None)
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        let mut items = Vec::new();
        
        // Check context for smart completions
        if let Some(content) = self.documents.get(&uri) {
            let lines: Vec<&str> = content.lines().collect();
            if let Some(line) = lines.get(position.line as usize) {
                let line_prefix = &line[..position.character.min(line.len() as u32) as usize];
                
                // If typing a variable ($)
                if line_prefix.ends_with('$') || line_prefix.contains("$_") {
                    items.extend(get_reserved_variable_completions());
                }
                // Otherwise, show all completions
                else {
                    // Add all keyword completions
                    items.extend(get_keyword_completions());
                    
                    // Add modifier keys
                    items.extend(get_modifier_completions());
                    
                    // Add special keys
                    items.extend(get_special_key_completions());
                    
                    // Add reserved variables
                    items.extend(get_reserved_variable_completions());
                }
            }
        } else {
            // Default: show all completions
            items.extend(get_keyword_completions());
            items.extend(get_modifier_completions());
            items.extend(get_special_key_completions());
            items.extend(get_reserved_variable_completions());
        }

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri;

        if let Some(content) = self.documents.get(&uri) {
            let edits = format_document(&content, &params.options);
            
            if edits.is_empty() {
                Ok(None)
            } else {
                Ok(Some(edits))
            }
        } else {
            Ok(None)
        }
    }
    
    async fn semantic_tokens_full(&self, params: SemanticTokensParams) -> Result<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri;
        
        if let Some(content) = self.documents.get(&uri) {
            let tokens = generate_semantic_tokens(&content);
            
            Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
                result_id: None,
                data: tokens,
            })))
        } else {
            Ok(None)
        }
    }

    async fn goto_definition(&self, params: GotoDefinitionParams) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        if let Some(content) = self.documents.get(&uri) {
            if let Some(location) = find_definition(&content, position, &uri) {
                return Ok(Some(GotoDefinitionResponse::Scalar(location)));
            }
        }

        Ok(None)
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        if let Some(content) = self.documents.get(&uri) {
            let locations = find_references(&content, position, params.context.include_declaration, &uri);
            
            if !locations.is_empty() {
                return Ok(Some(locations));
            }
        }

        Ok(None)
    }

    async fn prepare_rename(&self, params: TextDocumentPositionParams) -> Result<Option<PrepareRenameResponse>> {
        let uri = params.text_document.uri;
        let position = params.position;

        if let Some(content) = self.documents.get(&uri) {
            if let Some(range) = prepare_rename(&content, position) {
                return Ok(Some(PrepareRenameResponse::Range(range)));
            }
        }

        Ok(None)
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let new_name = params.new_name;

        if let Some(content) = self.documents.get(&uri) {
            if let Some(edit) = rename_symbol(&content, position, &new_name, &uri) {
                return Ok(Some(edit));
            }
        }

        Ok(None)
    }

    async fn document_symbol(&self, params: DocumentSymbolParams) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri;

        if let Some(content) = self.documents.get(&uri) {
            let symbols = get_document_symbols(&content);
            
            if !symbols.is_empty() {
                return Ok(Some(DocumentSymbolResponse::Nested(symbols)));
            }
        }

        Ok(None)
    }

    async fn document_highlight(&self, params: DocumentHighlightParams) -> Result<Option<Vec<DocumentHighlight>>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        if let Some(content) = self.documents.get(&uri) {
            let highlights = get_document_highlights(&content, position);
            
            if !highlights.is_empty() {
                return Ok(Some(highlights));
            }
        }

        Ok(None)
    }

    async fn code_lens(&self, params: CodeLensParams) -> Result<Option<Vec<CodeLens>>> {
        let uri = params.text_document.uri;

        if let Some(content) = self.documents.get(&uri) {
            let lenses = get_code_lenses(&content, &uri);
            
            if !lenses.is_empty() {
                return Ok(Some(lenses));
            }
        }

        Ok(None)
    }

    async fn signature_help(&self, params: SignatureHelpParams) -> Result<Option<SignatureHelp>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        if let Some(content) = self.documents.get(&uri) {
            return Ok(get_signature_help(&content, position));
        }

        Ok(None)
    }

    async fn folding_range(&self, params: FoldingRangeParams) -> Result<Option<Vec<FoldingRange>>> {
        let uri = params.text_document.uri;

        if let Some(content) = self.documents.get(&uri) {
            let ranges = get_folding_ranges(&content);
            
            if !ranges.is_empty() {
                return Ok(Some(ranges));
            }
        }

        Ok(None)
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let uri = params.text_document.uri;
        let range = params.range;
        let diagnostics = params.context.diagnostics;

        if let Some(content) = self.documents.get(&uri) {
            let actions = get_code_actions(&content, range, diagnostics, &uri);
            
            if !actions.is_empty() {
                return Ok(Some(actions.into_iter().map(CodeActionOrCommand::CodeAction).collect()));
            }
        }

        Ok(None)
    }

    async fn inlay_hint(&self, params: InlayHintParams) -> Result<Option<Vec<InlayHint>>> {
        let uri = params.text_document.uri;
        let range = params.range;

        if let Some(content) = self.documents.get(&uri) {
            let hints = get_inlay_hints(&content, range);
            
            if !hints.is_empty() {
                return Ok(Some(hints));
            }
        }

        Ok(None)
    }

    async fn selection_range(&self, params: SelectionRangeParams) -> Result<Option<Vec<SelectionRange>>> {
        let uri = params.text_document.uri;
        let positions = params.positions;

        if let Some(content) = self.documents.get(&uri) {
            let ranges = get_selection_ranges(&content, positions);
            
            if !ranges.is_empty() {
                return Ok(Some(ranges));
            }
        }

        Ok(None)
    }

    async fn on_type_formatting(&self, params: DocumentOnTypeFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let ch = params.ch;
        let options = params.options;

        if let Some(content) = self.documents.get(&uri) {
            let edits = get_on_type_formatting(&content, position, &ch, &options);
            
            if !edits.is_empty() {
                return Ok(Some(edits));
            }
        }

        Ok(None)
    }

    async fn range_formatting(&self, params: DocumentRangeFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri;
        let range = params.range;
        let options = params.options;

        if let Some(content) = self.documents.get(&uri) {
            let edits = format_range(&content, range, &options);
            
            if !edits.is_empty() {
                return Ok(Some(edits));
            }
        }

        Ok(None)
    }

    async fn linked_editing_range(&self, params: LinkedEditingRangeParams) -> Result<Option<LinkedEditingRanges>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        if let Some(content) = self.documents.get(&uri) {
            return Ok(get_linked_editing_ranges(&content, position));
        }

        Ok(None)
    }

    async fn prepare_call_hierarchy(&self, params: CallHierarchyPrepareParams) -> Result<Option<Vec<CallHierarchyItem>>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        if let Some(content) = self.documents.get(&uri) {
            if let Some(items) = prepare_call_hierarchy(&content, position, &uri) {
                return Ok(Some(items));
            }
        }

        Ok(None)
    }

    async fn incoming_calls(&self, params: CallHierarchyIncomingCallsParams) -> Result<Option<Vec<CallHierarchyIncomingCall>>> {
        let uri = params.item.uri.clone();

        if let Some(content) = self.documents.get(&uri) {
            let calls = get_incoming_calls(&content, &params.item, &uri);
            
            if !calls.is_empty() {
                return Ok(Some(calls));
            }
        }

        Ok(None)
    }

    async fn outgoing_calls(&self, params: CallHierarchyOutgoingCallsParams) -> Result<Option<Vec<CallHierarchyOutgoingCall>>> {
        let uri = params.item.uri.clone();

        if let Some(content) = self.documents.get(&uri) {
            let calls = get_outgoing_calls(&content, &params.item, &uri);
            
            if !calls.is_empty() {
                return Ok(Some(calls));
            }
        }

        Ok(None)
    }
}
