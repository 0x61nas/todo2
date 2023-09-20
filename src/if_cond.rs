use crate::Result;
use proc_macro::token_stream::IntoIter;
use proc_macro::TokenStream;
use std::iter::Peekable;

pub(crate) fn parse_if(tokens: &mut Peekable<IntoIter>) -> Result<TokenStream> {
    core::todo!()
}
