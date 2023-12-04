use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error::proc_macro_error;
use quote::{format_ident, quote};
use rstml::{
    node::{Node, NodeAttribute, NodeBlock},
    Parser, ParserConfig,
};
use syn::{
    parse_macro_input, spanned::Spanned, Block, Expr, ExprCast, ExprForLoop, ExprIf, ExprMatch,
    Stmt, Type,
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
    .into()
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
                        let mut template = ::hirola::prelude::GenericNode::element(#name);
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

fn some_kind_of_uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn attribute_to_tokens(attribute: &NodeAttribute) -> TokenStream {
    match attribute {
        NodeAttribute::Block(block) => quote! {
            #block
        }
        .into(),
        NodeAttribute::Attribute(attr) => {
            let name = attr.key.to_string();
            let value = attr.value();
            if name.starts_with("on:") {
                let name = name.replace("on:", "");
                quote! {
                    ::hirola::prelude::EventListener::event(&mut template, #name, #value);
                }
            } else if name.starts_with("use:") {
                let effect = if value.is_some() {
                    quote! {
                        #value
                    }
                } else {
                    let cleaned_name = Ident::new(&name.replace("use:", ""), Span::call_site());
                    quote! {
                        #cleaned_name
                    }
                };
                quote! {
                    ::hirola::prelude::GenericNode::effect(
                        &template,
                        #effect
                    );

                }
            } else if name.starts_with("mixin:") || name.starts_with("x:") {
                let name_space = name.replace("mixin:", "").replace("x:", "");
                let ns_struct =
                    format_ident!("{}", &some_kind_of_uppercase_first_letter(&name_space));
                quote! {
                    hirola::prelude::Mixin::<#ns_struct, _>::mixin(#value, &template);
                }
            } else if &name == "ref" {
                quote! {
                    let _ = ::hirola::prelude::NodeReference::set(
                        &#value,
                        ::std::clone::Clone::clone(&template),
                    );

                }
            } else if name.starts_with("bind:") {
                let attribute_name = convert_name(&name).replace("bind:", "");
                quote! {
                {
                        use hirola::signal::SignalExt;
                        let template_clone = ::std::clone::Clone::clone(&template);
                        ::hirola::prelude::GenericNode::set_attribute(
                            &template,
                            #attribute_name,
                            &::std::format!("{}", #value.get_cloned()),
                        );
                        ::hirola::prelude::GenericNode::effect(&template, #value.signal_ref(move |value| {
                            ::hirola::prelude::GenericNode::set_attribute(
                                &template_clone,
                                #attribute_name,
                                &::std::format!("{}", value),
                            );
                        }).to_future());
                }

                }
            } else {
                let attribute_name = convert_name(&name);
                quote! {
                    ::hirola::prelude::GenericNode::set_attribute(
                        &mut template,
                        #attribute_name,
                        &::std::format!("{}", #value),
                    );
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
                                ::hirola::prelude::GenericNode::append_render(&mut template, #node );
                            });
                        }
                        _ => {
                            append_children.extend(quote! {
                                ::hirola::prelude::GenericNode::append_child(&mut template, &#node );
                            });
                        }
                    }
                }
                Node::Text(text) => {
                    append_children.extend(quote! {
                        ::hirola::prelude::GenericNode::append_child(
                            &mut template,
                            #[allow(unused_braces)]
                            &::hirola::prelude::GenericNode::text_node(#text),
                        );
                    });
                }
                Node::Comment(comment) => {
                    let s = comment.value.clone();
                    append_children.extend(quote! {
                        ::hirola::prelude::GenericNode::append_child(
                            &mut template,
                            #[allow(unused_braces)]
                            &::hirola::prelude::GenericNode::comment(#s),
                        );
                    });
                }
                Node::Doctype(_) => {}
                Node::Block(block) => match block {
                    NodeBlock::ValidBlock(block) => match braced_for_control(&block) {
                        Some(Control::ExprForLoop(ExprForLoop {
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
                                    &Type::Path(ref path) => {
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
                                            &mut template,
                                            &#body,
                                        );
                                    }
                                });
                            }
                        }
                        Some(Control::ExprIf(ExprIf {
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
                                            let mut template = {
                                                let switch = ::hirola::prelude::Switch {
                                                    signal: #expr,
                                                    renderer: |res| {
                                                        if res {
                                                            #then_branch
                                                        } else {
                                                            #else_branch
                                                        }
                                                    }
                                                };
                                                Box::new(switch)
                                            };
                                        });
                                    }
                                    &Type::Path(ref path) => {
                                        let ident = Ident::new("Signal", Span::call_site());
                                        if path.path.is_ident(&ident) {
                                            append_children.extend(quote! {
                                                let mut template = {
                                                    let switch = ::hirola::prelude::Switch {
                                                        signal: #expr,
                                                        renderer: |res| {
                                                            if res {
                                                                #then_branch
                                                            } else {
                                                                #else_branch
                                                            }
                                                        }
                                                    };
                                                    Box::new(switch)
                                                };
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
                                        &mut template,
                                        #[allow(unused_braces)]
                                        #block,
                                    );
                                });
                            }
                        }

                        Some(Control::ExprMatch(ExprMatch { expr, arms, .. })) => match *expr {
                            Expr::Await(fut) => {
                                let fut = fut.base;
                                append_children.extend(quote! {
                                    let suspense = {

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
                                        &mut template,
                                        suspense
                                    );

                                });
                            }
                            _ => {
                                append_children.extend(quote! {
                                    ::hirola::prelude::GenericNode::append_child(
                                        &mut template,
                                        #[allow(unused_braces)]
                                        &#block,
                                    );
                                });
                            }
                        },
                        _ => {
                            append_children.extend(quote! {
                                let _ = ::hirola::prelude::Render::render_into(
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
    ExprForLoop(ExprForLoop),
    ExprIf(ExprIf),
    ExprMatch(ExprMatch),
}

fn braced_for_control(block: &Block) -> Option<Control> {
    let len = block.stmts.len();
    if len != 1 {
        None
    } else {
        let stmt = &block.stmts[0];
        match stmt {
            Stmt::Expr(Expr::ForLoop(expr), _) => Some(Control::ExprForLoop(expr.clone())),
            Stmt::Expr(Expr::If(expr), _) => Some(Control::ExprIf(expr.clone())),
            Stmt::Expr(Expr::Match(expr), _) => Some(Control::ExprMatch(expr.clone())),
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
