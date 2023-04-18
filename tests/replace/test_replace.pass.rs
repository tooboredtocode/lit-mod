use lit_mod::replace;

fn main() {
    let s = replace!("Hello, world!", "world", "Rust");
    assert_eq!(s, "Hello, Rust!");
}