use syn::{parse_macro_input, Block};
use proc_macro::TokenStream;
use quote::quote;


#[proc_macro]
pub fn gl_call(body: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(body as Block);
    let stmts = parsed.stmts;

    stmts.iter().map(|stmt| -> TokenStream {
        TokenStream::from(quote!(
            unsafe { #stmt }
        ))
    }).collect()
}