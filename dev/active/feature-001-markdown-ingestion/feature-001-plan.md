# Feature-001: Document Ingestion - Markdown

**Last Updated:** 2025-12-12
**Status:** In Progress
**Priority:** High

## Executive Summary

Implement the foundational markdown document ingestion system for Athena. This feature enables recursive directory scanning for `.md` files, parsing with heading context preservation, and semantic chunking suitable for vector embedding.

## Scope

### In Scope
- Recursive directory scanning for `.md` and `.markdown` files
- Markdown parsing with structure preservation (headings, paragraphs, code blocks, lists)
- Semantic text chunking with configurable size and overlap
- Heading context tracking for each chunk
- Source file and position metadata

### Out of Scope
- PDF ingestion (feature-002)
- Embedding generation (feature-003)
- Qdrant storage (feature-004)
- Full CLI interface (feature-006)

## Architecture

```
Directory Path
      │
      ▼
┌─────────────────┐
│ MarkdownIngestor│
│  (walkdir)      │
└────────┬────────┘
         │ .md files
         ▼
┌─────────────────┐
│ MarkdownParser  │
│ (pulldown-cmark)│
└────────┬────────┘
         │ ParsedDocument
         ▼
┌─────────────────┐
│ SemanticChunker │
│ (uuid)          │
└────────┬────────┘
         │
         ▼
   Vec<DocumentChunk>
```

## Implementation Phases

### Phase 1: Foundation Setup
- Update Cargo.toml with all dependencies
- Create ingestion module skeleton

### Phase 2: Core Types & Traits
- DocumentChunk, ParsedDocument, HeadingInfo, ChunkConfig structs
- IngestionError enum
- DocumentParserLocal, TextChunker, Ingestor traits

### Phase 3: Markdown Parser
- MarkdownParser using pulldown-cmark
- Heading context tracking
- Unit tests

### Phase 4: Semantic Chunker
- SemanticChunker with boundary detection
- Overlap management
- Unit tests

### Phase 5: Directory Scanner
- MarkdownIngestor combining all components
- walkdir-based file discovery
- Tracing integration

### Phase 6: Integration
- main.rs updates
- Integration tests
- Manual verification

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| tokio | 1.x | Async runtime |
| pulldown-cmark | 0.13 | Markdown parsing |
| walkdir | 2.x | Directory traversal |
| anyhow | 1.x | Error handling |
| thiserror | 1.x | Custom errors |
| serde | 1.x | Serialization |
| uuid | 1.x | Unique IDs |
| tracing | 0.1 | Logging |
| tempfile | 3.x | Test fixtures (dev) |

## Success Criteria

- [ ] Recursively scan directories for .md files
- [ ] Parse markdown syntax and extract plain text
- [ ] Preserve document structure for context
- [ ] Split documents into appropriately-sized chunks
- [ ] Track source file path and chunk position metadata
- [ ] All tests pass (`cargo test`)
- [ ] Manual test on `dev_docs/` succeeds
