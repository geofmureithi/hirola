use std::collections::HashMap;

use leptosfmt_prettyplease::MacroFormatter;

use crate::formatter::format::{Formatter, FormatterSettings, HtmlMacro};

pub struct HtmlMacroFormatter<'a> {
    settings: FormatterSettings,
    comments: HashMap<usize, Option<&'a str>>,
}

impl<'a> HtmlMacroFormatter<'a> {
    pub fn new(settings: FormatterSettings, comments: HashMap<usize, Option<&'a str>>) -> Self {
        Self { settings, comments }
    }
}

impl MacroFormatter for HtmlMacroFormatter<'_> {
    fn format(&self, printer: &mut leptosfmt_pretty_printer::Printer, mac: &syn::Macro) -> bool {
        if !mac.path.is_ident("html") {
            return false;
        }

        let Some(m) = HtmlMacro::try_parse(None, mac) else { return false; };

        let mut formatter = Formatter::new(self.settings, printer, self.comments.clone());
        formatter.html_macro(&m);
        true
    }
}
