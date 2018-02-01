#[allow(unused_imports)]
#[macro_use]
extern crate cstr_macros;
#[macro_use]
extern crate procedural_masquerade;

pub use cstr_macros::*;

define_invoke_proc_macro!(cstr__invoke_build_bytes);

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

#[cfg(test)]
mod tests {
    use std::ffi::CStr;

    #[test]
    fn test_basic() {
        let test: &'static CStr = cstr!("aaa");
        assert_eq!(test, CStr::from_bytes_with_nul(b"aaa\0").unwrap());
    }
}
