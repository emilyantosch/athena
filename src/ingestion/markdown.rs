//! Markdown document parser using pulldown-cmark.
use std::fs;
use std::path::Path;

use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

use crate::ingestion::{DocumentParserLocal, HeadingInfo, IngestionError, ParsedDocument};

/// Parser for markdown documents.
pub struct MarkdownParser;

impl MarkdownParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse markdown content string into a ParsedDocument.
    fn parse_content(&self, markdown: &str) -> ParsedDocument {
        let parser = Parser::new(markdown);

        let mut content = String::new();
        let mut headings = Vec::new();
        let mut current_heading_level: Option<HeadingLevel> = None;
        let mut current_heading_text = String::new();
        let mut in_code_block = false;
        let mut list_depth: usize = 0;

        for event in parser {
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    current_heading_level = Some(level);
                    current_heading_text.clear();
                }
                Event::End(TagEnd::Heading(_)) => {
                    if let Some(level) = current_heading_level.take() {
                        // Add spacing before heading
                        if !content.is_empty() {
                            content.push_str("\n\n");
                        }
                        // Record char_offset at the actual start of heading text
                        let char_offset = content.len();
                        content.push_str(&current_heading_text);
                        content.push('\n');

                        headings.push(HeadingInfo {
                            level: heading_level_to_u32(level),
                            text: current_heading_text.clone(),
                            char_offset,
                        });
                        current_heading_text.clear();
                    }
                }
                Event::Start(Tag::Paragraph) => {
                    if !content.is_empty() && !content.ends_with('\n') {
                        content.push_str("\n\n");
                    }
                }
                Event::End(TagEnd::Paragraph) => {
                    content.push('\n');
                }
                Event::Start(Tag::CodeBlock(_)) => {
                    in_code_block = true;
                    if !content.is_empty() && !content.ends_with('\n') {
                        content.push_str("\n\n");
                    }
                }
                Event::End(TagEnd::CodeBlock) => {
                    in_code_block = false;
                    content.push('\n');
                }
                Event::Start(Tag::List(_)) => {
                    list_depth += 1;
                    if list_depth == 1 && !content.is_empty() && !content.ends_with('\n') {
                        content.push('\n');
                    }
                }
                Event::End(TagEnd::List(_)) => {
                    list_depth = list_depth.saturating_sub(1);
                    if list_depth == 0 {
                        content.push('\n');
                    }
                }
                Event::Start(Tag::Item) => {
                    let indent = "  ".repeat(list_depth.saturating_sub(1));
                    content.push_str(&indent);
                    content.push_str("- ");
                }
                Event::End(TagEnd::Item) => {
                    if !content.ends_with('\n') {
                        content.push('\n');
                    }
                }
                Event::Text(text) => {
                    if current_heading_level.is_some() {
                        current_heading_text.push_str(&text);
                    } else {
                        content.push_str(&text);
                    }
                }
                Event::Code(code) => {
                    if current_heading_level.is_some() {
                        current_heading_text.push('`');
                        current_heading_text.push_str(&code);
                        current_heading_text.push('`');
                    } else {
                        content.push('`');
                        content.push_str(&code);
                        content.push('`');
                    }
                }
                Event::SoftBreak | Event::HardBreak => {
                    if current_heading_level.is_some() {
                        current_heading_text.push(' ');
                    } else if in_code_block {
                        content.push('\n');
                    } else {
                        content.push(' ');
                    }
                }
                _ => {}
            }
        }

        // Trim trailing whitespace
        let content = content.trim_end().to_string();

        ParsedDocument { content, headings }
    }
}

fn heading_level_to_u32(level: HeadingLevel) -> u32 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

impl Default for MarkdownParser {
    fn default() -> Self {
        Self::new()
    }
}

impl DocumentParserLocal for MarkdownParser {
    fn parse(&self, path: &Path) -> Result<ParsedDocument, IngestionError> {
        let markdown = fs::read_to_string(path).map_err(IngestionError::FileRead)?;

        Ok(self.parse_content(&markdown))
    }

    fn supported_extensions(&self) -> &[&str] {
        &["md", "markdown"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_simple_markdown() {
        let parser = MarkdownParser::new();
        let markdown = "# Hello World\n\nThis is a simple paragraph.";
        let doc = parser.parse_content(markdown);

        assert!(doc.content.contains("Hello World"));
        assert!(doc.content.contains("This is a simple paragraph."));
        assert_eq!(doc.headings.len(), 1);
        assert_eq!(doc.headings[0].level, 1);
        assert_eq!(doc.headings[0].text, "Hello World");
    }

    #[test]
    fn test_nested_headings() {
        let parser = MarkdownParser::new();
        let markdown = r#"# Top Level

Some intro text.

## Second Level

More details here.

### Third Level

Even more details.

## Another Second Level

Back to level 2.
"#;
        let doc = parser.parse_content(markdown);

        assert_eq!(doc.headings.len(), 4);

        assert_eq!(doc.headings[0].level, 1);
        assert_eq!(doc.headings[0].text, "Top Level");

        assert_eq!(doc.headings[1].level, 2);
        assert_eq!(doc.headings[1].text, "Second Level");

        assert_eq!(doc.headings[2].level, 3);
        assert_eq!(doc.headings[2].text, "Third Level");

        assert_eq!(doc.headings[3].level, 2);
        assert_eq!(doc.headings[3].text, "Another Second Level");

        // Verify char_offsets are in order (each heading comes after the previous)
        for i in 1..doc.headings.len() {
            assert!(
                doc.headings[i].char_offset > doc.headings[i - 1].char_offset,
                "Heading {} should have char_offset > heading {}",
                i,
                i - 1
            );
        }
    }

    #[test]
    fn test_code_blocks() {
        let parser = MarkdownParser::new();
        let markdown = r#"# Code Example

Here is some code:

```rust
fn main() {
    println!("Hello");
}
```

And some inline `code` too.
"#;
        let doc = parser.parse_content(markdown);

        assert!(doc.content.contains("fn main()"));
        assert!(doc.content.contains("println!"));
        assert!(doc.content.contains("`code`"));
    }

    #[test]
    fn test_lists() {
        let parser = MarkdownParser::new();
        let markdown = r#"# Shopping List

- Apples
- Bananas
- Oranges

And numbered items too.
"#;
        let doc = parser.parse_content(markdown);

        assert!(doc.content.contains("- Apples"));
        assert!(doc.content.contains("- Bananas"));
        assert!(doc.content.contains("- Oranges"));
    }

    #[test]
    fn test_nested_lists() {
        let parser = MarkdownParser::new();
        let markdown = r#"# Nested List

- Item 1
  - Sub item A
  - Sub item B
- Item 2
"#;
        let doc = parser.parse_content(markdown);

        // The content should contain the list items
        assert!(doc.content.contains("Item 1"));
        assert!(doc.content.contains("Sub item A"));
        assert!(doc.content.contains("Sub item B"));
        assert!(doc.content.contains("Item 2"));
    }

    #[test]
    fn test_supported_extensions() {
        let parser = MarkdownParser::new();
        let extensions = parser.supported_extensions();

        assert!(extensions.contains(&"md"));
        assert!(extensions.contains(&"markdown"));
        assert_eq!(extensions.len(), 2);
    }

    #[test]
    fn test_parse_file() {
        let parser = MarkdownParser::new();

        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "# Test File\n\nSome content here.").unwrap();

        let doc = parser.parse(temp_file.path()).unwrap();

        assert!(doc.content.contains("Test File"));
        assert!(doc.content.contains("Some content here."));
        assert_eq!(doc.headings.len(), 1);
    }

    #[test]
    fn test_heading_with_inline_code() {
        let parser = MarkdownParser::new();
        let markdown = "# The `main` function\n\nDescription here.";
        let doc = parser.parse_content(markdown);

        assert_eq!(doc.headings.len(), 1);
        assert_eq!(doc.headings[0].text, "The `main` function");
    }

    #[test]
    fn test_empty_document() {
        let parser = MarkdownParser::new();
        let doc = parser.parse_content("");

        assert!(doc.content.is_empty());
        assert!(doc.headings.is_empty());
    }

    #[test]
    fn test_document_without_headings() {
        let parser = MarkdownParser::new();
        let markdown = "Just some plain text.\n\nWith multiple paragraphs.";
        let doc = parser.parse_content(markdown);

        assert!(doc.content.contains("Just some plain text."));
        assert!(doc.content.contains("With multiple paragraphs."));
        assert!(doc.headings.is_empty());
    }

    #[test]
    fn test_heading_char_offset_accuracy() {
        let parser = MarkdownParser::new();
        let markdown = "# First\n\nSome text.\n\n## Second";
        let doc = parser.parse_content(markdown);

        assert_eq!(doc.headings.len(), 2);

        // First heading should start at offset 0
        assert_eq!(doc.headings[0].char_offset, 0);
        assert_eq!(&doc.content[doc.headings[0].char_offset..].starts_with("First"), &true);

        // Second heading char_offset should point to where "Second" starts in content
        let second_offset = doc.headings[1].char_offset;
        assert!(doc.content[second_offset..].starts_with("Second"));
    }
}
