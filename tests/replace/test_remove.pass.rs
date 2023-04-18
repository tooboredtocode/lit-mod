use lit_mod::replace;

fn main() {
    let s = replace!("Hello, world! I love you..", " I love you..");
    assert_eq!(s, "Hello, world!");
}