use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::elems::Content;

#[derive(Debug)]
pub struct ContentParser<'s> {
    origin_str: &'s str,
    contents: Vec<Content>,
}

impl<'s> ContentParser<'s> {
    pub fn new(s: &'s str) -> Self {
        ContentParser {
            origin_str: s.trim(),
            contents: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<usize> {
        let ci = self.origin_str.char_indices();
        let mut s = String::new();

        let mut cidx = 0;

        for (idx, c) in ci {
            log::debug!("Read char: {c} in {idx}");

            match c {
                '*' => {
                    let advance = self.parse_bold_or_ltalic()?;
                    return Ok(idx + advance);
                }
                '_' => {
                    let advance = self.parse_bold_or_ltalic()?;
                    return Ok(idx + advance);
                }
                _ => {
                    s.push(c);
                    cidx = idx;
                }
            }
        }

        self.contents.push(Content::Text(s));

        Ok(cidx)
    }

    pub fn parse_bold_or_ltalic(&mut self) -> Result<usize> {
        let bold = parse_bold(self.origin_str);
        let ltalic = parse_ltalic(self.origin_str);

        match (bold, ltalic) {
            (Some(b), Some(l)) => {
                if b.1 <= l.1 {
                    self.contents.push(b.0);
                    Ok(b.2)
                } else {
                    self.contents.push(l.0);
                    Ok(l.2)
                }
            }
            (Some(b), None) => {
                self.contents.push(b.0);
                Ok(b.2)
            }
            (None, Some(l)) => {
                self.contents.push(l.0);
                Ok(l.2)
            }
            (None, None) => Err(anyhow!("No bold or ltalic found")),
        }
    }
}

fn parse_bold(s: &str) -> Option<(Content, usize, usize)> {
    static BOLD_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(\*\*.*?\*\*|\_\_.*?\_\_)").expect("Failed to inital bold regex")
    });

    if let Some(cap) = BOLD_RE.captures(s) {
        if let Some(m) = cap.get(1) {
            let ms = m.as_str();
            let ms = ms[2..ms.len() - 2].into();

            Some((Content::Bold(ms), m.start(), m.end()))
        } else {
            None
        }
    } else {
        None
    }
}

fn parse_ltalic(s: &str) -> Option<(Content, usize, usize)> {
    static LTALIC_RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(\*.*?\*|\_.*?\_)").expect("Failed to inital ltaic regex"));

    if let Some(cap) = LTALIC_RE.captures(s) {
        if let Some(m) = cap.get(1) {
            let ms = m.as_str();
            let ms = ms[1..ms.len() - 1].into();

            Some((Content::Ltalic(ms), m.start(), m.end()))
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{elems::Content, tests};

    use super::ContentParser;

    fn test_parse_bold(s: &str, expected_idx: usize, expected_s: &str) {
        let mut parser = ContentParser::new(s);

        let idx = parser.parse_bold_or_ltalic().unwrap();
        assert_eq!(idx, expected_idx);
        assert_eq!(parser.contents[0], Content::Bold(expected_s.into()));
    }

    fn test_parse_latlic(s: &str, expected_idx: usize, expected_s: &str) {
        let mut parser = ContentParser::new(s);

        let idx = parser.parse_bold_or_ltalic().unwrap();
        println!("{:?}", parser.contents[0]);
        assert_eq!(idx, expected_idx);
        assert_eq!(parser.contents[0], Content::Ltalic(expected_s.into()));
    }

    #[test]
    fn test_bold() {
        tests::init();

        test_parse_bold("**abcd asdas **", 15, "abcd asdas ");
        test_parse_bold("** **", 5, " ");
        test_parse_bold("****", 4, "");
        test_parse_bold("**abcd asdas ****asdasd**", 15, "abcd asdas ");

        test_parse_bold("__abcd asdas __", 15, "abcd asdas ");
        test_parse_bold("__ __", 5, " ");
        test_parse_bold("____", 4, "");
        test_parse_bold("__abcd asdas ____asdasd__", 15, "abcd asdas ");

        test_parse_latlic("*abcd asdas *", 13, "abcd asdas ");
        test_parse_latlic("* *", 3, " ");
        test_parse_latlic("**", 2, "");
        test_parse_latlic("*abcd asdas **asdasd**", 13, "abcd asdas ");
    }
}
