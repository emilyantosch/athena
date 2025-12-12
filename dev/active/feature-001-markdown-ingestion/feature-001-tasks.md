# Feature-001 Task Checklist

**Last Updated:** 2025-12-12

## Phase 1: Foundation Setup

- [ ] **1.1** Update Cargo.toml with dependencies
  - Add tokio, pulldown-cmark, walkdir, anyhow, thiserror, serde, uuid, tracing
  - Add tempfile as dev-dependency
  - Verify: `cargo check` passes

- [ ] **1.2** Create `src/ingestion/mod.rs` skeleton
  - Add module declarations
  - Verify: `cargo check` passes

## Phase 2: Core Types & Traits

- [ ] **2.1** Define `DocumentChunk` struct
  - Fields: id, content, source_file, chunk_index, heading_context, page_number
  - Derive: Debug, Clone, Serialize, Deserialize, PartialEq

- [ ] **2.2** Define `ParsedDocument` and `HeadingInfo`
  - HeadingInfo: level, text, char_offset
  - ParsedDocument: content, headings Vec

- [ ] **2.3** Define `ChunkConfig` with defaults
  - Default: chunk_size=512, chunk_overlap=50
  - Add `from_env()` constructor

- [ ] **2.4** Define `IngestionError` enum
  - Variants: FileRead, UnsupportedFileType, MarkdownParse, DirectoryNotFound, InvalidConfig

- [ ] **2.5** Define traits
  - `DocumentParserLocal`: parse(), supported_extensions()
  - `TextChunker`: chunk()
  - `Ingestor`: ingest_directory(), ingest_file()

## Phase 3: Markdown Parser

- [ ] **3.1** Implement `MarkdownParser` struct
  - Constructor: `new()`
  - Extensions: .md, .markdown

- [ ] **3.2** Implement pulldown-cmark event iteration
  - Handle: Heading, Text, Code, Paragraph, List, CodeBlock
  - Build content string with proper formatting

- [ ] **3.3** Implement heading context tracking
  - Record HeadingInfo with level, text, char_offset
  - Handle nested headings correctly

- [ ] **3.4** Write unit tests
  - [ ] Test simple markdown
  - [ ] Test nested headings
  - [ ] Test code blocks
  - [ ] Test lists
  - [ ] Test supported_extensions()

**Checkpoint:** `cargo test markdown` passes

## Phase 4: Semantic Chunker

- [ ] **4.1** Implement `SemanticChunker` struct
  - Constructor: `new()`

- [ ] **4.2** Implement `find_split_point` algorithm
  - Priority: \n\n > .!? > \n > space
  - Search window: target ± 100 chars

- [ ] **4.3** Implement `build_heading_context`
  - Find headings before chunk start
  - Build "A > B > C" format

- [ ] **4.4** Implement `TextChunker` trait
  - Generate UUID for each chunk
  - Handle overlap correctly

- [ ] **4.5** Write unit tests
  - [ ] Test short document (single chunk)
  - [ ] Test paragraph boundaries
  - [ ] Test heading context attachment
  - [ ] Test chunk overlap
  - [ ] Test unique IDs
  - [ ] Test empty document

**Checkpoint:** `cargo test chunker` passes

## Phase 5: Directory Scanner & Integration

- [ ] **5.1** Implement `MarkdownIngestor` struct
  - Fields: parser, chunker, config
  - Constructor: `new(config)`

- [ ] **5.2** Implement `find_markdown_files`
  - Use walkdir for recursive traversal
  - Filter by extension
  - Follow symlinks

- [ ] **5.3** Implement `Ingestor` trait
  - `ingest_directory()`: scan + parse + chunk all files
  - `ingest_file()`: parse + chunk single file
  - Continue on error (warn and skip)

- [ ] **5.4** Add tracing
  - Log file discovery count
  - Log chunk generation count
  - Log errors/warnings

**Checkpoint:** `cargo test ingest` passes

## Phase 6: Main Integration & Testing

- [ ] **6.1** Update `src/main.rs`
  - Add `mod ingestion;`
  - Initialize tracing subscriber

- [ ] **6.2** Add basic CLI
  - Accept directory path argument
  - Print chunk summary

- [ ] **6.3** Create integration tests
  - [ ] Test recursive directory scanning
  - [ ] Test non-markdown file filtering
  - [ ] Test metadata correctness

- [ ] **6.4** Manual testing
  - [ ] Run `cargo run -- dev_docs`
  - [ ] Verify output makes sense

**Final Checkpoint:** `cargo test` passes all tests

## Completion

- [ ] Update feature_specs.json status to "in-progress"
- [ ] All acceptance criteria verified
- [ ] Documentation updated if needed
