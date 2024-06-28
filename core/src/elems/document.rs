use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::BasicItem;

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub attrs: HashMap<String, String>,
    pub items: Vec<BasicItem>,
}
