use std::collections::HashMap;

use leptosfmt_pretty_printer::Printer;
use proc_macro2::Span;
use rstml::node::Node;
use syn::{spanned::Spanned, Macro};

use super::{Formatter, FormatterSettings};

pub struct HtmlMacro<'a> {
    pub parent_ident: Option<usize>,
    pub nodes: Vec<Node>,
    pub span: Span,
    pub mac: &'a Macro,
}

impl<'a> HtmlMacro<'a> {
    pub fn try_parse(parent_ident: Option<usize>, mac: &'a Macro) -> Option<Self> {
        let tokens = mac.tokens.clone().into_iter();
        let nodes = rstml::parse2(tokens.collect()).ok()?;

        Some(Self {
            parent_ident,
            nodes,
            span: mac.span(),
            mac,
        })
    }

    pub fn inner(&self) -> &Macro {
        self.mac
    }
}

impl Formatter<'_> {
    pub fn html_macro(&mut self, dom_mac: &HtmlMacro) {
        let HtmlMacro {
            parent_ident,
            nodes,
            span,
            ..
        } = dom_mac;

        self.start_line_offset = Some(span.start().line - 1);

        let indent = parent_ident
            .map(|i| i + self.settings.tab_spaces)
            .unwrap_or(0);

        self.printer.cbox(indent as isize);

        self.printer.word("html! {");
        self.dom_macro_nodes(nodes);
        self.printer.word("}");
        self.printer.end();
    }

    fn dom_macro_nodes(&mut self, nodes: &[Node]) {
        self.printer.cbox_indent();
        self.printer.space();

        let mut iter = nodes.iter().peekable();
        while let Some(node) = iter.next() {
            self.node(node);

            if iter.peek().is_some() {
                self.printer.hardbreak();
            }
        }

        self.printer.space();
        self.printer.end_dedent();
    }
}

pub fn format_macro(mac: &HtmlMacro, settings: &FormatterSettings, source: Option<&str>) -> String {
    let mut printer = Printer::new(settings.into());
    let mut formatter = match source {
        Some(source) => Formatter::with_source(*settings, &mut printer, source),
        None => Formatter::new(*settings, &mut printer, HashMap::new()),
    };

    formatter.html_macro(mac);
    printer.eof()
}

#[cfg(test)]
mod tests {
    use super::format_macro;
    use super::HtmlMacro;
    use quote::quote;
    use syn::Macro;

    macro_rules! dom_macro {
        ($($tt:tt)*) => {{
            let mac: Macro = syn::parse2(quote! { $($tt)* }).unwrap();
            format_macro(&HtmlMacro::try_parse(None, &mac).unwrap(), &Default::default(), None)
        }}
    }

    #[test]
    fn one_liner() {
        let formatted = dom_macro!(html! { <div>"hi"</div> });
        insta::assert_snapshot!(formatted, @r###"html! { <div>"hi"</div> }"###);
    }

    #[test]
    fn with_nested_nodes() {
        let formatted = dom_macro!(html! { <div><span>"hi"</span></div> });
        insta::assert_snapshot!(formatted, @r###"
        html! {
            <div>
                <span>"hi"</span>
            </div>
        }
        "###);
    }
}
