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
//! # fn main() {
//! let test = cstr!(b"hello\xff");
//! assert_eq!(test, CStr::from_bytes_with_nul(b"hello\xff\0").unwrap());
//! let test = cstr!("hello");
//! assert_eq!(test, CStr::from_bytes_with_nul(b"hello\0").unwrap());
//! let test = cstr!(hello);
//! assert_eq!(test, CStr::from_bytes_with_nul(b"hello\0").unwrap());
//! # }
//! ```

use proc_macro::TokenStream as RawTokenStream;
use proc_macro2::{Literal, Span, TokenStream};
use quote::quote;
use std::ffi::CString;
use syn::parse::{Parse, ParseBuffer};
use syn::{Error, Ident, LitByteStr, LitStr, Result};

#[proc_macro]
pub fn cstr(input: RawTokenStream) -> RawTokenStream {
    let tokens = match build_byte_str(input.into()) {
        Ok(s) => quote!(unsafe {
            ::std::mem::transmute::<_, &::std::ffi::CStr>(
                #s as *const [u8] as *const ::std::ffi::CStr
            )
        }),
        Err(e) => e.to_compile_error(),
    };
    tokens.into()
}

fn build_byte_str(input: TokenStream) -> Result<Literal> {
    let Input(bytes, span) = syn::parse2::<Input>(input)?;
    CString::new(bytes)
        .map(|s| {
            let mut lit = Literal::byte_string(s.as_bytes_with_nul());
            lit.set_span(span);
            lit
        })
        .map_err(|_| Error::new(span, "nul byte found in the literal"))
}

struct Input(Vec<u8>, Span);

impl Parse for Input {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitByteStr) {
            let b = input.parse::<LitByteStr>().unwrap();
            return Ok(Input(b.value(), b.span()));
        }
        if lookahead.peek(LitStr) {
            let s = input.parse::<LitStr>().unwrap();
            return Ok(Input(s.value().into_bytes(), s.span()));
        }
        if lookahead.peek(Ident) {
            let i = input.parse::<Ident>().unwrap();
            return Ok(Input(i.to_string().into_bytes(), i.span()));
        }
        Err(lookahead.error())
    }
}
