use regex::Regex;
use std::any;
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::Hasher;
use wasm_bindgen::{prelude::*, JsCast};

thread_local! {
    static STYLED: RefCell<HashSet<u64>> = RefCell::new(HashSet::new());
    static SHEET: RefCell<Option<web_sys::CssStyleSheet>> = RefCell::new(None);
    static CLASS_SELECTER: Regex = Regex::new(r"\.([a-zA-Z][a-zA-Z0-9\-_]*)").unwrap();
}

fn hash_of_type<C>() -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write(any::type_name::<C>().as_bytes());
    hasher.finish()
}

fn styled_class_prefix<C>() -> String {
    let mut hasher = DefaultHasher::new();
    hasher.write(any::type_name::<C>().as_bytes());
    format!("{:X}", hasher.finish())
}

fn styled_class<C>(class_name: &str) -> String {
    format!("_{}__{}", styled_class_prefix::<C>(), class_name)
}

pub trait Styled: Sized {
    fn style() -> Style;
    fn styled<T>(node: T) -> T {
        STYLED.with(|styled| {
            let component_id = hash_of_type::<Self>();
            if styled.borrow().get(&component_id).is_none() {
                let style = Self::style();
                style.write::<Self>();
                styled.borrow_mut().insert(component_id);
            }
        });

        node
    }
    fn class(class_name: &str) -> String {
        styled_class::<Self>(class_name)
    }
}

#[derive(Clone, PartialEq)]
enum Rule {
    Selecter(String, Vec<(String, String)>),
    Keyframes(String, Style),
    Media(String, Style),
}

#[derive(Clone)]
pub struct Style {
    rules: Vec<Rule>,
}

impl Style {
    pub fn new() -> Self {
        Self { rules: vec![] }
    }

    pub fn add(
        &mut self,
        selector: impl Into<String>,
        property: impl Into<String>,
        value: impl Into<String>,
    ) {
        let selecter = selector.into();
        let property = property.into();
        let value = value.into();

        for rule in self.rules.iter_mut() {
            match rule {
                Rule::Selecter(s, defs) if *s == selecter => {
                    for (p, v) in defs.iter_mut() {
                        if *p == property {
                            *v = value;
                            return;
                        }
                    }
                    defs.push((property, value));
                    return;
                }
                _ => {}
            }
        }
        self.rules
            .push(Rule::Selecter(selecter, vec![(property, value)]));
    }

    pub fn add_keyframes(&mut self, name: impl Into<String>, style: Style) {
        let name = name.into();
        self.rules.push(Rule::Keyframes(name, style));
    }

    pub fn add_media(&mut self, query: impl Into<String>, style: Style) {
        let query = query.into();
        self.rules.push(Rule::Media(query, style));
    }

    pub fn append(&mut self, other: &Self) {
        for rule in &other.rules {
            match rule {
                Rule::Selecter(s, defs) => {
                    for (p, v) in defs {
                        self.add(s, p, v);
                    }
                }
                Rule::Keyframes(n, s) => {
                    self.add_keyframes(n, s.clone());
                }
                Rule::Media(q, s) => {
                    self.add_media(q, s.clone());
                }
            }
        }
    }

    fn rules<C>(&self) -> Vec<String> {
        let mut str_rules = vec![];

        for rule in self.rules.iter() {
            let str_rule = match rule {
                Rule::Selecter(selecter, defs) => {
                    let mut str_rule = String::new();
                    let str_selecter = CLASS_SELECTER.with(|class_selecter| {
                        class_selecter.replace_all(
                            selecter,
                            format!("._{}__$1", styled_class_prefix::<C>()).as_str(),
                        )
                    });
                    str_rule += &str_selecter;
                    str_rule += "{";
                    for (property, value) in defs {
                        str_rule += format!("{}:{};", property, value).as_str();
                    }
                    str_rule += "}";
                    str_rule
                }

                Rule::Keyframes(name, keyframes) => {
                    let mut str_rule = String::from("@keyframes ");
                    str_rule += name;
                    str_rule += "{";
                    for child_rule in &keyframes.rules::<C>() {
                        str_rule += child_rule;
                    }
                    str_rule += "}";

                    str_rule
                }

                Rule::Media(query, media_style) => {
                    let mut str_rule = String::from("@media ");
                    str_rule += query;
                    str_rule += "{";
                    for child_rule in &media_style.rules::<C>() {
                        str_rule += child_rule;
                    }
                    str_rule += "}";

                    str_rule
                }
            };

            str_rules.push(str_rule);
        }

        str_rules
    }

    fn write<C>(&self) {
        Self::add_style_element();

        for rule in &self.rules::<C>() {
            SHEET.with(|sheet| {
                if let Some(sheet) = sheet.borrow().as_ref() {
                    if let Err(err) = sheet
                        .insert_rule_with_index(rule.as_str(), sheet.css_rules().unwrap().length())
                    {
                        web_sys::console::log_1(&JsValue::from(err));
                    }
                }
            });
        }
    }

    fn add_style_element() {
        SHEET.with(|sheet| {
            if sheet.borrow().is_none() {
                let style_element = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .create_element("style")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlStyleElement>()
                    .unwrap();

                let head = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_elements_by_tag_name("head")
                    .item(0)
                    .unwrap();

                let _ = head.append_child(&style_element);

                *sheet.borrow_mut() = Some(
                    style_element
                        .sheet()
                        .unwrap()
                        .dyn_into::<web_sys::CssStyleSheet>()
                        .unwrap(),
                );
            }
        });
    }
}

macro_rules! return_if {
    ($x:ident = $y:expr; $z: expr) => {{
        let $x = $y;
        if $z {
            return $x;
        }
    }};
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rule in &self.rules {
            match rule {
                Rule::Selecter(selecter, defs) => {
                    return_if!(x = write!(f, "{} {}\n", selecter, "{"); x.is_err());
                    for (property, value) in defs {
                        return_if!(x = write!(f, "    {}: {};\n", property, value); x.is_err());
                    }
                    return_if!(x = write!(f, "{}\n", "}"); x.is_err());
                }

                Rule::Keyframes(name, keyframes) => {
                    return_if!(x = write!(f, "@keyframes {} {}\n", name, "{"); x.is_err());

                    for a_line in format!("{}", keyframes).split("\n") {
                        if a_line != "" {
                            return_if!(x = write!(f, "    {}\n", a_line); x.is_err());
                        }
                    }

                    return_if!(x = write!(f, "{}\n", "}"); x.is_err());
                }

                Rule::Media(query, style) => {
                    return_if!(x = write!(f, "@media {} {}\n", query, "{"); x.is_err());

                    for a_line in format!("{}", style).split("\n") {
                        if a_line != "" {
                            return_if!(x = write!(f, "    {}\n", a_line); x.is_err());
                        }
                    }

                    return_if!(x = write!(f, "{}\n", "}"); x.is_err());
                }
            }
        }

        write!(f, "")
    }
}

impl PartialEq for Style {
    fn eq(&self, other: &Self) -> bool {
        self.rules.eq(&other.rules)
    }
}

#[macro_export]
macro_rules! style {

    {
        instance: $inst:ident;
        @charset $import:expr;
        $($others:tt)*
    } => {{
        $inst.append(&($import));

        style! {
            instance: $inst;
            $($others)*
        }
    }};

    {
        instance: $inst:ident;
        @extends $extends:expr;
        $($others:tt)*
    } => {{
        $inst.append(&($extends));

        style! {
            instance: $inst;
            $($others)*
        }
    }};

    {
        instance: $inst:ident;
        @keyframes $name:tt {$($keyframes:tt)*}
        $($others:tt)*
    } => {{
        $inst.add_keyframes($name, style!{$($keyframes)*});

        style! {
            instance: $inst;
            $($others)*
        }
    }};

    {
        instance: $inst:ident;
        @media $query:tt {$($media_style:tt)*}
        $($others:tt)*
    } => {{
        $inst.add_media($query, style!{$($media_style)*});

        style! {
            instance: $inst;
            $($others)*
        }
    }};

    {
        instance: $inst:ident;
        $selector:literal {$(
            $property:tt : $value:expr;
        )*}
        $($others:tt)*
    } => {{
        $(
            $inst.add(format!("{}", $selector), format!("{}", $property), format!("{}", $value));
        )*

        style! {
            instance: $inst;
            $($others)*
        }
    }};

    {
        instance: $inst:ident;
    } => {{}};

    {
        $($others:tt)*
    } => {{
        #[allow(unused_mut)]
        let mut instance = Style::new();

        style! {
            instance: instance;
            $($others)*
        };

        instance
    }};
}

#[cfg(test)]
mod tests {
    use super::Rule;
    use super::Style;

    #[test]
    fn it_works() {
        assert!(true);
    }

    #[test]
    fn debug_style() {
        let style = Style {
            rules: vec![
                Rule::Selecter(
                    String::from("foo"),
                    vec![
                        (String::from("width"), String::from("100px")),
                        (String::from("height"), String::from("100px")),
                    ],
                ),
                Rule::Selecter(
                    String::from("bar"),
                    vec![
                        (String::from("width"), String::from("100px")),
                        (String::from("height"), String::from("100px")),
                    ],
                ),
            ],
        };

        let style_str = concat!(
            "foo {\n",
            "    width: 100px;\n",
            "    height: 100px;\n",
            "}\n",
            "bar {\n",
            "    width: 100px;\n",
            "    height: 100px;\n",
            "}\n",
        );

        assert_eq!(format!("{:?}", style), style_str);
    }

    #[test]
    fn debug_style_with_media() {
        let media_style = Style {
            rules: vec![
                Rule::Selecter(
                    String::from("foo"),
                    vec![
                        (String::from("width"), String::from("100px")),
                        (String::from("height"), String::from("100px")),
                    ],
                ),
                Rule::Selecter(
                    String::from("bar"),
                    vec![
                        (String::from("width"), String::from("100px")),
                        (String::from("height"), String::from("100px")),
                    ],
                ),
            ],
        };
        let style = Style {
            rules: vec![
                Rule::Selecter(
                    String::from("foo"),
                    vec![
                        (String::from("width"), String::from("100px")),
                        (String::from("height"), String::from("100px")),
                    ],
                ),
                Rule::Selecter(
                    String::from("bar"),
                    vec![
                        (String::from("width"), String::from("100px")),
                        (String::from("height"), String::from("100px")),
                    ],
                ),
                Rule::Media(String::from("query"), media_style),
            ],
        };

        let style_str = concat!(
            "foo {\n",
            "    width: 100px;\n",
            "    height: 100px;\n",
            "}\n",
            "bar {\n",
            "    width: 100px;\n",
            "    height: 100px;\n",
            "}\n",
            "@media query {\n",
            "    foo {\n",
            "        width: 100px;\n",
            "        height: 100px;\n",
            "    }\n",
            "    bar {\n",
            "        width: 100px;\n",
            "        height: 100px;\n",
            "    }\n",
            "}\n",
        );

        assert_eq!(format!("{:?}", style), style_str);
    }

    #[test]
    fn gen_style_by_manual() {
        let style_a = Style {
            rules: vec![
                Rule::Selecter(
                    String::from("foo"),
                    vec![
                        (String::from("width"), String::from("100px")),
                        (String::from("height"), String::from("100px")),
                    ],
                ),
                Rule::Selecter(
                    String::from("bar"),
                    vec![
                        (String::from("width"), String::from("100px")),
                        (String::from("height"), String::from("100px")),
                    ],
                ),
            ],
        };

        let mut style_b = Style::new();
        style_b.add("foo", "width", "100px");
        style_b.add("foo", "height", "100px");
        style_b.add("bar", "width", "100px");
        style_b.add("bar", "height", "100px");

        assert_eq!(style_a, style_b);
    }

    #[test]
    fn gen_style_with_media_by_manual() {
        let media_style_a = Style {
            rules: vec![
                Rule::Selecter(
                    String::from("foo"),
                    vec![
                        (String::from("width"), String::from("100px")),
                        (String::from("height"), String::from("100px")),
                    ],
                ),
                Rule::Selecter(
                    String::from("bar"),
                    vec![
                        (String::from("width"), String::from("100px")),
                        (String::from("height"), String::from("100px")),
                    ],
                ),
            ],
        };
        let style_a = Style {
            rules: vec![
                Rule::Selecter(
                    String::from("foo"),
                    vec![
                        (String::from("width"), String::from("100px")),
                        (String::from("height"), String::from("100px")),
                    ],
                ),
                Rule::Selecter(
                    String::from("bar"),
                    vec![
                        (String::from("width"), String::from("100px")),
                        (String::from("height"), String::from("100px")),
                    ],
                ),
                Rule::Media(String::from("query"), media_style_a),
            ],
        };

        let mut media_style_b = Style::new();
        media_style_b.add("foo", "width", "100px");
        media_style_b.add("foo", "height", "100px");
        media_style_b.add("bar", "width", "100px");
        media_style_b.add("bar", "height", "100px");
        let mut style_b = Style::new();
        style_b.add("foo", "width", "100px");
        style_b.add("foo", "height", "100px");
        style_b.add("bar", "width", "100px");
        style_b.add("bar", "height", "100px");
        style_b.add_media("query", media_style_b);

        assert_eq!(style_a, style_b);
    }

    #[test]
    fn gen_style_by_macro() {
        let mut style_a = Style::new();
        style_a.add("foo", "width", "100px");
        style_a.add("foo", "height", "100px");
        style_a.add("bar", "width", "100px");
        style_a.add("bar", "height", "100px");

        let style_b = style! {
            "foo" {
                "width": "100px";
                "height": "100px";
            }

            "bar" {
                "width": "100px";
                "height": "100px";
            }
        };

        assert_eq!(style_a, style_b);
    }

    #[test]
    fn gen_style_with_media_by_macro() {
        let mut media_style_a = Style::new();
        media_style_a.add("foo", "width", "100px");
        media_style_a.add("foo", "height", "100px");
        media_style_a.add("bar", "width", "100px");
        media_style_a.add("bar", "height", "100px");
        let mut style_a = Style::new();
        style_a.add("foo", "width", "100px");
        style_a.add("foo", "height", "100px");
        style_a.add("bar", "width", "100px");
        style_a.add("bar", "height", "100px");
        style_a.add_media("query", media_style_a);

        let style_b = style! {
            "foo" {
                "width": "100px";
                "height": "100px";
            }

            "bar" {
                "width": "100px";
                "height": "100px";
            }

            @media "query" {
                "foo" {
                    "width": "100px";
                    "height": "100px";
                }

                "bar" {
                    "width": "100px";
                    "height": "100px";
                }
            }
        };

        assert_eq!(style_a, style_b);
    }
}
