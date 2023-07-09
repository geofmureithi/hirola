mod component;
use proc_macro2::{Span, TokenStream};
use proc_macro_error::proc_macro_error;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input,
    spanned::Spanned,
    token::{Brace, Paren},
    Expr, ExprBlock, ExprCast, ExprForLoop, ExprIf, Ident, Stmt, Token, Type, TypeInfer, ExprMatch,
};
use syn_rsx::{Node, NodeType, ParserConfig};

fn to_token_stream(input: proc_macro::TokenStream) -> TokenStream {
    match syn_rsx::parse_with_config(input, ParserConfig::new()) {
        Ok(mut nodes) => {
            if nodes.len() == 1 {
                let node = nodes.pop().expect("unable to convert node to tokens");
                node_to_tokens(node)
            } else {
                fragment_to_tokens(nodes)
            }
        }
        Err(error) => error.to_compile_error(),
    }
}

fn fragment_to_tokens(nodes: Vec<Node>) -> TokenStream {
    let mut tokens = TokenStream::new();
    let children_tokens = children_to_tokens(nodes);
    tokens.extend(quote! {
            {

                let template =  ::hirola::prelude::ViewBuilder::fragment();
                #children_tokens
                template
            }

    });
    tokens
}

fn node_to_tokens(node: Node) -> TokenStream {
    let mut tokens = TokenStream::new();

    // NodeType::Element nodes can't have no name
    let name = node.name_as_string();

    if let Some(name) = name {
        if name[0..1].to_lowercase() == name[0..1] {
            let attributes = node.attributes.iter().map(attribute_to_tokens);

            let children_tokens = children_to_tokens(node.children);

            tokens.extend(quote! {
            {
                let mut template: ::hirola::prelude::ViewBuilder<::hirola::prelude::DomNode> = ::hirola::prelude::ViewBuilder::element(#name);
                #children_tokens
                #(#attributes)*
                template
             }
        });
        } else {
            let fnname: Ident = syn::parse_str(&name).unwrap();

            let mut attributes = node
                .attributes
                .iter()
                .map(|attribute| match &attribute.value {
                    Some(expr) => {
                        let ident: proc_macro2::TokenStream =
                            attribute.name_as_string().unwrap().parse().unwrap();
                        quote! {
                            #ident: #expr
                        }
                    }
                    None => quote! {},
                })
                .collect::<Vec<TokenStream>>();
            if !node.children.is_empty() {
                let children_tokens = children_to_tokens(node.children);
                attributes.extend(vec![quote! {
                    children: {

                        let template = ::hirola::prelude::ViewBuilder::new();
                        #children_tokens
                        template
                     }
                }]);
            }

            let quoted = if attributes.is_empty() {
                quote!({#fnname })
            } else {
                quote!({ #fnname {#(#attributes),*} })
            };
            tokens.extend(quote! {
                {
                    #quoted
                 }
            });
        }
    } else {
        tokens.extend(fragment_to_tokens(node.children));
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

fn attribute_to_tokens(attribute: &Node) -> TokenStream {
    match &attribute.value {
        Some(value) => {
            match attribute.node_type {
                NodeType::Block => {
                    quote! {
                        #value
                    }
                }
                NodeType::Attribute => {
                    // NodeType::Attribute nodes can't have no name
                    let name = attribute
                        .name_as_string()
                        .expect("attribute should have name");

                    if name.starts_with("on:") {
                        let name = name.replace("on:", "");
                        quote! {
                            ::hirola::prelude::ViewBuilder::event(
                                &mut template,
                                #name,
                                ::std::boxed::Box::new(#value),
                            );
                        }
                    } else if name.starts_with("mixin:") {
                        let name_space = name.replace("mixin:", "");
                        let ns_struct =
                            format_ident!("{}", &some_kind_of_uppercase_first_letter(&name_space));
                        quote! {
                            hirola::prelude::Mixin::<#ns_struct>::mixin(#value, &template);
                        }
                    } else if &name == "ref" {
                        quote! {
                            ::hirola::prelude::NodeRef::set(
                                &#value,
                                ::std::clone::Clone::clone(&template.node()),
                            );

                        }
                    } else {
                        let attribute_name = convert_name(&name);
                        quote! {
                            ::hirola::prelude::GenericNode::set_attribute(
                                template.node(),
                                #attribute_name,
                                &::std::format!("{}", #value),
                            );
                        }
                    }
                }
                _ => {
                    quote! {
                        compile_error!("Unexpected NodeType")
                    }
                }
            }
        }
        None => {
            let name = &attribute
                .name_as_string()
                .expect("attribute should have a name");
            if name.starts_with("use") {
                let effect = format_ident!("{}", &name.replace("use:", ""));
                quote! {
                    {
                        let effect = hirola::prelude::SideEffect::effect(#effect);
                        ::hirola::prelude::ViewBuilder::effect(&template, effect);
                    }
                }
            } else {
                let name = convert_name(name);
                quote! {
                    ::hirola::prelude::GenericNode::set_attribute(
                        template.node(),
                        #name,
                        &::std::format!(""),
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
            match child.node_type {
                NodeType::Element => {
                    let node = node_to_tokens(child);
                    append_children.extend(quote! {
                        ::hirola::prelude::ViewBuilder::append_child(&mut template, { #node });
                    });
                }
                NodeType::Text => {
                    let s = child
                        .value_as_string()
                        .expect("expecting a string on a text node");
                    append_children.extend(quote! {
                        ::hirola::prelude::ViewBuilder::append_child(
                            &mut template,
                            #[allow(unused_braces)]
                            ::hirola::prelude::ViewBuilder::Text(String::from(#s)),
                        );
                    });
                }
                NodeType::Comment => {
                    // let s = child
                    //     .value_as_string()
                    //     .expect("expecting a string on a text node");
                    // append_children.extend(quote! {
                    //     ::hirola::prelude::ViewBuilder::append_child(
                    //         &mut template,
                    //         #[allow(unused_braces)]
                    //         ::hirola::prelude::ViewBuilder::new_from_node(::hirola::prelude::GenericNode::comment(#s)),
                    //     );
                    // });
                }
                NodeType::Doctype => {}
                NodeType::Block => match child.value {
                    Some(syn::Expr::Block(expr_block)) => match braced_for_control(&expr_block) {
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
                                                ::hirola::prelude::ViewBuilder::Component(Box::new(indexed))
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
                                                    ::hirola::prelude::ViewBuilder::Component(Box::new(indexed))
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
                                        ::hirola::prelude::ViewBuilder::append_child(
                                            &mut template,
                                            #body,
                                        );
                                    }
                                });
                            }
                        }
                        Some(Control::ExprIf(ExprIf {
                            cond, then_branch, else_branch, ..
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
                                                ::hirola::prelude::ViewBuilder::Component(Box::new(switch))
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
                                                    ::hirola::prelude::ViewBuilder::Component(Box::new(switch))
                                                };
                                            });
                                        } else {
                                            append_children.extend(
                                                syn::Error::new(
                                                    ty.span(),
                                                    "expected Signal or _",
                                                )
                                                .to_compile_error(),
                                            );
                                        }
                                    }
                                    _ => {
                                        append_children.extend(
                                            syn::Error::new(ty.span(), "expected Signal, SignalVec or _")
                                                .to_compile_error(),
                                        );
                                    }
                                }
                            } else {
                                append_children.extend(quote! {
                                    ::hirola::prelude::ViewBuilder::append_render(
                                        &mut template,
                                        #expr_block,
                                    );
                                });
                            }
                        }

                        Some(Control::ExprMatch(ExprMatch {
                            expr, arms, ..
                        })) => {
                            match *expr {
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
                                            ::hirola::prelude::ViewBuilder::Component(Box::new(suspense))
                                        };
                                        ::hirola::prelude::ViewBuilder::append_render(
                                            &mut template,
                                            suspense
                                        );

                                    });
                                },
                                _ => {
                                    append_children.extend(quote! {
                                        ::hirola::prelude::ViewBuilder::append_render(
                                            &mut template,
                                            #expr_block,
                                        );
                                    });
                                }
                            }
                        }
                        _ => {
                            append_children.extend(quote! {
                                ::hirola::prelude::ViewBuilder::append_render(
                                    &mut template,
                                    #expr_block,
                                );
                            });
                        }
                    },
                    _ => {
                        return quote! {
                            compile_error!("Unexpected missing block for NodeType::Block")
                        }
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
    ExprMatch(ExprMatch)
}

fn braced_for_control(expr: &ExprBlock) -> Option<Control> {
    let len = expr.block.stmts.len();
    if len != 1 {
        None
    } else {
        let stmt = &expr.block.stmts[0];
        match stmt {
            Stmt::Expr(Expr::ForLoop(expr)) => Some(Control::ExprForLoop(expr.clone())),
            Stmt::Expr(Expr::If(expr)) => Some(Control::ExprIf(expr.clone())),
            Stmt::Expr(Expr::Match(expr)) => Some(Control::ExprMatch(expr.clone())),
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
        ::hirola::prelude::ViewBuilder::from(#output)
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
