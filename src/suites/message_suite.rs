use std::os::raw::c_char;
use tracing::error;
use tracing::instrument;

use crate::bindings::root;
use crate::bindings::root::OfxStatus;

// ==========================================
// 1. MESSAGING IMPLEMENTATION
// ==========================================

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn message(
    _handle: *mut std::os::raw::c_void,
    _message_type: *const c_char,
    _message_id: *const c_char,
    _format: *const c_char,
    _: ...
) -> OfxStatus {
    error!("OfxMessageSuiteV1::message called");
    0 // kOfxStatOK
}

// ==========================================
// SUITE BUILDER
// ==========================================

#[instrument(level = "trace", ret(level = "trace"))]
pub fn message_suite() -> root::OfxMessageSuiteV1 {
    root::OfxMessageSuiteV1 {
        message: Some(message),
    }
}
