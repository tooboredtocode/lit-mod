use lit_mod::slice;

fn main() {
    let s = slice!("I love Rust!", 2..6);
    assert_eq!(s, "love");
}