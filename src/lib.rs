#[allow(unused_imports)]
#[macro_use]
extern crate cstr_macros;
#[macro_use]
extern crate procedural_masquerade;

pub use cstr_macros::*;

define_invoke_proc_macro!(cstr__invoke_build_bytes);

/// Expands to a expression typed `&'static CStr` with the given characters.
/// (It currently only supports a UTF-8 string as input.)
///
/// Note that the expanded expression cannot be used to initialize `const`
/// currently.
///
/// # Example
///
/// ```
/// #[macro_use] extern crate cstr;
/// use std::ffi::CStr;
///
/// # fn main() {
/// let test = cstr!("hello");
/// assert_eq!(test, CStr::from_bytes_with_nul(b"hello\0").unwrap());
/// # }
/// ```
#[macro_export]
macro_rules! cstr {
    ($t: tt) => {
        {
            cstr__invoke_build_bytes! {
                cstr_internal__build_bytes!($t)
            }
            unsafe {
                ::std::ffi::CStr::from_bytes_with_nul_unchecked(BYTES)
            }
        }
    }
}
