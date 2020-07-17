# cstr

[![Build Status](https://travis-ci.org/upsuper/cstr.svg?branch=master)](https://travis-ci.org/upsuper/cstr)
[![Docs](https://docs.rs/cstr/badge.svg)](https://docs.rs/cstr)

<!-- cargo-sync-readme start -->

A macro for getting `&'static CStr` from literal or identifier.

This macro checks whether the given literal is valid for `CStr`
at compile time, and returns a static reference of `CStr`.

Note that it currently cannot be used to initialize constants due
to restriction of Rust.

## Example

```rust
use cstr::cstr;
use std::ffi::CStr;

let test = cstr!(b"hello\xff");
assert_eq!(test, CStr::from_bytes_with_nul(b"hello\xff\0").unwrap());
let test = cstr!("hello");
assert_eq!(test, CStr::from_bytes_with_nul(b"hello\0").unwrap());
let test = cstr!(hello);
assert_eq!(test, CStr::from_bytes_with_nul(b"hello\0").unwrap());
```

<!-- cargo-sync-readme end -->
