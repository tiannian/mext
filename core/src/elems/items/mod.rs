use serde::{Deserialize, Serialize};

use super::Content;

mod tittle;
pub use tittle::*;

/// Paragraph of document
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Paragraph {
    pub lines: Vec<Content>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum QouteItem {
    Content(String),
    Qoute(Box<QouteItem>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ListItem {
    Ordered(String, Vec<ListItem>),
    Plus(String, Vec<ListItem>),
    Dash(String, Vec<ListItem>),
    Star(String, Vec<ListItem>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Code {
    pub attr: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TableRow(Vec<String>);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Table {
    pub header: Vec<String>,
    pub rows: Vec<TableRow>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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
