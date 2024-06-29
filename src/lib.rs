use libc::c_char;
use std::ffi::CStr;
use libsql::Builder;

#[no_mangle]
pub extern "C" fn hello() {
    println!("Hello");
}

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
