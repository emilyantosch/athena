//! Core types for document ingestion.

use serde::{Deserialize, Serialize};

/// A chunk of text extracted from a document.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentChunk {
    pub id: String,
    pub content: String,
    pub source_file: String,
    pub chunk_index: usize,
    pub heading_context: Option<String>,
    pub page_number: Option<usize>,
}

/// Information about a heading in a document.
#[derive(Debug, Clone, PartialEq)]
pub struct HeadingInfo {
    pub level: u32,
    pub text: String,
    pub char_offset: usize,
}

/// A parsed document with content and structural information.
#[derive(Debug, Clone)]
pub struct ParsedDocument {
    pub content: String,
    pub headings: Vec<HeadingInfo>,
}

/// Configuration for text chunking.
#[derive(Debug, Clone)]
pub struct ChunkConfig {
    pub chunk_size: usize,
    pub chunk_overlap: usize,
}

impl Default for ChunkConfig {
    fn default() -> Self {
        Self {
            chunk_size: 512,
            chunk_overlap: 50,
        }
    }
}

impl ChunkConfig {
    /// Create a new ChunkConfig from environment variables.
    /// Falls back to defaults if not set.
    pub fn from_env() -> Self {
        let chunk_size = std::env::var("CHUNK_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(512);

        let chunk_overlap = std::env::var("CHUNK_OVERLAP")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(50);

        Self {
            chunk_size,
            chunk_overlap,
        }
    }
}
