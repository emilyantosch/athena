//! Error types for document ingestion.

use thiserror::Error;

/// Errors that can occur during document ingestion.
#[derive(Error, Debug)]
pub enum IngestionError {
    #[error("Failed to read file: {0}")]
    FileRead(#[from] std::io::Error),

    #[error("Unsupported file type: {0}")]
    UnsupportedFileType(String),

    #[error("Failed to parse markdown: {0}")]
    MarkdownParse(String),

    #[error("Directory not found: {0}")]
    DirectoryNotFound(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}
