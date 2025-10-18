use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, parse::Parse, parse::ParseStream, Token};

#[proc_macro]
pub fn hello_world(input: TokenStream) -> TokenStream{
        // Parse the input as a string literal
        let input = parse_macro_input!(input as syn::LitStr);
        let name = input.value();
        
        // Generate code that prints a greeting
        let output = quote! {
            println!("Hello, {}! Welcome to procedural macros!", #name);
        };
        
        TokenStream::from(output)
}