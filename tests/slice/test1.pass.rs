use lit_mod::slice;

fn main() {
    let s = slice!("Hello, world!", 7..);
    assert_eq!(s, "world!");
}