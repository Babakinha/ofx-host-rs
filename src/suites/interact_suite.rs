use crate::bindings::root;
use crate::bindings::root::{OfxInteractHandle, OfxPropertySetHandle, OfxStatus};

// ==========================================
// 1. INTERACT VIEWPORT & RENDERING
// ==========================================

unsafe extern "C" fn interact_swap_buffers(_interact_instance: OfxInteractHandle) -> OfxStatus {
    dbg!("interact_swap_buffers");
    eprintln!("OfxInteractSuiteV1::interactSwapBuffers called");
    2 // kOfxStatErrUnsupported
}

unsafe extern "C" fn interact_redraw(_interact_instance: OfxInteractHandle) -> OfxStatus {
    dbg!("interact_redraw");
    eprintln!("OfxInteractSuiteV1::interactRedraw called");
    2 // kOfxStatErrUnsupported
}

// ==========================================
// 2. PROPERTY SET ACCESS
// ==========================================

unsafe extern "C" fn interact_get_property_set(
    _interact_instance: OfxInteractHandle,
    property: *mut OfxPropertySetHandle,
) -> OfxStatus {
    dbg!("interact_get_property_set");
    eprintln!("OfxInteractSuiteV1::interactGetPropertySet called");
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

pub fn interact_suite() -> root::OfxInteractSuiteV1 {
    dbg!("interact_suite");
    root::OfxInteractSuiteV1 {
        interactSwapBuffers: Some(interact_swap_buffers),
        interactRedraw: Some(interact_redraw),
        interactGetPropertySet: Some(interact_get_property_set),
    }
}
