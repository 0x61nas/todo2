use crate::Result;
use proc_macro2::token_stream::IntoIter;
use proc_macro2::{TokenStream, TokenTree};
use std::iter::Peekable;
use quote::TokenStreamExt;

pub(crate) fn parse_if(tokens: &mut Peekable<IntoIter>) -> Result<TokenStream> {
    let mut rt = TokenStream::new();
    for nt in tokens.by_ref() {
        if let TokenTree::Punct(punct) = &nt {
            let punct = punct.as_char();
            if punct == ',' || punct == ';' {
                break;
            }
        }
        rt.append(nt);
    }
    Ok(rt)
}
