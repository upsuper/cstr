//! A macro for getting `&'static CStr` from literal or identifier.
//!
//! This macro checks whether the given literal is valid for `CStr`
//! at compile time, and returns a static reference of `CStr`.
//!
//! This macro can be used to to initialize constants on Rust 1.46 and above.
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

use crate::parse::parse_input;
use proc_macro::TokenStream as RawTokenStream;
use proc_macro2::{Literal, Span, TokenStream};
use quote::{quote, quote_spanned};
use std::ffi::CString;

mod parse;

struct Error(Span, &'static str);

#[proc_macro]
pub fn cstr(input: RawTokenStream) -> RawTokenStream {
    let tokens = match build_byte_str(input.into()) {
        Ok(s) => quote!(unsafe {
            ::std::mem::transmute::<_, &::std::ffi::CStr>(
                #s as *const [u8] as *const ::std::ffi::CStr
            )
        }),
        Err(Error(span, msg)) => quote_spanned!(span => compile_error!(#msg)),
    };
    tokens.into()
}

fn build_byte_str(input: TokenStream) -> Result<Literal, Error> {
    let (bytes, span) = parse_input(input)?;
    match CString::new(bytes) {
        Ok(s) => {
            let mut lit = Literal::byte_string(s.as_bytes_with_nul());
            lit.set_span(span);
            Ok(lit)
        }
        Err(_) => Err(Error(span, "nul byte found in the literal")),
    }
}
