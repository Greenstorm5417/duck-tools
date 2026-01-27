use dashmap::DashMap;
use tower_lsp::lsp_types::Url;

/// In-memory document storage
/// Thread-safe, no disk I/O needed
pub struct DocumentStore {
    documents: DashMap<Url, String>,
}

impl DocumentStore {
    pub fn new() -> Self {
        Self {
            documents: DashMap::new(),
        }
    }

    /// Store or update document content
    pub fn insert(&self, uri: Url, content: String) {
        self.documents.insert(uri, content);
    }

    /// Get document content
    pub fn get(&self, uri: &Url) -> Option<String> {
        self.documents.get(uri).map(|entry| entry.value().clone())
    }

    /// Remove document from store
    pub fn remove(&self, uri: &Url) {
        self.documents.remove(uri);
    }

    /// Check if document exists
    #[allow(dead_code)]
    pub fn contains(&self, uri: &Url) -> bool {
        self.documents.contains_key(uri)
    }
}

impl Default for DocumentStore {
    fn default() -> Self {
        Self::new()
    }
}
