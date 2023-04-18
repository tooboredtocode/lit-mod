macro_rules! slice_like {
    ($name:literal) => {
        use crate::util::unwind_expr_to_isize;
        use syn::{ExprRange, LitStr, RangeLimits, Token};
        use syn::parse::{Parse, ParseStream, Result};

        struct Slice {
            source: LitStr,
            ranges: Vec<(Option<isize>, Option<isize>)>,
        }

        impl Parse for Slice {
            fn parse(input: ParseStream) -> Result<Self> {
                let source = input.parse::<LitStr>()
                    .map_err(|_| input.error(concat!($name, " expects a string as the first argument")))?;
                input.parse::<Token![,]>()
                    .map_err(|_| input.error(concat!($name, " expects at least one range")))?;

                let mut ranges = Vec::new();

                while !input.is_empty() {
                    let range = input.parse::<ExprRange>()?;
                        let start = unwind_expr_to_isize(range.start)
                            .map_err(|err| input.error(format!("{} {}", $name, err)))?;
                        let end = unwind_expr_to_isize(range.end)
                            .map_err(|err| input.error(format!("{} {}", $name, err)))?;

                    if let (Some(start), Some(end)) = (start, end) {
                        if start > end {
                            return Err(input.error(concat!($name, " expects valid ranges")));
                        }
                    }

                    match range.limits {
                        RangeLimits::HalfOpen(_) => {
                            ranges.push((start, end));
                        }
                        RangeLimits::Closed(_) => {
                            ranges.push((start, end.map(|e| e + 1)));
                        }
                    }
                }

                Ok(Self { source, ranges })
            }
        }
    };
}

use syn::{Expr, ExprLit, ExprUnary, Lit};
pub(crate) use slice_like;

pub(crate) fn unwind_expr_to_isize(expr: Option<Box<Expr>>) -> Result<Option<isize>, &'static str> {
    match expr {
        Some(boxed) => {
            match *boxed {
                Expr::Lit(ExprLit { lit: Lit::Int(lit), .. }) => {
                    Ok(
                        lit.base10_parse::<isize>()
                            .map_err(|_| "expects ranges of integers")?
                            .into()
                    )
                }
                Expr::Unary(
                    ExprUnary {
                        op: syn::UnOp::Neg(_),
                        expr,
                        ..
                    }
                ) => {
                    match *expr {
                        Expr::Lit(ExprLit { lit: Lit::Int(lit), .. }) => {
                            Ok(
                                lit.base10_parse::<isize>()
                                    .map_err(|_| "expects ranges of integers")?
                                    .checked_neg()
                                    .ok_or("expects ranges of integers")?
                                    .into()
                            )
                        }
                        _ => Err("expects ranges of integers"),
                    }
                }
                _ => Err("expects ranges of integers"),
            }
        }
        None => Ok(None),
    }
}

pub fn start_to_usize(source_len: usize, index: Option<isize>) -> usize {
    match index {
        Some(index) if index < 0 => source_len - index.abs() as usize,
        Some(index) => index as usize,
        None => 0,
    }
}

pub fn end_to_usize(source_len: usize, index: Option<isize>) -> usize {
    match index {
        Some(index) if index < 0 => source_len - index.abs() as usize,
        Some(index) => index as usize,
        None => source_len,
    }
}