use std::collections::HashMap;

use leptosfmt_pretty_printer::{Printer, PrinterSettings};

mod attribute;
mod element;
mod expr;
mod fragment;
mod mac;
mod node;

pub use mac::format_macro;
pub use mac::HtmlMacro;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum AttributeValueBraceStyle {
    Always,
    AlwaysUnlessLit,
    WhenRequired,
    Preserve,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct FormatterSettings {
    // Maximum width of each line
    pub max_width: usize,

    // Number of spaces per tab
    pub tab_spaces: usize,

    // Determines placement of braces around single expression attribute values
    pub attr_value_brace_style: AttributeValueBraceStyle,
}

impl Default for FormatterSettings {
    fn default() -> Self {
        Self {
            max_width: 100,
            tab_spaces: 4,
            attr_value_brace_style: AttributeValueBraceStyle::WhenRequired,
        }
    }
}

impl From<&FormatterSettings> for PrinterSettings {
    fn from(value: &FormatterSettings) -> Self {
        Self {
            margin: value.max_width as isize,
            indent: value.tab_spaces as isize,
            min_space: 60,
            crlf_line_endings: true
        }
    }
}

pub struct Formatter<'a> {
    pub printer: &'a mut leptosfmt_pretty_printer::Printer,
    pub settings: FormatterSettings,
    line_offset: Option<usize>,
    start_line_offset: Option<usize>,
    comments: HashMap<usize, Option<&'a str>>,
}

impl<'a> Formatter<'a> {
    pub fn new(
        settings: FormatterSettings,
        printer: &'a mut Printer,
        comments: HashMap<usize, Option<&'a str>>,
    ) -> Self {
        Self {
            printer,
            settings,
            comments,
            line_offset: None,
            start_line_offset: None,
        }
    }

    pub fn with_source(
        settings: FormatterSettings,
        printer: &'a mut Printer,
        source: &'a str,
    ) -> Self {
        Self {
            printer,
            settings,
            comments: source
                .lines()
                .enumerate()
                .filter_map(|(i, l)| {
                    if l.trim().is_empty() {
                        Some((i, None))
                    } else {
                        l.split("//").nth(1).map(|l| (i, Some(l)))
                    }
                })
                .collect(),
            line_offset: None,
            start_line_offset: None,
        }
    }

    pub fn write_comments(&mut self, line_index: usize) {
        let last = self
            .line_offset
            .unwrap_or(self.start_line_offset.unwrap_or(0));

        let comments_or_empty_lines: Vec<_> = (last..=line_index)
            .filter_map(|l| self.comments.remove(&l))
            .collect();

        let mut prev_is_empty_line = false;

        for comment_or_empty in comments_or_empty_lines {
            if let Some(comment) = comment_or_empty {
                self.printer.word("//");
                self.printer.word(comment.to_string());
                self.printer.hardbreak();
                prev_is_empty_line = false;
            } else if self.line_offset.is_some() {
                // Do not print multiple consecutive empty lines
                if !prev_is_empty_line {
                    self.printer.hardbreak();
                }

                prev_is_empty_line = true;
            }
        }

        self.line_offset = Some(line_index);
    }
}
