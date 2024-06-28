use serde::{Deserialize, Serialize};

use super::BasicItem;

/// Markdown document
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Document {
    pub attrs: Vec<(String, String)>,
    pub items: Vec<BasicItem>,
}
