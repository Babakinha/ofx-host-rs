use crate::bindings::root;
use crate::bindings::root::{OfxInteractHandle, OfxPropertySetHandle, OfxStatus};

use tracing::error;
use tracing::instrument;

// ==========================================
// 1. INTERACT VIEWPORT & RENDERING
// ==========================================

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn interact_swap_buffers(_interact_instance: OfxInteractHandle) -> OfxStatus {
    error!("OfxInteractSuiteV1::interactSwapBuffers called");
    2 // kOfxStatErrUnsupported
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn interact_redraw(_interact_instance: OfxInteractHandle) -> OfxStatus {
    error!("OfxInteractSuiteV1::interactRedraw called");
    2 // kOfxStatErrUnsupported
}

// ==========================================
// 2. PROPERTY SET ACCESS
// ==========================================

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn interact_get_property_set(
    _interact_instance: OfxInteractHandle,
    property: *mut OfxPropertySetHandle,
) -> OfxStatus {
    error!("OfxInteractSuiteV1::interactGetPropertySet called");
    if property.is_null() {
        return 1; // kOfxStatFailed
    }

    // Once you fully implement Interacts, you will attach an OfxPropertySetHandle
    // to your interact backing instance here.
    2 // kOfxStatErrUnsupported
}

// ==========================================
// SUITE BUILDER
// ==========================================

#[instrument(level = "trace", ret(level = "trace"))]
pub fn interact_suite() -> root::OfxInteractSuiteV1 {
    root::OfxInteractSuiteV1 {
        interactSwapBuffers: Some(interact_swap_buffers),
        interactRedraw: Some(interact_redraw),
        interactGetPropertySet: Some(interact_get_property_set),
    }
}
