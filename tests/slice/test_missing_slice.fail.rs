use lit_mod::slice;

fn main() {
    let s = slice!("Hello, world!");
    assert_eq!(s, "Hello");
}