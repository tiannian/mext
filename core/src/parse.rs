use std::io::{BufRead, BufReader, Read};

use anyhow::{anyhow, Result};

use crate::elems::{BasicItem, Document};

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
            } else if self.state == ParserState::Begin && utils::is_tittle(&self.buffer) {
                let tittle = utils::parse_tittle(&self.buffer)?;

                self.document.items.push(BasicItem::Tittle(tittle));
                self.read_new_line = true;
            }
        }
    }

    pub fn document(self) -> Document {
        self.document
    }
}

mod utils {
    use anyhow::{anyhow, Result};
    use once_cell::sync::Lazy;
    use regex::Regex;

    use crate::elems::Tittle;

    static TITTLE_RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(#+)\s(.*)").expect("Failed to create regex"));

    pub fn is_tittle(s: &str) -> bool {
        TITTLE_RE.is_match(s)
    }

    pub fn parse_tittle(s: &str) -> Result<Tittle> {
        let caps = TITTLE_RE.captures(s).ok_or(anyhow!("Failed to match"))?;

        let index_s = caps
            .get(1)
            .ok_or(anyhow!("Failed to get index"))?
            .as_str()
            .trim();
        let tittle = caps
            .get(2)
            .ok_or(anyhow!("Failed to get tittle"))?
            .as_str()
            .into();

        Ok(Tittle {
            level: index_s.len() as u8,
            tittle,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        elems::{BasicItem, Tittle},
        tests, MarkdownParser,
    };

    #[test]
    fn test_attr() {
        tests::init();

        let document = "
---
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

    #[test]
    fn test_tittle() {
        tests::init();

        let document = "### Header 3";

        let mut parser = MarkdownParser::default();
        parser.parse_markdown(document.as_bytes()).unwrap();

        let doc = parser.document();

        let expect = BasicItem::Tittle(Tittle {
            level: 3,
            tittle: "Header 3".into(),
        });

        assert_eq!(doc.items[0], expect);
    }
}
