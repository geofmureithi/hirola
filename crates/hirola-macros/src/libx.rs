use proc_macro::TokenStream;
use quote::quote;
use syn::Expr;

use syn_rsx::{parse, Node, NodeType};
// fn walk_nodes(nodes: Vec<Node>) -> (String, Vec<Expr>) {
//     let mut out = String::new();
//     let mut values = vec![];
//     for node in nodes {
//         match node.node_type {
//             NodeType::Element => {
//                 let name = node.name_as_string().unwrap();
//                 out.push_str(&format!("<{}", name));

//                 // attributes
//                 let (html_string, attribute_values) = walk_nodes(node.attributes);
//                 out.push_str(&html_string);
//                 values.extend(attribute_values);
//                 out.push('>');

//                 // https://developer.mozilla.org/en-US/docs/Glossary/Empty_element
//                 match name.as_str() {
//                     "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link"
//                     | "meta" | "param" | "source" | "track" | "wbr" => continue,
//                     _ => (),
//                 }

//                 // children
//                 let (html_string, children_values) = walk_nodes(node.children);
//                 out.push_str(&html_string);
//                 values.extend(children_values);

//                 out.push_str(&format!("</{}>", name));
//             }
//             NodeType::Attribute => {
//                 out.push_str(&format!(" {}", node.name_as_string().unwrap()));
//                 if node.value.is_some() {
//                     out.push_str(r#"="{}""#);
//                     values.push(node.value.unwrap());
//                 }
//             }

//             NodeType::Text | NodeType::Block => {
//                 out.push_str("{}");
//                 values.push(node.value.unwrap());
//             }
//             NodeType::Fragment => {
//                 let (html_string, children_values) = walk_nodes(node.children);
//                 out.push_str(&html_string);
//                 values.extend(children_values);
//             }
//             NodeType::Comment => {
//                 out.push_str("<!-- {} -->");
//                 values.push(node.value.unwrap());
//             }
//             NodeType::Doctype => {
//                 let value = node.value_as_string().unwrap();
//                 out.push_str(&format!("<!DOCTYPE {}>", value));
//             }
//         }
//     }

//     (out, values)
// }

fn node_to_tokens(node: Node) -> TokenStream {
    let mut set_attributes = Vec::new();
    let mut set_event_listeners = Vec::new();
    let mut set_noderefs = Vec::new();
    let mut tokens = TokenStream::new();
    for node in nodes {
        // NodeType::Element nodes can't have no name
        let name = node.name_as_string().expect("node should have a name");

        let attributes = node
            .attributes
            .iter()
            .map(|attribute| attribute_to_tokens(attribute));
    }
}

fn attribute_to_tokens(attribute: &Node) -> TokenStream {
    match &attribute.value {
        Some(value) => {
            match attribute.node_type {
                NodeType::Block => {
                    quote! {
                        sauron::Attribute::from(#value)
                    }
                }
                NodeType::Attribute => {
                    // NodeType::Attribute nodes can't have no name

                    if name.starts_with("on:") {
                        let name = quote::format_ident!("{}", name);
                        set_event_listeners.push(quote_spanned! { expr_span=>
                            ::maple_core::generic_node::GenericNode::event(
                                &element,
                                #name,
                                ::std::boxed::Box::new(#value),
                            );
                        });
                    } else {
                        let name = attribute
                            .name_as_string()
                            .expect("attribute should have name");

                        let attribute_name = name.to_string();
                        set_attributes.push(quote_spanned! { expr_span=>
                            ::maple_core::reactive::create_effect({
                                let element = ::std::clone::Clone::clone(&element);
                                move || {
                                    ::maple_core::generic_node::GenericNode::set_attribute(
                                        &element,
                                        #attribute_name,
                                        &::std::format!("{}", #value),
                                    );
                                }
                            });
                        });
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
            let name = convert_name(
                &attribute
                    .name_as_string()
                    .expect("attribute should have a name"),
            );
            quote! {
                sauron::Attribute::new(
                    None,
                    #name,
                    sauron_core::prelude::html::attributes::AttributeValue::Empty,
                )
            }
        }
    }
}

#[proc_macro]
pub fn html_to_string(tokens: TokenStream) -> TokenStream {
    match parse(tokens) {
        Ok(nodes) => {
            // let (html_string, values) = walk_nodes(nodes);
            // quote! { format!(#html_string, #(#values),*) }
            node_to_tokens(node)
        }
        Err(error) => error.to_compile_error(),
    }
    .into()
}
