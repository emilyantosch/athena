//! Directory scanning and ingestion orchestration.

use std::path::Path;

use crate::ingestion::{
    ChunkConfig, DocumentChunk, IngestionError, Ingestor, MarkdownParser, SemanticChunker,
};

/// Ingestor for markdown documents.
pub struct MarkdownIngestor {
    parser: MarkdownParser,
    chunker: SemanticChunker,
    config: ChunkConfig,
}

impl MarkdownIngestor {
    pub fn new(config: ChunkConfig) -> Self {
        Self {
            parser: MarkdownParser::new(),
            chunker: SemanticChunker::new(),
            config,
        }
    }
}

impl Ingestor for MarkdownIngestor {
    fn ingest_directory(&self, _path: &Path) -> Result<Vec<DocumentChunk>, IngestionError> {
        todo!("Implement directory ingestion in Phase 5")
    }

    fn ingest_file(&self, _path: &Path) -> Result<Vec<DocumentChunk>, IngestionError> {
        todo!("Implement file ingestion in Phase 5")
    }
}
