use lit_mod::remove_lines;

fn main() {
    let s = remove_lines!("Hello, world!\nThis is rust", 1..);
    assert_eq!(s, "Hello, world!");
}