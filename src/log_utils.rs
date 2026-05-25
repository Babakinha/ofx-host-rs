use std::ffi::{CStr, c_char};

pub fn c_str_to_str(ptr: *const c_char) -> String {
    if ptr.is_null() {
        "<null>".to_string()
    } else {
        unsafe {
            CStr::from_ptr(ptr)
                .to_str()
                .unwrap_or(format!("<invalid utf-8> ({ptr:?})").as_str())
                .to_string()
        }
    }
}
