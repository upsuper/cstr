# cstr

[![Build Status](https://travis-ci.org/upsuper/cstr.svg?branch=master)](https://travis-ci.org/upsuper/cstr)
[![Docs](https://docs.rs/cstr/badge.svg)](https://docs.rs/cstr)

A macro for getting `&'static CStr` from literal.

This macro checks whether the given literal is valid for `CStr`
at compile time, and returns a static reference of `CStr`.

Note that it currently cannot be used to initialize constants due
to restriction of Rust.

Also, it currently only supports a UTF-8 string as input because
Rust's tokenizer only accepts that without the `b` prefix. This
may be expanded in the future if necessary.

# Example

```
#[macro_use] extern crate cstr;
use std::ffi::CStr;

# fn main() {
let test = cstr!("hello");
assert_eq!(test, CStr::from_bytes_with_nul(b"hello\0").unwrap());
# }
```
