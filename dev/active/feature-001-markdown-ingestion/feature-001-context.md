# Feature-001 Context

**Last Updated:** 2025-12-12

## Key Files

### Files to Create
| File | Purpose |
|------|---------|
| `src/ingestion/mod.rs` | Module root: types, traits, errors, MarkdownIngestor |
| `src/ingestion/markdown.rs` | MarkdownParser implementation |
| `src/ingestion/chunker.rs` | SemanticChunker implementation |
| `tests/integration_test.rs` | Full pipeline integration tests |

### Files to Modify
| File | Changes |
|------|---------|
| `Cargo.toml` | Add 8 dependencies + 1 dev-dependency |
| `src/main.rs` | Add `mod ingestion;` and basic CLI |

### Reference Files
| File | Purpose |
|------|---------|
| `dev_docs/architecture.md` | System design and module structure |
| `feature_specs.json` | Feature requirements and acceptance criteria |
| `CLAUDE.md` | Development guidelines |
| `dev_docs/decisions/0001-use-qdrant.md` | ADR for vector storage choice |

## Key Decisions

### 1. Markdown Parser: pulldown-cmark
- **Rationale**: Recommended in feature_specs.json, well-maintained, CommonMark compliant
- **Alternative considered**: markdown-rs (less mature)

### 2. Directory Traversal: walkdir (sync)
- **Rationale**: More mature than async alternatives, file discovery is not a bottleneck
- **Alternative considered**: async-walkdir (added complexity)

### 3. Chunking Strategy: Semantic boundaries
- **Priority order**: Paragraph (\n\n) > Sentence (.!?) > Line (\n) > Word (space)
- **Rationale**: Preserves semantic meaning at chunk boundaries

### 4. Heading Context Format
- **Format**: "Title > Section > Subsection"
- **Rationale**: Hierarchical, human-readable, useful for search context

## Core Data Types

```rust
pub struct DocumentChunk {
    pub id: String,                      // UUID v4
    pub content: String,                 // Chunk text
    pub source_file: PathBuf,            // Source path
    pub chunk_index: usize,              // Position in document
    pub heading_context: Option<String>, // "A > B > C"
    pub page_number: Option<usize>,      // None for markdown
}

pub struct ParsedDocument {
    pub content: String,
    pub headings: Vec<HeadingInfo>,
}

pub struct HeadingInfo {
    pub level: u8,
    pub text: String,
    pub char_offset: usize,
}

pub struct ChunkConfig {
    pub chunk_size: usize,    // Default: 512
    pub chunk_overlap: usize, // Default: 50
}
```

## Dependencies on This Feature

| Feature | Dependency Type |
|---------|-----------------|
| feature-002 (PDF) | Shares types and traits |
| feature-003 (Embedding) | Consumes DocumentChunk |
| feature-004 (Qdrant) | Stores DocumentChunk vectors |
| feature-005 (Search) | Queries stored chunks |

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `CHUNK_SIZE` | 512 | Target chunk size in characters |
| `CHUNK_OVERLAP` | 50 | Overlap between consecutive chunks |

## Testing Strategy

### Unit Tests (in each module)
- `markdown.rs`: Parse simple, nested headings, code blocks, lists
- `chunker.rs`: Short docs, boundaries, overlap, empty docs, unique IDs

### Integration Tests (`tests/integration_test.rs`)
- Recursive directory scanning
- Non-markdown file filtering
- Full pipeline with metadata verification
