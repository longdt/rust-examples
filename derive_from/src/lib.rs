use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(From, attributes(from))]
pub fn from_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_from(&ast)
}

fn impl_from(ast: &DeriveInput) -> TokenStream {
    let target_name = &ast.ident;
    let source_name = ast.attrs
        .iter()
        .find(|attr| attr.path().is_ident("from"))
        .expect("Could not find `from` attribute")
        .parse_args::<syn::Path>()
        .expect("Could not parse argument to `from`");
    // Generate field mapping
    let struct_data = match &ast.data {
        Data::Struct(data) => data,
        _ => panic!("From can only be used with structs"),
    };

    let field_mapping = if let Fields::Named(fields_named) = &struct_data.fields {
        fields_named.named.iter().map(|field| {
            let field_name = &field.ident;
            quote! { #field_name: value.#field_name }
        })
    } else {
        panic!("From only supports named fields")
    };
    let gen = quote! {
        impl ::std::convert::From<#source_name> for #target_name {
            fn from(value: #source_name) -> Self {
                Self {
                    #(#field_mapping),*
                }
            }
        }
    };
    gen.into()
}