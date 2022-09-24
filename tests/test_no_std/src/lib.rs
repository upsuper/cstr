//! Verifies that cstr! can be used on no_std systems.
//!
//! To ensure that `std` is not sneaked in through a dependency (even though this crate has none at
//! runtime), this should best be built on a target that has no `std` because it has no operating
//! system, eg. thumbv7em-none-eabi.
//!
//! Note that building the [`cstr`] crate alone is insufficient, as it does not run throuogh any
//! `cstr!()` code generation and thus not trip over std-isms in the generated code.
#![no_std]

use core::ffi::CStr;

pub fn can_use_cstr_macro() -> &'static CStr {
    cstr::cstr!("Hello World!")
}
