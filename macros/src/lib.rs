#[macro_use]
extern crate procedural_masquerade;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use std::char;
use std::ffi::CString;
use std::fmt::Write;

define_proc_macros! {
    #[allow(non_snake_case)]
    pub fn cstr_internal__build_bytes(input: &str) -> String {
        let bytes = build_bytes(input);
        format!("const BYTES: &'static [u8] = {};", bytes)
    }
}

fn build_bytes(input: &str) -> String {
    let s = match syn::parse_str::<syn::LitStr>(input) {
        Ok(s) => s,
        _ => panic!("expected a string literal, got {}", input)
    };
    let cstr = match CString::new(s.value()) {
        Ok(s) => s,
        _ => panic!("literal must not contain zero char")
    };
    let mut bytes = String::from(r#"b""#);
    for &b in cstr.as_bytes().iter() {
        match b {
            b'\t' => bytes.push_str(r"\t"),
            b'\r' => bytes.push_str(r"\r"),
            b'\n' => bytes.push_str(r"\n"),
            b'\"' => bytes.push_str(r#"\""#),
            b'\\' => bytes.push_str(r"\\"),
            0x20...0x7e => bytes.push(char::from_u32(b as u32).unwrap()),
            _ => write!(&mut bytes, r"\x{:02x}", b).unwrap(),
        }
    }
    bytes.push_str(r#"\0""#);
    bytes
}

#[cfg(test)]
mod tests {
    use super::build_bytes;

    macro_rules! build_bytes {
        ($($t:tt)*) => {
            build_bytes(&quote!($($t)*).to_string())
        }
    }
    macro_rules! result {
        ($($t:tt)*) => {
            quote!($($t)*).to_string()
        }
    }

    #[test]
    fn test_build_bytes() {
        assert_eq!(build_bytes!("aaa"), result!(b"aaa\0"));
        assert_eq!(build_bytes!("\t\n\r\"\\'"), result!(b"\t\n\r\"\\'\0"));
        assert_eq!(build_bytes!("\x01\x02 \x7f"), result!(b"\x01\x02 \x7f\0"));
        assert_eq!(build_bytes!("你好"), result!(b"\xe4\xbd\xa0\xe5\xa5\xbd\0"));
    }

    #[test]
    #[should_panic]
    fn test_build_bytes_nul_inside() {
        build_bytes!("a\x00a");
    }
}
