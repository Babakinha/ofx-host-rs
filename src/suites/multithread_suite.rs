use crate::bindings::root;
use crate::bindings::root::{OfxMutexHandle, OfxStatus, OfxThreadFunctionV1};
use std::os::raw::{c_int, c_uint, c_void};

// ==========================================
// 1. THREAD SPAWNING & INFORMATION
// ==========================================

unsafe extern "C" fn multi_thread(
    func: OfxThreadFunctionV1,
    n_threads: c_uint,
    custom_arg: *mut c_void,
) -> OfxStatus {
    dbg!("multi_thread");
    let func = func.unwrap();
    dbg!(&func, &n_threads, &custom_arg);
    unsafe {
        func(0, 1, custom_arg);
    }
    0
}

unsafe extern "C" fn multi_thread_num_cpus(n_cpus: *mut c_uint) -> OfxStatus {
    dbg!("multi_thread_num_cpus");
    eprintln!("multiThreadNumCPUs called - reporting 1 CPU core");
    if !n_cpus.is_null() {
        unsafe {
            *n_cpus = 1;
        } // Safely declare 1 core to prevent divide-by-zero crashes
        0 // kOfxStatOK
    } else {
        1 // kOfxStatFailed
    }
}

unsafe extern "C" fn multi_thread_index(thread_index: *mut c_uint) -> OfxStatus {
    dbg!("multi_thread_index");
    eprintln!("multiThreadIndex called");
    if !thread_index.is_null() {
        unsafe {
            *thread_index = 0;
        } // Base index for single-threaded fallback
        0
    } else {
        1
    }
}

unsafe extern "C" fn multi_thread_is_spawned_thread() -> c_int {
    dbg!("multi_thread_is_spawned_thread");
    eprintln!("multiThreadIsSpawnedThread called");
    0 // Return false (0) since we didn't spawn it via the host thread pool
}

// ==========================================
// 2. MUTEX SYNCHRONIZATION POOL
// ==========================================

unsafe extern "C" fn mutex_create(_mutex: *mut OfxMutexHandle, _lock_count: c_int) -> OfxStatus {
    dbg!("mutex_create");
    eprintln!("mutexCreate not implemented");
    2
}

unsafe extern "C" fn mutex_destroy(_mutex: OfxMutexHandle) -> OfxStatus {
    dbg!("mutex_destroy");
    eprintln!("mutexDestroy not implemented");
    2
}

unsafe extern "C" fn mutex_lock(_mutex: OfxMutexHandle) -> OfxStatus {
    dbg!("mutex_lock");
    eprintln!("mutexLock not implemented");
    2
}

unsafe extern "C" fn mutex_unlock(_mutex: OfxMutexHandle) -> OfxStatus {
    dbg!("mutex_unlock");
    eprintln!("mutexUnLock not implemented");
    2
}

unsafe extern "C" fn mutex_try_lock(_mutex: OfxMutexHandle) -> OfxStatus {
    dbg!("mutex_try_lock");
    eprintln!("mutexTryLock not implemented");
    2
}

// ==========================================
// SUITE BUILDER
// ==========================================

pub fn multithread_suite() -> root::OfxMultiThreadSuiteV1 {
    dbg!("multithread_suite");
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
