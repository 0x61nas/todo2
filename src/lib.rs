#[cfg(all(feature = "chrono-backend", feature = "time-backend"))]
compile_error!("You can only use one backend at a time");

mod date;
mod if_cond;

extern crate proc_macro;

use crate::date::parse_date;
use crate::if_cond::parse_if;
use proc_macro::TokenStream;
use proc_macro2::token_stream::IntoIter;
use proc_macro2::TokenTree;
use quote::{quote, TokenStreamExt};
use std::iter::Peekable;
use std::time::SystemTime;

pub(crate) type Result<T> = std::result::Result<T, String>;

enum ConditionTyp {
    By(u64),
    If(proc_macro2::TokenStream),
}

#[proc_macro]
pub fn todo(tokens: TokenStream) -> TokenStream {
    let mut tokens = proc_macro2::TokenStream::from(tokens)
        .into_iter()
        .peekable();
    // Parse the message, should be the first argument(at least for now)
    let msg = match parse_msg(&mut tokens) {
        Ok(msg) => {
            if let Some(msg) = msg {
                let msg = msg.trim_matches(|c| c == '"' || c == '\'');
                msg.to_string()
            } else {
                return TokenStream::from(quote!(core::todo!()));
            }
        }
        Err(e) => return TokenStream::from(quote!(compile_error!(#e))),
    };
    let Some(nt) = tokens.peek() else {
        // If there no other tokens, then that means that the user wanna use the original `todo` macro
        #[cfg(feature = "original-compatibility")]
        return TokenStream::from(quote!(core::todo!(#msg)));
        #[cfg(not(feature = "original-compatibility"))]
        return TokenStream::from(quote!(compile_error!(
            "You should specify at least one condition, or if you do this accidentally, \
        then maybe you want to enable the `original-compatibility` feature"
        )));
    };
    if let TokenTree::Punct(punct) = nt {
        #[cfg(feature = "strict-syntax")]
        {
            let punct = punct.as_char();
            if punct != ',' && punct != ';' {
                return TokenStream::from(quote!(compile_error!("Unexpected `{}`", #punct)));
            }
        }
        let _ = tokens.next();
    } else {
        #[cfg(feature = "strict-syntax")]
        return TokenStream::from(quote!(compile_error!(
            "Expected `,` or `;` after the massage"
        )));
    }
    let conditions = match parse_conditions(tokens) {
        Ok(conditions) => conditions,
        Err(e) => return TokenStream::from(quote!(compile_error!(#e))),
    };

    let mut rt = quote!();

    let mut time_stamp = 0;
    for condition in conditions {
        match condition {
            ConditionTyp::By(time) => {
                #[cfg(feature = "compile-error")]
                {
                    let ct = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    if time <= ct {
                        let msg =
                            format!("TODO: The deadline for `{}` has passed, do it now!", msg);
                        return TokenStream::from(quote!(compile_error!(#msg)));
                    }
                }
                // TODO: consider `no_std` compatibility?
                #[cfg(not(any(feature = "with-chrono", feature = "with-time")))]
                rt.append_all(quote! {
                    if #time <= ::std::time::SystemTime::now().duration_since(::std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs() {
                        ::core::panic!("TODO: The deadline for `{}` has passed, do it now!", #msg);
                    }
                });
                #[cfg(feature = "with-chrono")]
                rt.append_all(quote! {
                    if #time <= ::chrono::Utc::now().timestamp() as u64 {
                        ::core::panic!("TODO: The deadline for `{}` has passed, do it now!", #msg);
                    }
                });
                #[cfg(feature = "with-time")]
                rt.append_all(quote! {
                    if #time <= ::time::OffsetDateTime::now_utc().unix_timestamp() as u64 {
                        ::core::panic!("TODO: The deadline for `{}` has passed, do it now!", #msg);
                    }
                });
                time_stamp = time;
            }
            ConditionTyp::If(if_cond) => {
                let msg = format!("TODO: {}", msg);
                rt.append_all(quote! {
                    if #if_cond {
                        ::core::panic!(#msg);
                    }
                });
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

    TokenStream::from(rt)
}

fn parse_msg(tokens: &mut Peekable<IntoIter>) -> Result<Option<String>> {
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

fn parse_conditions(mut tokens: Peekable<IntoIter>) -> Result<Vec<ConditionTyp>> {
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
                    "by" => conditions.push(ConditionTyp::By(parse_date(&mut tokens)?)),
                    "if" => conditions.push(ConditionTyp::If(parse_if(&mut tokens)?)),
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
