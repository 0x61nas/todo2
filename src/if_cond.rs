use proc_macro::token_stream::IntoIter;
use std::iter::Peekable;

pub(crate) fn parse_if(tokens: &mut Peekable<IntoIter>) ->