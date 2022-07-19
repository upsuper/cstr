use crate::parse::parse_input;
pub(super) use proc_macro::TokenStream as RawTokenStream;
use proc_macro2::{Literal, Span, TokenStream};
use quote::{quote, quote_spanned};
use std::ffi::CString;

pub(crate) struct Error(pub(crate) Span, pub(crate) &'static str);

pub(super) fn cstr(input: RawTokenStream) -> RawTokenStream {
    let tokens = match build_byte_str(input.into()) {
        Ok(s) => quote!(unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(#s) }),
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
