use crate::util::{end_to_usize, slice_like, start_to_usize};
use proc_macro::TokenStream;
use syn::__private::ToTokens;
use syn::parse_macro_input;

slice_like!("slice!");

pub fn slice(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Slice);

    let source = input.source.value();
    let source_len = source.chars().count();
    let mut result = String::new();

    for range in input.ranges {
        let start = start_to_usize(source_len, range.0);
        let end = end_to_usize(source_len, range.1) - start;

        source
            .chars()
            .skip(start)
            .take(end)
            .for_each(|c| result.push(c));
    }

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
        t.pass("tests/slice/*.pass.rs");
        t.compile_fail("tests/slice/*.fail.rs");
    }
}
