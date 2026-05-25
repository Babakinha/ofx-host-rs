use crate::bindings::root;
use crate::bindings::root::{OfxMutexHandle, OfxStatus, OfxThreadFunctionV1};
use std::os::raw::{c_int, c_uint, c_void};
use tracing::{debug, error, instrument};

// ==========================================
// 1. THREAD SPAWNING & INFORMATION
// ==========================================

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn multi_thread(
    func: OfxThreadFunctionV1,
    n_threads: c_uint,
    custom_arg: *mut c_void,
) -> OfxStatus {
    let func = func.unwrap();
    unsafe {
        func(0, 1, custom_arg);
    }
    0
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn multi_thread_num_cpus(n_cpus: *mut c_uint) -> OfxStatus {
    debug!("multiThreadNumCPUs called - reporting 1 CPU core");
    if !n_cpus.is_null() {
        unsafe {
            *n_cpus = 1;
        } // Safely declare 1 core to prevent divide-by-zero crashes
        0 // kOfxStatOK
    } else {
        1 // kOfxStatFailed
    }
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn multi_thread_index(thread_index: *mut c_uint) -> OfxStatus {
    debug!("multiThreadIndex called");
    if !thread_index.is_null() {
        unsafe {
            *thread_index = 0;
        } // Base index for single-threaded fallback
        0
    } else {
        1
    }
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn multi_thread_is_spawned_thread() -> c_int {
    error!("multiThreadIsSpawnedThread not implemented");
    0 // Return false (0) since we didn't spawn it via the host thread pool
}

// ==========================================
// 2. MUTEX SYNCHRONIZATION POOL
// ==========================================

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn mutex_create(_mutex: *mut OfxMutexHandle, _lock_count: c_int) -> OfxStatus {
    error!("mutexCreate not implemented");
    2
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn mutex_destroy(_mutex: OfxMutexHandle) -> OfxStatus {
    error!("mutexDestroy not implemented");
    2
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn mutex_lock(_mutex: OfxMutexHandle) -> OfxStatus {
    error!("mutexLock not implemented");
    2
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn mutex_unlock(_mutex: OfxMutexHandle) -> OfxStatus {
    error!("mutexUnLock not implemented");
    2
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn mutex_try_lock(_mutex: OfxMutexHandle) -> OfxStatus {
    error!("mutexTryLock not implemented");
    2
}

// ==========================================
// SUITE BUILDER
// ==========================================

#[instrument(level = "trace", ret(level = "trace"))]
pub fn multithread_suite() -> root::OfxMultiThreadSuiteV1 {
    root::OfxMultiThreadSuiteV1 {
        multiThread: Some(multi_thread),
        multiThreadNumCPUs: Some(multi_thread_num_cpus),
        multiThreadIndex: Some(multi_thread_index),
        multiThreadIsSpawnedThread: Some(multi_thread_is_spawned_thread),
        mutexCreate: Some(mutex_create),
        mutexDestroy: Some(mutex_destroy),
        mutexLock: Some(mutex_lock),
        mutexUnLock: Some(mutex_unlock),
        mutexTryLock: Some(mutex_try_lock),
    }
}
