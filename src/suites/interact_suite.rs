use crate::bindings::root;
use crate::bindings::root::{OfxInteractHandle, OfxPropertySetHandle, OfxStatus};
use crate::ofx_constants::{kOfxStatErrUnsupported, kOfxStatFailed};

use tracing::error;
use tracing::instrument;

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn interact_swap_buffers(_interact_instance: OfxInteractHandle) -> OfxStatus {
    error!("interact_swap_buffers not implemented");
    kOfxStatErrUnsupported
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn interact_redraw(_interact_instance: OfxInteractHandle) -> OfxStatus {
    error!("interact_redraw not implemented");
    kOfxStatErrUnsupported
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn interact_get_property_set(
    _interact_instance: OfxInteractHandle,
    property: *mut OfxPropertySetHandle,
) -> OfxStatus {
    error!("interact_get_property_set not implemented");
    if property.is_null() {
        error!("getPropertySet received a NULL handle");
        return kOfxStatFailed;
    }

    kOfxStatErrUnsupported
}

#[instrument(level = "trace", ret(level = "trace"))]
pub fn interact_suite() -> root::OfxInteractSuiteV1 {
    root::OfxInteractSuiteV1 {
        interactSwapBuffers: Some(interact_swap_buffers),
        interactRedraw: Some(interact_redraw),
        interactGetPropertySet: Some(interact_get_property_set),
    }
}
