use heck::{ToPascalCase, ToUpperCamelCase};
use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error::proc_macro_error;
use quote::{format_ident, quote};
use rstml::{
    node::{Node, NodeAttribute, NodeBlock},
    Parser, ParserConfig,
};
use syn::{
    parse_macro_input, spanned::Spanned, Block, Data, DeriveInput, Expr, ExprCast, ExprForLoop,
    ExprIf, ExprMatch, Fields, ItemFn, Stmt, Type,
};

mod component;

fn to_token_stream(input: proc_macro::TokenStream) -> TokenStream {
    let config = ParserConfig::default().recover_block(true);
    let parser = Parser::new(config);
    let (mut nodes, errors) = parser.parse_recoverable(input).split_vec();
    let errors = errors.into_iter().map(|e| e.emit_as_expr_tokens());
    let nodes_output = if nodes.len() == 1 {
        let node = nodes.pop().expect("unable to convert node to tokens");
        node_to_tokens(node)
    } else {
        fragment_to_tokens(nodes)
    };
    quote! {
        {
            #(#errors;)*
            #nodes_output
        }
    }
}

fn fragment_to_tokens(nodes: Vec<Node>) -> TokenStream {
    let mut tokens = TokenStream::new();
    let children_tokens = children_to_tokens(nodes);
    tokens.extend(quote! {
            {
                let mut template =  ::hirola::prelude::GenericNode::fragment();
                #children_tokens
                template
            }
    });
    tokens
}

fn node_to_tokens(node: Node) -> TokenStream {
    let mut tokens = TokenStream::new();

    match node {
        Node::Element(node) => {
            let name = node.name().to_string();
            if name[0..1].to_lowercase() == name[0..1] {
                let attributes = node.attributes().iter().map(attribute_to_tokens);

                let children_tokens = children_to_tokens(node.children.clone());

                tokens.extend(quote! {
                    {
                        let template = ::hirola::prelude::GenericNode::element(#name);
                        #children_tokens
                        #(#attributes)*
                        template
                     }
                });
            } else {
                let fnname: Ident = syn::parse_str(&name).unwrap();

                let mut attributes = node
                    .attributes()
                    .iter()
                    .map(|attribute| match &attribute {
                        NodeAttribute::Block(expr) => {
                            quote! {
                                #expr
                            }
                        }
                        NodeAttribute::Attribute(attr) => {
                            let key = &attr.key;
                            let value = &attr.value();
                            quote! {
                                #key : #value
                            }
                        }
                    })
                    .collect::<Vec<TokenStream>>();
                if !node.children.is_empty() {
                    let children_tokens = children_to_tokens(node.children);
                    attributes.extend(vec![quote! {
                        children: {
                            Box::new(#children_tokens)
                         }
                    }]);
                }

                let quoted = if attributes.is_empty() {
                    quote!(#fnname)
                } else {
                    quote!(#fnname {#(#attributes),*})
                };
                tokens.extend(quote! {
                    #quoted
                });
            }
        }
        Node::Fragment(fragment) => tokens.extend(fragment_to_tokens(fragment.children)),
        _ => {}
    }
    tokens
}


fn attribute_to_tokens(attribute: &NodeAttribute) -> TokenStream {
    match attribute {
        NodeAttribute::Block(block) => quote! {
            #block
        },
        NodeAttribute::Attribute(attr) => {
            let name = attr.key.to_string();
            let value = attr.value();
            let parts: Vec<&str> = name.split(':').collect();
            if parts.len() == 2 {
                let name_space =
                    format_ident!("{}Effect", &parts[0].to_pascal_case());
                let attr = &parts[1].to_pascal_case();
                let attr_space = format_ident!("{}", attr);
                quote! {
                    ::hirola::prelude::SideEffect::effect(&#name_space, &template, #attr_space, #value);
                }
            } else {
                let attribute_name = convert_name(&name);
                let name_space = format_ident!("DefaultAttributeEffect");
                quote! {

                    ::hirola::prelude::SideEffect::effect(&#name_space, &template, DefaultAttrStr(#attribute_name), #value);
                }
            }
        }
    }
}

fn children_to_tokens(children: Vec<Node>) -> TokenStream {
    let mut append_children = Vec::new();
    let mut tokens = TokenStream::new();
    if !children.is_empty() {
        for child in children {
            match &child {
                Node::Element(element) => {
                    let node = node_to_tokens(child.clone());
                    let name = element.name().to_string();

                    match child {
                        // Its a component
                        Node::Element(_) if name[0..1].to_lowercase() != name[0..1] => {
                            append_children.extend(quote! {
                                #[allow(unused_braces)]
                                ::hirola::prelude::GenericNode::append_render(&template, #node );
                            });
                        }
                        _ => {
                            append_children.extend(quote! {
                                #[allow(unused_braces)]
                                ::hirola::prelude::GenericNode::append_child(&template, &#node );
                            });
                        }
                    }
                }
                Node::Text(text) => {
                    append_children.extend(quote! {
                        ::hirola::prelude::GenericNode::append_child(
                            &template,
                            #[allow(unused_braces)]
                            &::hirola::prelude::GenericNode::text_node(#text),
                        );
                    });
                }
                Node::Comment(comment) => {
                    let s = comment.value.clone();
                    append_children.extend(quote! {
                        ::hirola::prelude::GenericNode::append_child(
                            &template,
                            #[allow(unused_braces)]
                            &::hirola::prelude::GenericNode::comment(#s),
                        );
                    });
                }
                Node::Doctype(_) => {}
                Node::Block(block) => match block {
                    NodeBlock::ValidBlock(block) => match braced_for_control(block) {
                        Some(Control::ForLoop(ExprForLoop {
                            pat, expr, body, ..
                        })) => {
                            if let Expr::Cast(ExprCast { ty, expr, .. }) = expr.as_ref() {
                                match ty.as_ref() {
                                    &Type::Infer(_) => {
                                        append_children.extend(quote! {
                                            let template = {
                                                let props = ::hirola::prelude::IndexedProps {
                                                    iterable: #expr,
                                                    template: move | #pat | {
                                                        #body
                                                    }
                                                };
                                                let indexed = ::hirola::prelude::Indexed {
                                                    props
                                                };
                                                Box::new(indexed)
                                            };
                                        });
                                    }
                                    Type::Path(path) => {
                                        let ident = Ident::new("SignalVec", Span::call_site());
                                        if path.path.is_ident(&ident) {
                                            append_children.extend(quote! {
                                                let template = {
                                                    let props = ::hirola::prelude::IndexedProps {
                                                        iterable: #expr,
                                                        template: move | #pat | {
                                                            #body
                                                        }
                                                    };
                                                    let indexed = ::hirola::prelude::Indexed {
                                                        props
                                                    };
                                                    Box::new(indexed)
                                                };
                                            });
                                        } else {
                                            append_children.extend(
                                                syn::Error::new(
                                                    ty.span(),
                                                    "expected SignalVec or _",
                                                )
                                                .to_compile_error(),
                                            );
                                        }
                                    }
                                    _ => {
                                        append_children.extend(
                                            syn::Error::new(ty.span(), "expected SignalVec or _")
                                                .to_compile_error(),
                                        );
                                    }
                                }
                            } else {
                                append_children.extend(quote! {
                                    for #pat in #expr {
                                        ::hirola::prelude::GenericNode::append_child(
                                            &template,
                                            #[allow(unused_braces)]
                                            &#body,
                                        );
                                    }
                                });
                            }
                        }
                        Some(Control::If(ExprIf {
                            cond,
                            then_branch,
                            else_branch,
                            ..
                        })) => {
                            let (_, else_branch) = else_branch.unwrap();

                            if let Expr::Cast(ExprCast { ty, expr, .. }) = cond.as_ref() {
                                match ty.as_ref() {
                                    &Type::Infer(_) => {
                                        append_children.extend(quote! {
                                            #[allow(unused_braces)]
                                            let switch = {
                                                let switch = ::hirola::prelude::Switch {
                                                    signal: #expr,
                                                    renderer: move |res| {
                                                        if res {
                                                            #then_branch
                                                        } else {
                                                            #else_branch
                                                        }
                                                    }
                                                };
                                                switch
                                            };
                                            ::hirola::prelude::GenericNode::append_render(
                                                &template,
                                                switch
                                            );
                                        });
                                    }
                                    Type::Path(path) => {
                                        let ident = Ident::new("Signal", Span::call_site());
                                        if path.path.is_ident(&ident) {
                                            append_children.extend(quote! {
                                                let switch = {
                                                    #[allow(unused_braces)]
                                                    let switch = ::hirola::prelude::Switch {
                                                        signal: #expr,
                                                        renderer: move |res| {
                                                            if res {
                                                                #then_branch
                                                            } else {
                                                                #else_branch
                                                            }
                                                        }
                                                    };
                                                    switch
                                                };
                                                ::hirola::prelude::GenericNode::append_render(
                                                    &template,
                                                    switch
                                                );
                                            });
                                        } else {
                                            append_children.extend(
                                                syn::Error::new(ty.span(), "expected Signal or _")
                                                    .to_compile_error(),
                                            );
                                        }
                                    }
                                    _ => {
                                        append_children.extend(
                                            syn::Error::new(
                                                ty.span(),
                                                "expected Signal, SignalVec or _",
                                            )
                                            .to_compile_error(),
                                        );
                                    }
                                }
                            } else {
                                append_children.extend(quote! {
                                    ::hirola::prelude::GenericNode::append_child(
                                        &template,
                                        #[allow(unused_braces)]
                                        &#block,
                                    );
                                });
                            }
                        }

                        Some(Control::Match(ExprMatch { expr, arms, .. })) => match *expr {
                            Expr::Await(fut) => {
                                let fut = fut.base;
                                append_children.extend(quote! {
                                    let suspense = {
                                        #[allow(unused_braces)]
                                        let suspense = ::hirola::prelude::Suspense {
                                            future: Box::pin(#fut),
                                            template: Box::new(move |res| {
                                                match res {
                                                    #(#arms)*
                                                }
                                            })
                                        };
                                        suspense
                                    };
                                    ::hirola::prelude::GenericNode::append_render(
                                        &template,
                                        suspense
                                    );

                                });
                            }
                            _ => {
                                append_children.extend(quote! {
                                    ::hirola::prelude::GenericNode::append_child(
                                        &template,
                                        #[allow(unused_braces)]
                                        &#block,
                                    );
                                });
                            }
                        },
                        _ => {
                            append_children.extend(quote! {
                                let _ = ::hirola::prelude::Render::render_into(
                                    #[allow(unused_braces)]
                                    Box::new(#block),
                                    &template,
                                );
                            });
                        }
                    },
                    NodeBlock::Invalid { body, .. } => {
                        return syn::Error::new(body.span(), "Invalid block").to_compile_error()
                    }
                },
                _ => {
                    return quote! {
                        compile_error!(format!("Unexpected NodeType for child: {}", child.node_type))
                    }
                }
            }
        }
    }

    let quoted = quote! {
        #(#append_children)*

    };
    tokens.extend(quoted);
    tokens
}

enum Control {
    ForLoop(ExprForLoop),
    If(ExprIf),
    Match(ExprMatch),
}

fn braced_for_control(block: &Block) -> Option<Control> {
    let len = block.stmts.len();
    if len != 1 {
        None
    } else {
        let stmt = &block.stmts[0];
        match stmt {
            Stmt::Expr(Expr::ForLoop(expr), _) => Some(Control::ForLoop(expr.clone())),
            Stmt::Expr(Expr::If(expr), _) => Some(Control::If(expr.clone())),
            Stmt::Expr(Expr::Match(expr), _) => Some(Control::Match(expr.clone())),
            _ => None,
        }
    }
}

fn convert_name(name: &str) -> String {
    let mut out = String::with_capacity(name.len());

    for c in name.trim_matches('_').chars() {
        match c {
            '_' => out.push('-'),
            c => out.push(c),
        }
    }

    out
}

#[proc_macro]
pub fn html(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output = to_token_stream(input);

    let quoted = quote! {
        #output
    };
    quoted.into()
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn component(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let f = parse_macro_input!(item as syn::ItemFn);
    component::create_function_component(f)
}

#[proc_macro_attribute]
pub fn mixin(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let struct_name = format_ident!(
        "{}",
        &input_fn.sig.ident.to_string().to_pascal_case()
    );
    let raw_struct_name = struct_name.to_string();
    // Generate the additional struct and impl
    let expanded = quote! {
        #input_fn

        pub struct #struct_name;

        impl EffectAttribute for #struct_name {
            type Handler = XEffect; //TODO: Make this input
            fn read_as_attr(&self) -> String {
                #raw_struct_name.to_string()
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(FormEntity)]
pub fn fields_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let enum_name = syn::Ident::new(&format!("{}Form", name), name.span());

    let fields = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .map(|f| (&f.ident, &f.ty))
                .collect::<Vec<_>>(),
            _ => Vec::new(),
        },
        _ => Vec::new(),
    };

    let match_arms = fields.iter().map(|(f, _)| {
        let variant_name = f.as_ref().unwrap().to_string().to_upper_camel_case();
        let variant_name = format_ident!("{variant_name}");
        let field_name_str = f.as_ref().unwrap().to_string();
        quote! {
            #enum_name::#variant_name => #field_name_str
        }
    });

    // Implement Into<BTreeMap> for struct
    let into_btreemap = fields.iter().map(|(ident, _)| {
        let variant = syn::Ident::new(
            &ident.as_ref().unwrap().to_string().to_upper_camel_case(),
            ident.span(),
        );
        quote! {
            btreemap.insert(#enum_name::#variant, self.#ident.clone());
        }
    });

    // Implement From<BTreeMap> for struct
    let from_btreemap = fields.iter().map(|(ident, _)| {
        let variant = syn::Ident::new(
            &ident.as_ref().unwrap().to_string().to_upper_camel_case(),
            ident.span(),
        );
        quote! {
            #ident: btreemap.get(&#enum_name::#variant).ok_or("Field not found").unwrap().clone()
        }
    });
    let variants = fields.iter().map(|(f, _)| {
        let variant_name = f.as_ref().unwrap().to_string();
        let variant_name = variant_name.to_upper_camel_case();
        let variant_name = format_ident!("{variant_name}");
        quote! { #variant_name }
    });

    let from_str_match_arms = fields.iter().map(|(ident, _)| {
        let variant_str = ident.as_ref().unwrap().to_string().to_string();
        let variant_name = variant_str.to_upper_camel_case();
        let variant_name = format_ident!("{variant_name}");
        quote! { #variant_str => Ok(#enum_name::#variant_name) }
    });

    let gen = quote! {
        impl hirola_form::FormEntity for #name {
            type Columns = #enum_name;
        }

        #[derive(Debug, Copy, Clone, Ord, Eq, PartialEq, PartialOrd, Hash)]
        pub enum #enum_name {
            #(#variants),*
        }
        impl hirola_form::FormColumn for #enum_name {
            fn name(&self) -> &str {
                match self {
                    #(#match_arms),*
                }
            }
        }
        impl From<std::collections::BTreeMap<#enum_name, String>> for #name {
            fn from(mut btreemap: std::collections::BTreeMap<#enum_name, String>) -> Self {
                #name {
                    #(#from_btreemap),*
                }
            }
        }

        impl Into<std::collections::BTreeMap<#enum_name, String>> for #name {
            fn into(self) -> std::collections::BTreeMap<#enum_name, String> {
                let mut btreemap = std::collections::BTreeMap::new();
                #(#into_btreemap)*
                btreemap
            }
        }
        impl std::str::FromStr for #enum_name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #( #from_str_match_arms, )*
                    _ => Err(format!("Invalid value for {}: {}", stringify!(#enum_name), s)),
                }
            }
        }
    };

    gen.into()
}
