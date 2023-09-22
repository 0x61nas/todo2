mod unix_time;

use crate::date::unix_time::{UnixTimeCalc, ONE_HOUR};
use crate::Result;
use proc_macro2::token_stream::IntoIter;
use proc_macro2::TokenTree;
use std::iter::Peekable;

pub(crate) fn parse_date(tokens: &mut Peekable<IntoIter>) -> Result<u64> {
    #[cfg(feature = "chrono-backend")]
    let un_calc = unix_time::chrono::ChronoCalc::new();
    #[cfg(feature = "time-backend")]
    let un_calc = unix_time::time::TimeCalc::new();
    #[cfg(not(any(feature = "chrono-backend", feature = "time-backend")))]
    let un_calc = unix_time::simple::SimpleCalc::new();

    _parse(tokens, un_calc)
}

fn _parse<C: UnixTimeCalc>(tokens: &mut Peekable<IntoIter>, mut un_calc: C) -> Result<u64> {
    let mut time_stamp = 0;
    #[cfg(feature = "and-time")]
    let mut parsing_date = true;
    #[cfg(feature = "and-time")]
    let mut hour_not_set = true;
    #[cfg(feature = "and-time")]
    let mut minute_not_set = true;
    for nt in tokens.by_ref() {
        match nt {
            TokenTree::Punct(punct) => {
                let punct = punct.as_char();
                if punct == '-' {
                    #[cfg(feature = "and-time")]
                    if !parsing_date {
                        return Err(format!("Unexpected `{}`, you can't use `-` in the time part only use `:` or `.`", punct));
                    }
                    continue;
                }
                #[cfg(feature = "and-time")]
                if punct == '@' {
                    if !parsing_date {
                        return Err(
                            "You've already specified the time, you can't specify it again"
                                .to_string(),
                        );
                    }
                    parsing_date = false;
                    time_stamp = un_calc.calc()?;
                    continue;
                } else if !parsing_date && punct == ':' {
                    continue;
                }
                if punct == ',' || punct == ';' {
                    break;
                } else {
                    return Err(format!("Unexpected `{}`", punct));
                }
            }
            TokenTree::Literal(lit) => {
                let lit = lit.to_string();
                #[cfg(feature = "and-time")]
                if !parsing_date {
                    if time_stamp == 0 {
                        return Err("You must at least specify year in the date part before specifying the time :)".to_string());
                    }
                    let n = lit
                        .parse::<u64>()
                        .map_err(|e| format!("Invalid number `{}`: {}", lit, e))?;
                    if hour_not_set {
                        if n > 23 {
                            return Err(format!("Invalid hour `{}`, it must be less than 24", n));
                        }
                        time_stamp += n * ONE_HOUR;
                        hour_not_set = false
                    } else if minute_not_set {
                        if n > 59 {
                            return Err(format!("Invalid minute `{}`", n));
                        }
                        time_stamp += n * 60;
                        minute_not_set = false;
                    } else {
                        return Err(format!("Unexpected `{}`", lit));
                    }
                    continue;
                }
                if !un_calc.is_year_set() {
                    un_calc.year(lit)?;
                } else if !un_calc.is_month_set() {
                    un_calc.month(lit)?;
                } else if !un_calc.is_day_set() {
                    un_calc.day(lit)?;
                } else {
                    return Err(format!("Unexpected  `{}`", lit));
                }
            }
            TokenTree::Ident(ident) => {
                #[cfg(feature = "and-time")]
                if ident == "at" {
                    if !parsing_date {
                        return Err(
                            "You've already specified the time, you can't specify it again"
                                .to_string(),
                        );
                    }
                    parsing_date = false;
                    time_stamp = un_calc.calc()?;
                    continue;
                }
                return Err(format!(
                    "You can't use `{}` here, maybe you meant to enable the `and-time` feature",
                    ident
                ));
            }
            _ => {
                return Err(format!("Unexpected `{}`", nt));
            }
        }
    }
    if time_stamp == 0 {
        time_stamp = un_calc.calc()?;
    }
    if time_stamp == 0 {
        return Err("You must at least specify the year".to_string());
    }
    Ok(time_stamp)
}
