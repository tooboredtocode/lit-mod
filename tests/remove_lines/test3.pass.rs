use lit_mod::remove_lines;

fn main() {
    let s = remove_lines!(r#"Hello world!
Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Sed euismod, nisl nec."#,
        1..2
    );
    assert_eq!(s, "Hello world!\nSed euismod, nisl nec.");
}