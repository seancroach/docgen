//! This small crate is to let developers easily document items generated in
//! macros where the content of the documentation is dynamic.
//!
//! # Examples
//!
//! ```
//! #[macro_use]
//! extern crate docgen;
//!
//! doc!(
//!     "Here is some documentation!"
//!     ""
//!     "Empty lines are represented by empty strings.";
//!     pub fn foo() {}
//! );
//!
//! # fn main() {}
//! ```
//!
//! Commas can be used as delimitter between lines:
//!
//! ```
//! #[macro_use]
//! extern crate docgen;
//!
//! doc!(
//!     "Here is some documentation!",
//!     "",
//!     "Empty lines are represented by empty strings.";
//!     pub fn foo() {}
//! );
//! # fn main() {}
//! ```
//!
//! This is particularly useful when documenting items created by macros:
//!
//! ```
//! #[macro_use]
//! extern crate docgen;
//!
//! macro_rules! add_fn {
//!     ($name:ident, $ty:ty) => {
//!         doc!(
//!             concat!("Add two [`", stringify!($ty), "`] values together.");
//!             pub fn $name(a: $ty, b: $ty) -> $ty {
//!                 a + b
//!             }
//!         );
//!     }
//! }
//!
//! add_fn!(add_u8, u8);
//! add_fn!(add_i8, i8);
//! # fn main() {}
//! ```

/// This macro is used to generate documentation upon an item.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate docgen;
///
/// doc!(
///     "Here is the documentation!"
///     ""
///     "Tah dah!";
///     #[inline]
///     pub fn foo() {}
/// );
///
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! doc {
    ($($expr:expr)*; $x:item) => {
        $(#[doc = $expr])*
        $x
    };
    // The same as the case above, but commas between expressions are allowed.
    ($($expr:expr),*; $x:item) => {
        $(#[doc = $expr])*
        $x
    };
}
