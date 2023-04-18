macro_rules! slice_like {
    ($name:literal) => {
        use syn::__private::ToTokens;
        use syn::{Expr, ExprLit, ExprRange, Lit, LitStr, parse_macro_input, RangeLimits, Token};
        use syn::parse::{Parse, ParseStream, Result};

        struct Slice {
            source: LitStr,
            ranges: Vec<(Option<usize>, Option<usize>)>,
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

                    let start = match range.start {
                        Some(boxed) => {
                            match *boxed {
                                Expr::Lit(ExprLit { lit: Lit::Int(lit), .. }) => {
                                    lit.base10_parse::<usize>()
                                        .map_err(|_| input.error(concat!($name, " expects valid ranges")))?
                                        .into()
                                }
                                _ => return Err(input.error(concat!($name, " expects ranges of integers"))),
                            }
                        }
                        None => None,
                    };

                    let end = match range.end {
                        Some(boxed) => {
                            match *boxed {
                                Expr::Lit(ExprLit { lit: Lit::Int(lit), .. }) => {
                                    lit.base10_parse::<usize>()
                                        .map_err(|_| input.error(concat!($name, " expects valid ranges")))?
                                        .into()
                                }
                                _ => return Err(input.error(concat!($name, " expects ranges of integers"))),
                            }
                        }
                        None => None,
                    };

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

pub(crate) use slice_like;
