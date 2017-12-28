use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::mem;

use rlox::api::run_string;

#[no_mangle]
pub fn run_from_wasm(data: *const c_char) -> *const c_char {
    let incoming_str;

    unsafe {
        incoming_str = CStr::from_ptr(data).to_str().unwrap().to_owned();
    }

    CString::new(run_string(incoming_str)).unwrap().into_raw()
}

#[no_mangle]
pub fn alloc(size: usize) -> *const c_void {
    let buf = Vec::with_capacity(size);
    let ptr = buf.as_ptr();
    mem::forget(buf);
    ptr
}
