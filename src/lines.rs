use crate::util::slice_like;
use proc_macro::TokenStream;

slice_like!("lines!");

pub fn lines(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Slice);

    let source = input.source.value();
    let mut result = String::new();

    for range in input.ranges {
        let start = range.0.unwrap_or(0);
        let end = range.1.unwrap_or(input.source.value().len()) - start;

        source.lines().skip(start).take(end).for_each(|s| {
            result.push_str(s);
            result.push('\n');
        });
    }

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
        t.pass("tests/lines/*.pass.rs");
        t.compile_fail("tests/lines/*.fail.rs");
    }
}
