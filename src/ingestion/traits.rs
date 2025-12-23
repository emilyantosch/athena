//! Traits defining the ingestion pipeline interfaces.

use std::path::Path;

use crate::ingestion::{ChunkConfig, DocumentChunk, IngestionError, ParsedDocument};

/// A parser that can convert document files into parsed documents.
pub trait DocumentParserLocal {
    /// Parse a file at the given path into a ParsedDocument.
    fn parse(&self, path: &Path) -> Result<ParsedDocument, IngestionError>;

    /// Return the file extensions this parser supports (e.g., ["md", "markdown"]).
    fn supported_extensions(&self) -> &[&str];
}

/// A chunker that splits parsed documents into chunks.
pub trait TextChunker {
    /// Split a parsed document into chunks.
    fn chunk(
        &self,
        document: &ParsedDocument,
        source_file: &str,
        config: &ChunkConfig,
    ) -> Vec<DocumentChunk>;
}

/// An ingestor that orchestrates the full ingestion pipeline.
pub trait Ingestor {
    /// Ingest all supported documents in a directory recursively.
    fn ingest_directory(&self, path: &Path) -> Result<Vec<DocumentChunk>, IngestionError>;

    /// Ingest a single file.
    fn ingest_file(&self, path: &Path) -> Result<Vec<DocumentChunk>, IngestionError>;
}
