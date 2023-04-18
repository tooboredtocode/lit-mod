use crate::util::slice_like;
use proc_macro::TokenStream;

slice_like!("remove_lines!");

pub fn remove_lines(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Slice);

    let source = input.source.value();
    let mut result = String::new();

    let mut iter = source.lines().skip(0);
    let mut prev = 0;

    for range in input.ranges {
        let start = range.0.unwrap_or(0) - prev;
        let end = range.1.unwrap_or(input.source.value().len());

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
