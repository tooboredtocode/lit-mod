use crate::util::slice_like;
use proc_macro::TokenStream;

slice_like!("slice!");

pub fn slice(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Slice);

    let source = input.source.value();
    let mut result = String::new();

    for range in input.ranges {
        let start = range.0.unwrap_or(0);
        let end = range.1.unwrap_or(input.source.value().len()) - start;

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
