use std::alloc::{alloc, dealloc, Layout};
use std::os::raw::c_void;
use crate::bindings::root;
use crate::bindings::root::OfxStatus;

const ALIGNMENT: usize = 16;

// ==========================================
// 1. MEMORY MANAGEMENT IMPLEMENTATION
// ==========================================

unsafe extern "C" fn memory_alloc(
    _handle: *mut c_void,
    n_bytes: usize,
    allocated_data: *mut *mut c_void,
) -> OfxStatus {
    dbg!("memory_alloc");
    if allocated_data.is_null() || n_bytes == 0 {
        return 1; // kOfxStatFailed
    }

    // We allocate extra space at the front to store the total allocation size
    // so we can reconstruct the exact Layout when memory_free is called.
    let padding = ALIGNMENT; 
    let total_size = n_bytes + padding;

    match Layout::from_size_align(total_size, ALIGNMENT) {
        Ok(layout) => {
            let raw_ptr = alloc(layout);
            if raw_ptr.is_null() {
                eprintln!("OfxMemorySuiteV1: Out of memory trying to allocate {} bytes", n_bytes);
                return 3; // kOfxStatErrMemory
            }

            // Write the total size metadata into the hidden prefix area
            *(raw_ptr as *mut usize) = total_size;

            // Offset the pointer passed back to the C plugin past our metadata
            let client_ptr = raw_ptr.add(padding);
            *allocated_data = client_ptr as *mut c_void;
            
            0 // kOfxStatOK
        }
        Err(_) => 1, // kOfxStatFailed
    }
}

unsafe extern "C" fn memory_free(allocated_data: *mut c_void) -> OfxStatus {
    dbg!("memory_free");
    if allocated_data.is_null() {
        return 0; // Freeing a null pointer is explicitly allowed and a no-op
    }

    // Move backward to reveal the hidden metadata address block
    let padding = ALIGNMENT;
    let raw_ptr = (allocated_data as *mut u8).sub(padding);

    // Read the total allocation size we stored during memory_alloc
    let total_size = *(raw_ptr as *mut usize);

    if let Ok(layout) = Layout::from_size_align(total_size, ALIGNMENT) {
        dealloc(raw_ptr, layout);
        0 // kOfxStatOK
    } else {
        4 // kOfxStatErrBadHandle
    }
}

// ==========================================
// SUITE BUILDER
// ==========================================

pub fn memory_suite() -> root::OfxMemorySuiteV1 {
    dbg!("memory_suite");
    root::OfxMemorySuiteV1 {
        memoryAlloc: Some(memory_alloc),
        memoryFree: Some(memory_free),
    }
}
