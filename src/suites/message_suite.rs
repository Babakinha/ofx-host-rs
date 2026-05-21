use std::os::raw::c_char;
use crate::bindings::root;
use crate::bindings::root::OfxStatus;

// ==========================================
// 1. MESSAGING IMPLEMENTATION
// ==========================================

unsafe extern "C" fn message(
    _handle: *mut std::os::raw::c_void,
    _message_type: *const c_char,
    _message_id: *const c_char,
    _format: *const c_char,
    _: ...
) -> OfxStatus {
    dbg!("message");
    eprintln!("OfxMessageSuiteV1::message called (log formatting not implemented)");
    0 // kOfxStatOK
}

// ==========================================
// SUITE BUILDER
// ==========================================

pub fn message_suite() -> root::OfxMessageSuiteV1 {
    dbg!("message_suite");
    root::OfxMessageSuiteV1 {
        message: Some(message),
    }
}
