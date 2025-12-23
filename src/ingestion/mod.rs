//! Document ingestion module for Athena knowledge base.
//!
//! This module provides functionality for:
//! - Parsing markdown documents
//! - Chunking text with semantic awareness
//! - Scanning directories for documents
//! - Coordinating the full ingestion pipeline

mod types;
mod error;
mod traits;
mod markdown;
mod chunker;
mod ingestor;

pub use types::{DocumentChunk, ParsedDocument, HeadingInfo, ChunkConfig};
pub use error::IngestionError;
pub use traits::{DocumentParserLocal, TextChunker, Ingestor};
pub use markdown::MarkdownParser;
pub use chunker::SemanticChunker;
pub use ingestor::MarkdownIngestor;
