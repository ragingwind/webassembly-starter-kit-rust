use std::ffi::CString;
use std::os::raw::c_char;

pub fn main() {}

static HELLO: &'static str = "Hello, WASM!";

#[no_mangle]
pub fn hello_offset() -> *mut c_char {
  let s = CString::new(HELLO).unwrap();
  s.into_raw()
}

#[no_mangle]
pub fn hello_len() -> usize {
    HELLO.len()
}