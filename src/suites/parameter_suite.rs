use crate::bindings::root::{self, OfxParamSetStruct, OfxParamStruct, OfxPropertySetStruct};
use crate::bindings::root::{
    OfxParamHandle, OfxParamSetHandle, OfxPropertySetHandle, OfxRangeD, OfxStatus, OfxTime,
};
use crate::instance::{self, OfxHandle, PropertySet};
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uint};

// ==========================================
// 1. DEFINITION & HANDLE FETCHING
// ==========================================

unsafe extern "C" fn param_define(
    param_set: OfxParamSetHandle,
    param_type: *const c_char,
    name: *const c_char,
    property_set: *mut OfxPropertySetHandle,
) -> OfxStatus {
    dbg!("param_define");
    let instance_ptr = param_set as *mut OfxHandle;
    let instance = unsafe { &mut *instance_ptr };

    match &mut instance.target {
        crate::instance::OfxHandleTarget::BabaFx(babafx_instance) => {
            let c_str = unsafe { CStr::from_ptr(param_type) };
            let param_type_str = match c_str.to_str() {
                Ok(s) => s.to_string(),
                Err(_) => return 1, // kOfxStatFailed
            };

            let c_str = unsafe { CStr::from_ptr(name) };
            let name_str = match c_str.to_str() {
                Ok(s) => s.to_string(),
                Err(_) => return 1, // kOfxStatFailed
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
                    *property_set = babafx_instance.parameters.get_mut(&name_str).unwrap().as_mut() as *mut _
                        as *mut OfxPropertySetStruct;
                }
            }

            return 0;
        }
        _ => return 1, // kOfxStatFailed
    }
}

unsafe extern "C" fn param_get_handle(
    param_set: OfxParamSetHandle,
    name: *const c_char,
    param: *mut OfxParamHandle,
    property_set: *mut OfxPropertySetHandle,
) -> OfxStatus {
    dbg!("param_get_handle");
    let instance_ptr = param_set as *mut OfxHandle;
    let instance = unsafe { &mut *instance_ptr };

    let c_str = unsafe { CStr::from_ptr(name) };
    let name_str = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return 1, // kOfxStatFailed
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
            return 1;
        }
    } else {
        return 1; // Error
    }

    0
}

unsafe extern "C" fn param_set_get_property_set(
    _param_set: OfxParamSetHandle,
    _prop_handle: *mut OfxPropertySetHandle,
) -> OfxStatus {
    dbg!("param_set_get_property_set");
    eprintln!("paramSetGetPropertySet not implemented");
    2
}

unsafe extern "C" fn param_get_property_set(
    _param: OfxParamHandle,
    _prop_handle: *mut OfxPropertySetHandle,
) -> OfxStatus {
    dbg!("param_get_property_set");
    eprintln!("paramGetPropertySet not implemented");
    2
}

// ==========================================
// 2. VALUE GETTERS & EVALUATION
// ==========================================

unsafe extern "C" fn param_get_value(_param_handle: OfxParamHandle, _: ...) -> OfxStatus {
    dbg!("param_get_value");
    eprintln!("paramGetValue not implemented");
    2
}

unsafe extern "C" fn param_get_value_at_time(
    _param_handle: OfxParamHandle,
    _time: OfxTime,
    _: ...
) -> OfxStatus {
    dbg!("param_get_value_at_time");
    eprintln!("paramGetValueAtTime not implemented");
    2
}

unsafe extern "C" fn param_get_derivative(
    _param_handle: OfxParamHandle,
    _time: OfxTime,
    _: ...
) -> OfxStatus {
    dbg!("param_get_derivative");
    eprintln!("paramGetDerivative not implemented");
    2
}

unsafe extern "C" fn param_get_integral(
    _param_handle: OfxParamHandle,
    _time1: OfxTime,
    _time2: OfxTime,
    _: ...
) -> OfxStatus {
    dbg!("param_get_integral");
    eprintln!("paramGetIntegral not implemented");
    2
}

// ==========================================
// 3. VALUE SETTERS & KEYFRAMES
// ==========================================

unsafe extern "C" fn param_set_value(_param_handle: OfxParamHandle, _: ...) -> OfxStatus {
    dbg!("param_set_value");
    eprintln!("paramSetValue not implemented");
    2
}

unsafe extern "C" fn param_set_value_at_time(
    _param_handle: OfxParamHandle,
    _time: OfxTime,
    _: ...
) -> OfxStatus {
    dbg!("param_set_value_at_time");
    eprintln!("paramSetValueAtTime not implemented");
    2
}

unsafe extern "C" fn param_get_num_keys(
    _param_handle: OfxParamHandle,
    _number_of_keys: *mut c_uint,
) -> OfxStatus {
    dbg!("param_get_num_keys");
    eprintln!("paramGetNumKeys not implemented");
    2
}

unsafe extern "C" fn param_get_key_time(
    _param_handle: OfxParamHandle,
    _nth_key: c_uint,
    _time: *mut OfxTime,
) -> OfxStatus {
    dbg!("param_get_key_time");
    eprintln!("paramGetKeyTime not implemented");
    2
}

unsafe extern "C" fn param_get_key_index(
    _param_handle: OfxParamHandle,
    _time: OfxTime,
    _direction: c_int,
    _index: *mut c_int,
) -> OfxStatus {
    dbg!("param_get_key_index");
    eprintln!("paramGetKeyIndex not implemented");
    2
}

// ==========================================
// 4. MANAGEMENT & OPERATIONS
// ==========================================

unsafe extern "C" fn param_delete_key(_param_handle: OfxParamHandle, _time: OfxTime) -> OfxStatus {
    dbg!("param_delete_key");
    eprintln!("paramDeleteKey not implemented");
    2
}

unsafe extern "C" fn param_delete_all_keys(_param_handle: OfxParamHandle) -> OfxStatus {
    dbg!("param_delete_all_keys");
    eprintln!("paramDeleteAllKeys not implemented");
    2
}

unsafe extern "C" fn param_copy(
    _param_to: OfxParamHandle,
    _param_from: OfxParamHandle,
    _dst_offset: OfxTime,
    _frame_range: *const OfxRangeD,
) -> OfxStatus {
    dbg!("param_copy");
    eprintln!("paramCopy not implemented");
    2
}

// ==========================================
// 5. UNDO/REDO TRANSACTION BLOCKS
// ==========================================

unsafe extern "C" fn param_edit_begin(
    _param_set: OfxParamSetHandle,
    _name: *const c_char,
) -> OfxStatus {
    dbg!("param_edit_begin");
    eprintln!("paramEditBegin not implemented");
    2
}

unsafe extern "C" fn param_edit_end(_param_set: OfxParamSetHandle) -> OfxStatus {
    dbg!("param_edit_end");
    eprintln!("paramEditEnd not implemented");
    2
}

// Suite builder

pub fn parameter_suite() -> root::OfxParameterSuiteV1 {
    dbg!("parameter_suite");
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
