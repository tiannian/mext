use serde::{Deserialize, Serialize};

use super::Content;

/// The tittle of document
///
/// For example:
///
/// ```markdown
/// # Header 1
/// ## Header 2
/// ### Header 3
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Tittle {
    pub level: u8,
    pub index: u16,
    pub tittle: String,
}

/// Paragraph of document
#[derive(Debug, Serialize, Deserialize)]
pub struct Paragraph {
    pub lines: Vec<Content>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum QouteItem {
    Content(String),
    Qoute(Box<QouteItem>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ListItem {
    Ordered(String, Vec<ListItem>),
    Plus(String, Vec<ListItem>),
    Dash(String, Vec<ListItem>),
    Star(String, Vec<ListItem>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Code {
    pub attr: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableRow(Vec<String>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub header: Vec<String>,
    pub rows: Vec<TableRow>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BasicItem {
    /// Tittle begin with `# `
    Tittle(Tittle),
    /// Paragraph splited by double `\n`
    Paragraph(Paragraph),
    /// Begin with `> `, `>> `
    Qoute(Vec<QouteItem>),
    /// Begin with `1.`, `2.`, `-`, `+`, `*`
    List(Vec<ListItem>),
    /// Surround by `\`\`\``
    Code(Code),
    /// Use `---`
    HLine,
    /// Surround by `<!--` `-->`
    Comment(String),
    /// Use `$$ $$`
    Latex(String),
    /// Table
    Table(Table),
}
