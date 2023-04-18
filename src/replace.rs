use proc_macro::TokenStream;
use syn::__private::ToTokens;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, LitStr, Token};

struct Replace {
    source: LitStr,
    from: LitStr,
    to: Option<LitStr>,
}

impl Parse for Replace {
    fn parse(input: ParseStream) -> Result<Self> {
        let source = input
            .parse::<LitStr>()
            .map_err(|_| input.error("replace! requires a string as the first argument"))?;

        input
            .parse::<Token![,]>()
            .map_err(|_| input.error("replace! requires a string as the second argument"))?;

        let from = input
            .parse::<LitStr>()
            .map_err(|_| input.error("replace! requires a string as the second argument"))?;

        let to = match input.parse::<Token![,]>() {
            Ok(_) => {
                let to = input
                    .parse::<LitStr>()
                    .map_err(|_| input.error("replace! requires a string as the third argument"))?;
                Some(to)
            }
            Err(_) => None,
        };

        if !input.is_empty() {
            return Err(input.error("replace! only accepts 2 or 3 arguments"));
        }

        Ok(Replace { source, from, to })
    }
}

pub fn replace(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Replace);

    let result = match input.to {
        Some(to) => input
            .source
            .value()
            .replace(&input.from.value(), &to.value()),
        None => input.source.value().replace(&input.from.value(), ""),
    };

    LitStr::new(&result, input.source.span())
        .into_token_stream()
        .into()
}

#[cfg(test)]
mod test {
    use trybuild::TestCases;

    #[test]
    fn test_build() {
        let t = TestCases::new();
        t.pass("tests/replace/*.pass.rs");
        t.compile_fail("tests/replace/*.fail.rs");
    }
}
