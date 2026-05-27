use crate::bindings::root;
use crate::bindings::root::{OfxInteractHandle, OfxPropertySetHandle, OfxStatus};
use crate::ofx_constants::{kOfxStatErrUnsupported, kOfxStatFailed};

use tracing::error;
use tracing::instrument;

// ==========================================
// 1. INTERACT VIEWPORT & RENDERING
// ==========================================

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn interact_swap_buffers(_interact_instance: OfxInteractHandle) -> OfxStatus {
    error!("OfxInteractSuiteV1::interactSwapBuffers called");
    kOfxStatErrUnsupported
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn interact_redraw(_interact_instance: OfxInteractHandle) -> OfxStatus {
    error!("OfxInteractSuiteV1::interactRedraw called");
    kOfxStatErrUnsupported
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
        return kOfxStatFailed;
    }

    // Once you fully implement Interacts, you will attach an OfxPropertySetHandle
    // to your interact backing instance here.
    kOfxStatErrUnsupported
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
