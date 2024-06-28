use std::io::{BufRead, BufReader, Read};

use anyhow::{anyhow, Result};

use crate::elems::Document;

#[derive(Debug, PartialEq, Eq)]
pub enum ParserState {
    Begin,
    DocumentAttr,
    Document,
}

pub struct MarkdownParser {
    state: ParserState,
    buffer: String,
    document: Document,
    read_new_line: bool,
}

impl Default for MarkdownParser {
    fn default() -> Self {
        Self {
            state: ParserState::Begin,
            buffer: String::default(),
            document: Document::default(),
            read_new_line: true,
        }
    }
}

impl MarkdownParser {
    pub fn parse_markdown(&mut self, reader: impl Read) -> Result<()> {
        let mut reader = BufReader::new(reader);

        loop {
            if self.read_new_line {
                self.buffer.clear();

                let nbytes = reader.read_line(&mut self.buffer)?;

                if nbytes == 0 {
                    return Ok(());
                }

                self.buffer = self.buffer.trim().into();

                log::debug!("Read text: {}", self.buffer);
            }
            log::debug!("State: {:?}", self.state);

            if self.state == ParserState::Begin && self.buffer == "---" {
                log::debug!("Meet `---` at begin of document, entry document attr");

                self.state = ParserState::DocumentAttr;
                self.read_new_line = true;
            } else if self.state == ParserState::DocumentAttr && self.buffer == "---" {
                log::debug!("Meet `---` at attr of document, complete attr, back document");

                self.state = ParserState::Begin;
                self.read_new_line = true;
            } else if self.state == ParserState::DocumentAttr {
                let mut splited = self.buffer.split(':');

                let key = splited.next().ok_or(anyhow!("Failed to get key"))?.trim();
                let value = splited.next().ok_or(anyhow!("Failed to get value"))?.trim();

                log::debug!("Parse key: {key}, value: {value}");

                self.document.attrs.push((key.into(), value.into()));
                self.read_new_line = true;
            } else if self.state == ParserState::Begin && utils::is_header(&self.buffer) {
            }
        }
    }

    pub fn document(self) -> Document {
        self.document
    }
}

mod utils {
    pub fn is_header(s: &str) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::{tests, MarkdownParser};

    #[test]
    fn test_attr() {
        tests::init();

        let document = "---
key1: value1
key2: value2
---
";

        let mut parser = MarkdownParser::default();

        parser.parse_markdown(document.as_bytes()).unwrap();

        let doc = parser.document();

        assert_eq!(doc.attrs[0].0, "key1");
        assert_eq!(doc.attrs[0].1, "value1");
        assert_eq!(doc.attrs[1].0, "key2");
        assert_eq!(doc.attrs[1].1, "value2");
    }
}
