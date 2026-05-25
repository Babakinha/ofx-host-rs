use crate::bindings::root::{self, OfxParamStruct, OfxPropertySetStruct};
use crate::bindings::root::{
    OfxParamHandle, OfxParamSetHandle, OfxPropertySetHandle, OfxRangeD, OfxStatus, OfxTime,
};
use crate::instance::{self, OfxHandle, PropertySet};
use crate::log_utils::c_str_to_str;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uint};
use tracing::error;
use tracing::{instrument, warn};

// ==========================================
// 1. DEFINITION & HANDLE FETCHING
// ==========================================

#[instrument(level = "trace", ret(level = "trace"), fields(name = c_str_to_str(name), param_type = c_str_to_str(param_type)))]
unsafe extern "C" fn param_define(
    param_set: OfxParamSetHandle,
    param_type: *const c_char,
    name: *const c_char,
    property_set: *mut OfxPropertySetHandle,
) -> OfxStatus {
    let instance_ptr = param_set as *mut OfxHandle;
    let instance = unsafe { &mut *instance_ptr };

    match &mut instance.target {
        crate::instance::OfxHandleTarget::BabaFx(babafx_instance) => {
            let c_str = unsafe { CStr::from_ptr(param_type) };
            let param_type_str = match c_str.to_str() {
                Ok(s) => s.to_string(),
                Err(_) => {
                    error!("Error");
                    return 1;
                } // kOfxStatFailed
            };

            let c_str = unsafe { CStr::from_ptr(name) };
            let name_str = match c_str.to_str() {
                Ok(s) => s.to_string(),
                Err(_) => {
                    error!("Error");
                    return 1;
                } // kOfxStatFailed
            };

            babafx_instance.parameters.insert(
                name_str.clone(),
                Box::new(OfxHandle {
                    target: crate::instance::OfxHandleTarget::ParameterThing(
                        crate::instance::ParameterThing {
                            name: name_str.clone(),
                            param_type: param_type_str,
                            properties: PropertySet::new(),
                        },
                    ),
                }),
            );

            if !property_set.is_null() {
                unsafe {
                    *property_set = babafx_instance
                        .parameters
                        .get_mut(&name_str)
                        .unwrap()
                        .as_mut() as *mut _
                        as *mut OfxPropertySetStruct;
                }
            }

            return 0;
        }
        _ => {
            error!("Error");
            return 1;
        } // kOfxStatFailed
    }
}

#[instrument(level = "trace", ret(level = "trace"), fields(name = c_str_to_str(name)))]
unsafe extern "C" fn param_get_handle(
    param_set: OfxParamSetHandle,
    name: *const c_char,
    param: *mut OfxParamHandle,
    property_set: *mut OfxPropertySetHandle,
) -> OfxStatus {
    let instance_ptr = param_set as *mut OfxHandle;
    let instance = unsafe { &mut *instance_ptr };

    let c_str = unsafe { CStr::from_ptr(name) };
    let name_str = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Error");
            return 1;
        } // kOfxStatFailed
    };

    if let instance::OfxHandleTarget::BabaFx(babafx) = &mut instance.target {
        if let Some(parameter) = babafx.parameters.get_mut(&name_str) {
            if !param.is_null() {
                unsafe {
                    *param = parameter.as_mut() as *mut _ as *mut OfxParamStruct;
                }
            }

            if !property_set.is_null() {
                unsafe {
                    // TODO: Techinically will work... i think?
                    *property_set = parameter.as_mut() as *mut _ as *mut OfxPropertySetStruct;
                }
            }
        } else {
            error!("Error");
            return 1;
        }
    } else {
        error!("Error");
        return 1; // Error
    }

    0
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_set_get_property_set(
    _param_set: OfxParamSetHandle,
    _prop_handle: *mut OfxPropertySetHandle,
) -> OfxStatus {
    error!("paramSetGetPropertySet not implemented");
    2
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_get_property_set(
    _param: OfxParamHandle,
    _prop_handle: *mut OfxPropertySetHandle,
) -> OfxStatus {
    error!("paramGetPropertySet not implemented");
    2
}

// ==========================================
// 2. VALUE GETTERS & EVALUATION
// ==========================================

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_get_value(_param_handle: OfxParamHandle, _: ...) -> OfxStatus {
    error!("paramGetValue not implemented");
    2
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_get_value_at_time(
    param_handle: OfxParamHandle,
    _time: OfxTime,
    mut args: ...
) -> OfxStatus {
    warn!("param_get_value_at_time half implemented");

    let instance_ptr = param_handle as *mut OfxHandle;
    let instance = unsafe { &mut *instance_ptr };

    if let instance::OfxHandleTarget::ParameterThing(param) = &mut instance.target {
        match param.param_type.as_str() {
            "OfxParamTypeDouble2D" => {
                unsafe {
                    // Pull two separate pointers off the variadic stack frame
                    let x_ptr = args.next_arg::<*mut f64>();
                    let y_ptr = args.next_arg::<*mut f64>();

                    if x_ptr.is_null() || y_ptr.is_null() {
                        error!("Error");
                        return 1; // kOfxStatErrBadHandle
                    }

                    if let Some(vals) = param.properties.doubles.get("OfxParamPropDefault") {
                        *x_ptr = vals[0];
                        *y_ptr = vals[1];
                        return 0;
                    }
                    error!("Error");
                    return 1;
                }
            }
            "OfxParamTypeRGBA" => {
                unsafe {
                    // Pull four separate pointers off the variadic stack frame
                    let r_ptr = args.next_arg::<*mut f64>();
                    let g_ptr = args.next_arg::<*mut f64>();
                    let b_ptr = args.next_arg::<*mut f64>();
                    let a_ptr = args.next_arg::<*mut f64>();

                    if r_ptr.is_null() || g_ptr.is_null() || b_ptr.is_null() || a_ptr.is_null() {
                        error!("Error");
                        return 1;
                    }

                    // Write to each individual scalar pointer target
                    *r_ptr = 1.0;
                    *g_ptr = 0.7;
                    *b_ptr = 0.7;
                    *a_ptr = 1.0;

                    return 0;
                }
            }
            a => error!("{a} Not implemented for now"),
        }
    } else {
        error!("Error");
        return 1; // Error
    }

    0
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_get_derivative(
    _param_handle: OfxParamHandle,
    _time: OfxTime,
    _: ...
) -> OfxStatus {
    error!("paramGetDerivative not implemented");
    2
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_get_integral(
    _param_handle: OfxParamHandle,
    _time1: OfxTime,
    _time2: OfxTime,
    _: ...
) -> OfxStatus {
    error!("paramGetIntegral not implemented");
    2
}

// ==========================================
// 3. VALUE SETTERS & KEYFRAMES
// ==========================================

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_set_value(_param_handle: OfxParamHandle, _: ...) -> OfxStatus {
    error!("paramSetValue not implemented");
    2
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_set_value_at_time(
    _param_handle: OfxParamHandle,
    _time: OfxTime,
    _: ...
) -> OfxStatus {
    error!("paramSetValueAtTime not implemented");
    2
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_get_num_keys(
    _param_handle: OfxParamHandle,
    _number_of_keys: *mut c_uint,
) -> OfxStatus {
    error!("paramGetNumKeys not implemented");
    2
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_get_key_time(
    _param_handle: OfxParamHandle,
    _nth_key: c_uint,
    _time: *mut OfxTime,
) -> OfxStatus {
    error!("paramGetKeyTime not implemented");
    2
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_get_key_index(
    _param_handle: OfxParamHandle,
    _time: OfxTime,
    _direction: c_int,
    _index: *mut c_int,
) -> OfxStatus {
    error!("paramGetKeyIndex not implemented");
    2
}

// ==========================================
// 4. MANAGEMENT & OPERATIONS
// ==========================================

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_delete_key(_param_handle: OfxParamHandle, _time: OfxTime) -> OfxStatus {
    error!("paramDeleteKey not implemented");
    2
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_delete_all_keys(_param_handle: OfxParamHandle) -> OfxStatus {
    error!("paramDeleteAllKeys not implemented");
    2
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_copy(
    _param_to: OfxParamHandle,
    _param_from: OfxParamHandle,
    _dst_offset: OfxTime,
    _frame_range: *const OfxRangeD,
) -> OfxStatus {
    error!("paramCopy not implemented");
    2
}

// ==========================================
// 5. UNDO/REDO TRANSACTION BLOCKS
// ==========================================

#[instrument(level = "trace", ret(level = "trace"), fields(name = c_str_to_str(_name)))]
unsafe extern "C" fn param_edit_begin(
    _param_set: OfxParamSetHandle,
    _name: *const c_char,
) -> OfxStatus {
    error!("paramEditBegin not implemented");
    2
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_edit_end(_param_set: OfxParamSetHandle) -> OfxStatus {
    error!("paramEditEnd not implemented");
    2
}

// Suite builder

#[instrument(level = "trace", ret(level = "trace"))]
pub fn parameter_suite() -> root::OfxParameterSuiteV1 {
    root::OfxParameterSuiteV1 {
        paramDefine: Some(param_define),
        paramGetHandle: Some(param_get_handle),
        paramSetGetPropertySet: Some(param_set_get_property_set),
        paramGetPropertySet: Some(param_get_property_set),
        paramGetValue: Some(param_get_value),
        paramGetValueAtTime: Some(param_get_value_at_time),
        paramGetDerivative: Some(param_get_derivative),
        paramGetIntegral: Some(param_get_integral),
        paramSetValue: Some(param_set_value),
        paramSetValueAtTime: Some(param_set_value_at_time),
        paramGetNumKeys: Some(param_get_num_keys),
        paramGetKeyTime: Some(param_get_key_time),
        paramGetKeyIndex: Some(param_get_key_index),
        paramDeleteKey: Some(param_delete_key),
        paramDeleteAllKeys: Some(param_delete_all_keys),
        paramCopy: Some(param_copy),
        paramEditBegin: Some(param_edit_begin),
        paramEditEnd: Some(param_edit_end),
    }
}
