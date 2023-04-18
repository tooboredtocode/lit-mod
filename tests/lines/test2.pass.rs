use lit_mod::lines;

fn main() {
    let s = lines!("Hello, world!\nThis is rust", ..1);
    assert_eq!(s, "Hello, world!");
}