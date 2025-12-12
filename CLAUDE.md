# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Athena** is a Rust-based knowledge base system that uses Qdrant vector database and text embedding to enable agentic code tools (like Claude Code) to work with a knowledge base containing markdown (.md) and PDF files.

### Purpose

Athena allows AI assistants to:
- Index and search through markdown documentation and PDF files
- Retrieve semantically relevant information from a knowledge base
- Provide context-aware responses based on embedded document content

### Technology Stack

- **Language:** Rust (Edition 2024)
- **Vector Database:** Qdrant - high-performance vector similarity search
- **Document Formats:** Markdown (.md) and PDF files
- **Embedding:** Text embedding for semantic search capabilities

## Build Commands

```bash
cargo build          # Build the project
cargo run            # Run the project
cargo test           # Run all tests
cargo test <name>    # Run a specific test by name
cargo clippy         # Run linter
cargo fmt            # Format code
```

## Project Structure

```
athena/
├── .claude/              # Claude Code configuration
│   └── agents/           # AI agent definitions
├── dev_docs/             # Development documentation
│   ├── architecture.md   # System architecture overview
│   ├── decisions/        # Architecture Decision Records (ADRs)
│   └── guides/           # How-to guides
├── src/                  # Source code
│   └── main.rs           # Application entry point
├── feature_specs.json    # Feature tracking and specifications
├── Cargo.toml            # Rust package manifest
└── CLAUDE.md             # This file - AI assistant guidance
```

## Architecture Overview

Athena follows a modular architecture designed for:

1. **Document Ingestion** - Parse and process MD/PDF files
2. **Text Embedding** - Convert text chunks into vector representations
3. **Vector Storage** - Store embeddings in Qdrant for efficient similarity search
4. **Query Interface** - Semantic search API for AI tools to retrieve relevant context

### Key Components (Planned)

- `ingestion/` - Document parsers for markdown and PDF
- `embedding/` - Text embedding service integration
- `storage/` - Qdrant client and vector operations
- `query/` - Search and retrieval interface
- `api/` - External interface for AI tool integration

## Development Guidelines

### Code Style

- Follow Rust idioms and conventions
- Use `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Write tests for new functionality

### Error Handling

- Use `anyhow` for application errors
- Use `thiserror` for library errors with custom types
- Propagate errors with `?` operator where appropriate

### Documentation

- Document public APIs with doc comments (`///`)
- Update `feature_specs.json` when adding new features
- Record significant architectural decisions in `dev_docs/decisions/`

## Feature Tracking

Features are tracked in `feature_specs.json`. Each feature has:
- **id**: Unique identifier (e.g., "feature-001")
- **status**: proposed | approved | in-progress | completed | deprecated
- **priority**: low | medium | high | critical
- **dependencies**: List of feature IDs this depends on

## Environment Setup

### Prerequisites

- Rust toolchain (stable, supporting Edition 2024)
- Qdrant instance (local or cloud)
- Access to a text embedding service/API

### Configuration

Environment variables (use `.env` file):
- `QDRANT_URL` - Qdrant server URL
- `QDRANT_API_KEY` - API key for Qdrant (if using cloud)
- `EMBEDDING_API_URL` - Text embedding service endpoint
- `EMBEDDING_API_KEY` - API key for embedding service

## AI Assistant Notes

When working on this codebase:

1. **Check feature_specs.json** before implementing new features to understand planned functionality
2. **Review dev_docs/architecture.md** for system design context
3. **Update documentation** when making significant changes
4. **Run tests** before suggesting code is complete: `cargo test`
5. **Verify compilation** with `cargo build` after changes
