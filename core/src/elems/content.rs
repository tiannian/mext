use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub link: String,
    pub alt: String,
    pub tittle: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hyperlink {
    pub name: String,
    pub url: String,
    pub tittle: String,
}

/// Date: `2024/06/29` or `2024-06-29`
///
/// Each section can be empty,
/// year can be empty: `09/19`
/// day can be empty `2024/09`
#[derive(Debug, Serialize, Deserialize)]
pub struct Date {
    pub year: Option<u16>,
    pub month: Option<u8>,
    pub day: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Time {
    pub hour: Option<u8>,
    pub minute: Option<u8>,
    pub second: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TimeDate {
    /// Date: `2024/06/29` or `2024-06-29`
    Date(Date),
    /// Time: `09:19:23`
    Time(Time),
    /// DateTime: `2024/06/29 09:19:23` or `2024-06-29 09:19:23`
    DateTime(Date, Time),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InlineCode {
    /// `rgb(0,0,0)` or `#ffffff`
    Color(u8, u8, u8),
    TimeDate(TimeDate),
    TimeConjob(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LineContent {
    Text(String),
    /// Use `**bold**` or `__blod__`
    Bold(String),
    /// Use `*ltalic*` or `_ltalic_`
    Ltalic(String),
    /// Use `~~delete~~`
    Delete(String),
    /// Use `![]()`
    Image(Image),
    /// Use `[]()`
    Hyperlink(Hyperlink),
    /// Use `$ $`
    Latex(String),
    /// Use `\`code\``
    Code(InlineCode),
    /// Use `[attr]`
    Attr(String),
    /// Use `[^attr]`
    FootnoteAttr(String),
    /// Use `[!attr]`
    WarningAttr(String),
    /// Use `#attr`
    DashAttr(String),
    /// Use `@namespace/ref/er/ence`
    Reference(String, String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub contents: LineContent,
}
