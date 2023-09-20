#[cfg(all(feature = "chrono-backend", feature = "time-backend"))]
compile_error!("You can only use one backend at a time");

mod date;

extern crate proc_macro;

use crate::date::parse_date;
use proc_macro::token_stream::IntoIter;
use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use std::iter::Peekable;
use std::time::SystemTime;

enum ConditionTyp {
    By(u64),
    If(TokenTree),
}

#[proc_macro]
pub fn todo(tokens: TokenStream) -> TokenStream {
    let mut tokens = tokens.into_iter().peekable();
    // Parse the message, should be the first argument(at least for now)
    let msg = match parse_msg(&mut tokens) {
        Ok(msg) => {
            if let Some(msg) = msg {
                let msg = msg.trim_matches(|c| c == '"' || c == '\'');
                msg.to_string()
            } else {
                return quote!(core::todo!()).into();
            }
        }
        Err(e) => return quote!(compile_error!(#e)).into(),
    };
    let Some(nt) = tokens.peek() else {
        // If there no other tokens, then that means that the user wanna use the original `todo` macro
        #[cfg(feature = "original-compatibility")]
        return quote!(core::todo!(#msg)).into();
        #[cfg(not(feature = "original-compatibility"))]
        return quote!(compile_error!("You should specify at least one condition, or if you do this accidentally, \
        then maybe you want to enable the `original-compatibility` feature")).into();
    };
    if let TokenTree::Punct(punct) = nt {
        #[cfg(feature = "strict-syntax")]
        {
            let punct = punct.as_char();
            if punct != ',' && punct != ';' {
                return quote!(compile_error!("Unexpected `{}`", #punct)).into();
            }
        }
        let _ = tokens.next();
    } else {
        #[cfg(feature = "strict-syntax")]
        return quote!(compile_error!("Expected `,` or `;` after the massage")).into();
    }
    let conditions = match parse_conditions(&mut tokens) {
        Ok(conditions) => conditions,
        Err(e) => return quote!(compile_error!(#e)).into(),
    };

    let mut rt = quote!();

    // let mut time_stamp = 0;
    for condition in conditions {
        match condition {
            ConditionTyp::By(time) => {
                #[cfg(feature = "compile-error")] {
                    let ct = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
                    if time <= ct {
                        let msg = format!("TODO: The deadline for `{}` has passed, do it now!", msg);
                        return quote!(compile_error!(#msg)).into();
                    }
                }
                // TODO: consider `no_std` compatibility?
                rt = quote! {
                    rt
                    if #time <= ::std::time::SystemTime::now().duration_since(::std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs() {
                        ::core::panic!("TODO: The deadline for `{}` has passed, do it now!", #msg);
                    }
                }
                // time_stamp = time;
            }
            ConditionTyp::If(_) => {
                core::todo!()
            }
        }
    }
    /*let ct = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    rt = quote! {
        println!("TODO: {}", #msg);
        println!("TODO: {}", #time_stamp);
        println!("TODO: {}", #ct);
    };*/

    rt.into()
}

fn parse_msg(tokens: &mut Peekable<IntoIter>) -> Result<Option<String>, String> {
    let Some(first_token) = tokens.next() else {
        #[cfg(not(feature = "original-compatibility"))]
        return Err("Seems like there is no arguments".to_string());
        #[cfg(feature = "original-compatibility")]
        return Ok(None);
    };

    match first_token {
        TokenTree::Literal(lit) => Ok(Some(lit.to_string())),
        _ => Err("The first argument must be literal".to_string()),
    }
}

fn parse_conditions(tokens: &mut Peekable<IntoIter>) -> Result<Vec<ConditionTyp>, String> {
    let mut conditions = Vec::with_capacity(2);
    while let Some(token) = tokens.next() {
        match token {
            TokenTree::Ident(ident) => {
                let Some(nt) = tokens.next() else {
                    return Err(format!("Expected `:` after `{}`", ident));
                };
                let TokenTree::Punct(punct) = nt else {
                    return Err(format!("Expected `:` after `{}` got `{}`", ident, nt));
                };
                if punct.as_char() != ':' {
                    return Err(format!("Expected `:` after `{}` got `{}`", ident, punct));
                }
                match ident.to_string().as_str() {
                    "by" => conditions.push(ConditionTyp::By(parse_date(tokens)?)),
                    "if" => {
                        core::todo!()
                    }
                    _ => return Err("Expected `by` or `if`".to_string()),
                }
            }
            TokenTree::Punct(punct) => {
                let punct = punct.as_char();
                if (punct == ',' || punct == ';') && !conditions.is_empty() {
                    continue;
                }
                return Err(format!("Unexpected `{punct}`"));
            }
            _ => return Err("Expected `by` or `if`".to_string()),
        }
    }
    Ok(conditions)
}
