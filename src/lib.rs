//! lit-mod is a collection of procedural macros for working with string literals.
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! lit-mod = "0.1"
//! ```

mod lines;
mod remove_lines;
mod replace;
mod slice;
mod util;

use proc_macro::TokenStream;

/// Replaces all matches of a pattern with another string in a string literal.
///
/// Alternatively the third argument can be ignored and the macro will remove all matches of the
/// pattern instead.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use lit_mod::replace;
///
/// assert_eq!("this is new", replace!("this is old", "old", "new"));
/// assert_eq!("than an old", replace!("this is old", "is", "an"));
/// assert_eq!("this is", replace!("this is old", " old"));
/// ```
///
/// When the pattern doesn't match:
///
/// ```
/// use lit_mod::replace;
///
/// assert_eq!(
///     "this is old",
///     replace!("this is old", "cookie monster", "little lamb")
/// );
/// ```
#[proc_macro]
pub fn replace(input: TokenStream) -> TokenStream {
    replace::replace(input)
}

/// Replaces the literal with a slice of the literal.
///
/// *Note:* This macro doesn't slice by bytes, but by characters.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use lit_mod::slice;
///
/// assert_eq!("world!", slice!("Hello, world!", 7..));
/// assert_eq!("Hello", slice!("Hello, world!", ..-8));
/// ```
#[proc_macro]
pub fn slice(input: TokenStream) -> TokenStream {
    slice::slice(input)
}

/// Replaces the literal with a literal of the lines.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use lit_mod::lines;
///
/// assert_eq!(
///     "Hello, world!",
///     lines!("Hello, world!\nThis is lines", 0..1)
/// );
/// assert_eq!(
///     "This is lines",
///     lines!("Hello, world!\nThis is lines", -1..)
/// );
/// ```
#[proc_macro]
pub fn lines(input: TokenStream) -> TokenStream {
    lines::lines(input)
}

/// Replaces the literal with a literal without the lines.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use lit_mod::remove_lines;
///
/// assert_eq!(
///     "Hello, world!",
///     remove_lines!("Hello, world!\nThis is lines", -1..)
/// );
/// assert_eq!(
///     "This is lines",
///     remove_lines!("Hello, world!\nThis is lines", ..1)
/// );
/// ```
#[proc_macro]
pub fn remove_lines(input: TokenStream) -> TokenStream {
    remove_lines::remove_lines(input)
}
