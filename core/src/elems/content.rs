use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Image {
    pub link: String,
    pub alt: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Hyperlink {
    pub name: String,
    pub url: String,
}

/// Date: `2024/06/29` or `2024-06-29`
///
/// Each section can be empty,
/// year can be empty: `09/19`
/// day can be empty `2024/09`
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Date {
    pub year: Option<u16>,
    pub month: Option<u8>,
    pub day: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Time {
    pub hour: Option<u8>,
    pub minute: Option<u8>,
    pub second: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TimeDate {
    /// Date: `2024/06/29` or `2024-06-29`
    Date(Date),
    /// Time: `09:19:23`
    Time(Time),
    /// DateTime: `2024/06/29 09:19:23` or `2024-06-29 09:19:23`
    DateTime(Date, Time),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum InlineCode {
    /// `rgb(0,0,0)` or `#ffffff`
    Color(u8, u8, u8),
    TimeDate(TimeDate),
    TimeConjob(String),
    Text(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Content {
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

// mod utils {
//     use once_cell::sync::{Lazy, OnceCell};
//     use regex::RegexSet;
//
//     const BOLD_RE: &str = r"\*\*(.*?)\*\*|\_\_(.*?)\_\_";
//     const LTALIC_RE: &str = r"\*(.*?)\*|\_(.*?)\_";
//     const DELETE_RE: &str = r"~~(.*?)~~";
//     const IMAGE_RE: &str = r"!\[(.*?)\]\((.*?)\)";
//     const HYPERLINK_RE: &str = r"\[(.*)\]\((.*?)\)";
//     const LATEX_RE: &str = r"\$(.*)\$";
//     const CODE_RE: &str = r"`(.*)`";
//
//     pub fn parse() {
//         static RE: Lazy<RegexSet> = Lazy::new(|| {
//             RegexSet::new([
//                 BOLD_RE,
//                 LTALIC_RE,
//                 DELETE_RE,
//                 IMAGE_RE,
//                 HYPERLINK_RE,
//                 LATEX_RE,
//                 CODE_RE,
//             ])
//             .unwrap()
//         });
//
//         // RE.ma
//     }
// }
