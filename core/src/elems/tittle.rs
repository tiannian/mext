use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

/// The tittle of document
///
/// For example:
///
/// ```markdown
/// # Header 1
/// ## Header 2
/// ### Header 3
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Tittle {
    pub level: u8,
    pub tittle: String,
}

static TITTLE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(#+)\s(.*)").expect("Failed to create regex"));

impl Tittle {
    pub fn is(s: &str) -> bool {
        TITTLE_RE.is_match(s)
    }

    pub fn new(s: &str) -> Result<Tittle> {
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
    use crate::{elems::Tittle, tests};

    #[test]
    fn test_tittle() {
        tests::init();

        let document = "### Header 3";

        assert!(Tittle::is(document));

        let tittle = Tittle::new(document).unwrap();
        assert_eq!(
            tittle,
            Tittle {
                level: 3,
                tittle: "Header 3".into(),
            }
        );
    }
}
