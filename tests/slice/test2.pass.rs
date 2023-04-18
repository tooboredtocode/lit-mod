use lit_mod::slice;

fn main() {
    let s = slice!("Hello, world!", ..5);
    assert_eq!(s, "Hello");
}