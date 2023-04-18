use crate::util::{end_to_usize, slice_like, start_to_usize};
use proc_macro::TokenStream;
use syn::__private::ToTokens;
use syn::parse_macro_input;

slice_like!("remove_lines!");

pub fn remove_lines(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Slice);

    let source = input.source.value();
    let source_len = source.lines().count();
    let mut result = String::new();

    let mut iter = source.lines().skip(0);
    let mut prev = 0;

    for range in input.ranges {
        let start = start_to_usize(source_len, range.0) - prev;
        let end = end_to_usize(source_len, range.1);

        iter.take(start).for_each(|line| {
            result.push_str(line);
            result.push('\n');
        });

        iter = source.lines().skip(end);

        prev = end;
    }

    iter.for_each(|line| {
        result.push_str(line);
        result.push('\n');
    });

    if result.ends_with('\n') {
        result.pop();
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
        t.pass("tests/remove_lines/*.pass.rs");
        t.compile_fail("tests/remove_lines/*.fail.rs");
    }
}
