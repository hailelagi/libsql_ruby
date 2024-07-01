/*
use libc::c_char;
use std::ffi::CStr;
use libsql::Builder;
*/

#[no_mangle]
pub extern "C" fn hello_rust() {
    println!("Hello this is rust!");
}

#[no_mangle]
pub extern "C" fn num() -> i32 {
    42
}
