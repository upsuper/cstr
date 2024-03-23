# cstr

[![CI](https://github.com/upsuper/cstr/workflows/CI/badge.svg)](https://github.com/upsuper/cstr/actions)
[![Crates.io](https://img.shields.io/crates/v/cstr.svg)](https://crates.io/crates/cstr)
[![Docs](https://docs.rs/cstr/badge.svg)](https://docs.rs/cstr)

<!-- cargo-sync-readme start -->

**This crate has been deprecated.
Rust 1.77.0 stabilized [C-string literals][c-string-literal].
From that version, `c"abc"` can be used in place of `cstr!("abc")` provided by this crate.
This new feature gives more concise code and faster compilation.
Hence, this crate will no longer be maintained.**

[c-string-literal]: https://blog.rust-lang.org/2024/03/21/Rust-1.77.0.html#c-string-literals

A macro for getting `&'static CStr` from literal or identifier.

This macro checks whether the given literal is valid for `CStr`
at compile time, and returns a static reference of `CStr`.

This macro can be used to initialize constants.

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
