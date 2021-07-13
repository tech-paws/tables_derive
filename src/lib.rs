extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(U64Id)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_u64id(&ast)
}

fn impl_u64id(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl U64Id for #name {
            fn get_id(&self) -> u64 {
                self.0
            }
        }
    };
    gen.into()
}
