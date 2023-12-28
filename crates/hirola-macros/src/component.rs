use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;
use syn::spanned::Spanned;

pub fn create_function_component(f: syn::ItemFn) -> TokenStream {
    let struct_name = f.sig.ident;
    let (impl_generics, ty_generics, where_clause) = f.sig.generics.split_for_impl();
    let inputs = f.sig.inputs;
    let block = f.block;
    let vis = f.vis;
    let output = f.sig.output;

    let output = match output {
        syn::ReturnType::Default => {
            emit_error!(output.span(), "A valid return is expected");
            panic!("invalid component provided")
        }
        syn::ReturnType::Type(_, ty) => ty,
    };

    let inputs_block = if !inputs.is_empty() {
        let input_names: Vec<_> = inputs.iter().collect();

        quote!({ #(#vis #input_names),* })
    } else {
        quote!(;)
    };

    let inputs_reading = if inputs.is_empty() {
        quote!()
    } else {
        let input_names: Vec<_> = inputs
            .iter()
            .filter_map(|argument| match argument {
                syn::FnArg::Typed(typed) => Some(typed),
                syn::FnArg::Receiver(rec) => {
                    emit_error!(rec.span(), "Don't use `self` on components");
                    None
                }
            })
            .map(|value| {
                let pat = &value.pat;
                quote!(#pat)
            })
            .collect();
        quote!(
            let #struct_name { #(#input_names),* } = *self;
        )
    };

    TokenStream::from(quote! {
        // #[derive(Debug)]
        #vis struct #struct_name #impl_generics #inputs_block

        impl #impl_generics ::hirola::prelude::Render<#output> for #struct_name #ty_generics #where_clause {
            fn render_into(self: Box<Self>, dom: &#output) -> Result<(), hirola::prelude::Error> {
                let result = {
                    #inputs_reading
                    #block
                };
                hirola::prelude::GenericNode::append_child(dom, &result);
                // Box::new(result).render_into(&dom)?;
                Ok(())
            }
        }
    })
}
