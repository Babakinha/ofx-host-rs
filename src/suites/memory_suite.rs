use crate::bindings::root::OfxStatus;
use crate::bindings::root::{self};
use crate::ofx_constants::{kOfxStatErrBadHandle, kOfxStatErrMemory, kOfxStatFailed, kOfxStatOK};
use std::alloc::{Layout, alloc, dealloc};
use std::os::raw::c_void;
use tracing::{error, instrument, warn};

const ALIGNMENT: usize = 16;

#[repr(C)]
struct MemoryHeader {
    total_size: usize,
}

const HEADER_PADDING: usize = {
    let header_size = std::mem::size_of::<MemoryHeader>();
    if header_size % ALIGNMENT == 0 {
        header_size
    } else {
        ((header_size / ALIGNMENT) + 1) * ALIGNMENT
    }
};

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn memory_alloc(
    _handle: *mut c_void,
    n_bytes: usize,
    allocated_data: *mut *mut c_void,
) -> OfxStatus {
    warn!("memory_alloc half implemented");
    if allocated_data.is_null() || n_bytes == 0 {
        error!("Error!");
        return kOfxStatFailed;
    }

    let total_size = HEADER_PADDING + n_bytes;
    match Layout::from_size_align(total_size, ALIGNMENT) {
        Ok(layout) => {
            unsafe {
                let raw_ptr = alloc(layout);
                if raw_ptr.is_null() {
                    error!("Out of memory trying to allocate {} bytes", n_bytes);
                    return kOfxStatErrMemory;
                }

                let header_ptr = raw_ptr as *mut MemoryHeader;
                *header_ptr = MemoryHeader { total_size };

                let client_ptr = raw_ptr.add(HEADER_PADDING);
                *allocated_data = client_ptr as *mut c_void;
            }
            kOfxStatOK
        }
        Err(_) => {
            error!("Couldn't allocate layout.");
            kOfxStatFailed
        }
    }
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn memory_free(allocated_data: *mut c_void) -> OfxStatus {
    if allocated_data.is_null() {
        error!("Error!");
        return kOfxStatOK;
    }

    unsafe {
        let raw_ptr = (allocated_data as *mut u8).sub(HEADER_PADDING);
        let total_size = (*(raw_ptr as *mut MemoryHeader)).total_size;

        if let Ok(layout) = Layout::from_size_align(total_size, ALIGNMENT) {
            dealloc(raw_ptr, layout);
            return kOfxStatOK;
        } else {
            error!("Couldn't allocate layout.");
            return kOfxStatErrBadHandle;
        }
    }
}

#[instrument(level = "trace", ret(level = "trace"))]
pub fn memory_suite() -> root::OfxMemorySuiteV1 {
    root::OfxMemorySuiteV1 {
        memoryAlloc: Some(memory_alloc),
        memoryFree: Some(memory_free),
    }
}
