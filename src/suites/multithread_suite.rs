use crate::bindings::root::{self};
use crate::bindings::root::{OfxMutexHandle, OfxStatus, OfxThreadFunctionV1};
use crate::ofx_constants::{kOfxStatErrUnsupported, kOfxStatFailed, kOfxStatOK};
use std::os::raw::{c_int, c_uint, c_void};
use tracing::{error, instrument, warn};

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn multi_thread(
    func: OfxThreadFunctionV1,
    n_threads: c_uint,
    custom_arg: *mut c_void,
) -> OfxStatus {
    warn!("multi_thread half implemented");
    let func = func.unwrap();
    unsafe {
        func(0, 1, custom_arg);
    }
    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn multi_thread_num_cpus(n_cpus: *mut c_uint) -> OfxStatus {
    warn!("multi_thread_num_cpus half implemented");
    if !n_cpus.is_null() {
        unsafe {
            *n_cpus = 1;
        }
        kOfxStatOK
    } else {
        error!("Error!");
        kOfxStatFailed
    }
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn multi_thread_index(thread_index: *mut c_uint) -> OfxStatus {
    warn!("multi_thread_index half implemented");
    if !thread_index.is_null() {
        unsafe {
            *thread_index = 0;
        }
        kOfxStatOK
    } else {
        error!("Error!");
        kOfxStatFailed
    }
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn multi_thread_is_spawned_thread() -> c_int {
    error!("multi_thread_is_spawned_thread not implemented");
    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn mutex_create(_mutex: *mut OfxMutexHandle, _lock_count: c_int) -> OfxStatus {
    error!("mutex_create not implemented");
    kOfxStatErrUnsupported
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn mutex_destroy(_mutex: OfxMutexHandle) -> OfxStatus {
    error!("mutex_destroy not implemented");
    kOfxStatErrUnsupported
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn mutex_lock(_mutex: OfxMutexHandle) -> OfxStatus {
    error!("mutex_lock not implemented");
    kOfxStatErrUnsupported
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn mutex_unlock(_mutex: OfxMutexHandle) -> OfxStatus {
    error!("mutex_unlock not implemented");
    kOfxStatErrUnsupported
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn mutex_try_lock(_mutex: OfxMutexHandle) -> OfxStatus {
    error!("mutex_try_lock not implemented");
    kOfxStatErrUnsupported
}

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
