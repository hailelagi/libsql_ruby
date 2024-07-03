use libc;

use libc::c_char;
use std::ffi::CStr;
use libsql::Builder;

#[allow(non_upper_case_globals)]
static Qnil: libc::uintptr_t = 0x08 as libc::uintptr_t;


#[no_mangle]
pub extern "C" fn hello_rust() {
    println!("Hello this is rust!");
}

#[no_mangle]
pub extern "C" fn num() -> i32 {
    42
}

#[no_mangle]
pub unsafe extern fn side_effect() -> libc::uintptr_t {
    // make sure libc is linked and we didn't break anything
    print!("side effect returns nil");
    Qnil
}
