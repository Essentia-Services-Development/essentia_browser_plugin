//! HTML parser.

use crate::errors::{BrowserError, BrowserResult};
use crate::types::{Document, Element};

/// HTML parser.
pub struct HtmlParser;

impl HtmlParser {
    /// Parse HTML string into a document.
    pub fn parse(html: &str, url: &str) -> BrowserResult<Document> {
        if html.is_empty() {
            return Err(BrowserError::Parse("Empty HTML".into()));
        }

        let root = Self::parse_element(html)?;
        let title = Self::extract_title(&root);

        Ok(Document {
            title,
            root,
            url: url.to_string(),
        })
    }

    /// Parse a single element (simplified).
    fn parse_element(html: &str) -> BrowserResult<Element> {
        // Simplified parser - production would use full HTML5 spec
        let html = html.trim();

        if html.starts_with("<!DOCTYPE") || html.starts_with("<!doctype") {
            // Skip doctype
            if let Some(pos) = html.find('>') {
                return Self::parse_element(&html[pos + 1..]);
            }
        }

        // Find first tag
        if let Some(start) = html.find('<') {
            if let Some(end) = html[start..].find('>') {
                let tag_content = &html[start + 1..start + end];
                let tag_name = tag_content.split_whitespace().next().unwrap_or("div");

                return Ok(Element::new(tag_name));
            }
        }

        Ok(Element::new("div"))
    }

    /// Extract title from document.
    fn extract_title(root: &Element) -> String {
        // Find title element recursively
        if root.tag == "title" {
            return root.text_content.clone().unwrap_or_default();
        }

        for child in &root.children {
            let title = Self::extract_title(child);
            if !title.is_empty() {
                return title;
            }
        }

        String::from("Untitled")
    }
}
