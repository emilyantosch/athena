//! Semantic text chunker with heading context awareness.

use crate::ingestion::{ChunkConfig, DocumentChunk, ParsedDocument, TextChunker};

/// A chunker that splits text at semantic boundaries.
pub struct SemanticChunker;

impl SemanticChunker {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SemanticChunker {
    fn default() -> Self {
        Self::new()
    }
}

impl TextChunker for SemanticChunker {
    fn chunk(
        &self,
        _document: &ParsedDocument,
        _source_file: &str,
        _config: &ChunkConfig,
    ) -> Vec<DocumentChunk> {
        let chunks = Vec::<DocumentChunk>::new();
        todo!("Implement semantic chunking in Phase 4.1")
    }
}
