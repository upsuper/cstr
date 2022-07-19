//! A macro for getting `&'static CStr` from literal or identifier.
//!
//! This macro checks whether the given literal is valid for `CStr`
//! at compile time, and returns a static reference of `CStr`.
//!
//! This macro can be used to to initialize constants on Rust 1.59 and above.
//!
//! ## Example
//!
//! ```
//! use cstr::cstr;
//! use std::ffi::CStr;
//!
//! let test = cstr!(b"hello\xff");
//! assert_eq!(test, CStr::from_bytes_with_nul(b"hello\xff\0").unwrap());
//! let test = cstr!("hello");
//! assert_eq!(test, CStr::from_bytes_with_nul(b"hello\0").unwrap());
//! let test = cstr!(hello);
//! assert_eq!(test, CStr::from_bytes_with_nul(b"hello\0").unwrap());
//! ```

// While this isn't necessary when using Cargo >= 1.42, omitting it actually requires path-less
// `--extern proc_macro` to be passed to `rustc` when building this crate. Some tools may not do
// this correctly. So it's added as a precaution.
#[cfg(proc_macro)]
extern crate proc_macro;

#[cfg(proc_macro)]
mod parse;
#[cfg(proc_macro)]
mod implementation;

#[cfg(proc_macro)]
#[proc_macro]
pub fn cstr(input: implementation::RawTokenStream) -> implementation::RawTokenStream {
    implementation::cstr(input)
}
