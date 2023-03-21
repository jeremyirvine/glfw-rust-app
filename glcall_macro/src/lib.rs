use syn::{parse_macro_input, Block};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};


#[proc_macro]
pub fn gl_call(body: TokenStream) -> TokenStream {
    let body_cloned = body.clone();
    let parsed = parse_macro_input!(body as Block);
    let stmts = parsed.stmts;


    stmts.iter().enumerate().map(|(i, stmt)| -> TokenStream {
        let stmt_str = stmt.to_token_stream().to_string();
        TokenStream::from(quote!(
            gl_clear_errors();
            unsafe { #stmt }
            gl_log_errors(file!(), line!(), #stmt_str.into());
        ))
    }).collect()
}