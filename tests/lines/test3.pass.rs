use lit_mod::lines;

fn main() {
    let s = lines!(r#"Hello world!
Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Sed euismod, nisl nec."#,
        1..2
    );
    assert_eq!(s, "Lorem ipsum dolor sit amet, consectetur adipiscing elit.");
}