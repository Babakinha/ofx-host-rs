use crate::bindings::root::{self, OfxImageEffectHandle};
use crate::bindings::root::{
    OfxParamHandle, OfxParamSetHandle, OfxPropertySetHandle, OfxRangeD, OfxStatus, OfxTime,
};
use crate::instance::{self, AsPropertySet, BabafxInstance, ParameterValue, PropertySet};
use crate::log_utils::c_str_to_str;
use crate::ofx_constants::{kOfxStatErrBadHandle, kOfxStatOK};
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
    let instance = unsafe {
        BabafxInstance::ref_mut_from_ofx_handle(param_set as OfxImageEffectHandle).unwrap()
    };

    let c_str = unsafe { CStr::from_ptr(param_type) };
    let param_type_str = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Error");
            return 1;
        }
    };

    let c_str = unsafe { CStr::from_ptr(name) };
    let name_str = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Error");
            return 1;
        }
    };

    instance.parameters.insert(
        name_str.clone(),
        crate::instance::ParameterThing {
            name: name_str.clone(),
            value: match param_type_str.as_str() {
                "OfxParamTypeInteger" => instance::ParameterValue::Integer(None),
                "OfxParamTypeInteger2D" => instance::ParameterValue::Integer2D(None),
                "OfxParamTypeInteger3D" => instance::ParameterValue::Integer3D(None),
                "OfxParamTypeDouble" => instance::ParameterValue::Double(None),
                "OfxParamTypeDouble2D" => instance::ParameterValue::Double2D(None),
                "OfxParamTypeDouble3D" => instance::ParameterValue::Double3D(None),
                "OfxParamTypeRGB" => instance::ParameterValue::RGB(None),
                "OfxParamTypeRGBA" => instance::ParameterValue::RGBA(None),
                _ => instance::ParameterValue::None,
            },
            properties: PropertySet::new(),
        },
    );

    if !property_set.is_null() {
        unsafe {
            *property_set = instance
                .parameters
                .get_mut(&name_str)
                .unwrap()
                .get_properties_mut()
                .as_raw_ofx_handle();
        }
    }

    return 0;
}

#[instrument(level = "trace", ret(level = "trace"), fields(name = c_str_to_str(name)))]
unsafe extern "C" fn param_get_handle(
    param_set: OfxParamSetHandle,
    name: *const c_char,
    param: *mut OfxParamHandle,
    property_set: *mut OfxPropertySetHandle,
) -> OfxStatus {
    let instance = unsafe {
        BabafxInstance::ref_mut_from_ofx_handle(param_set as OfxImageEffectHandle).unwrap()
    };

    let c_str = unsafe { CStr::from_ptr(name) };
    let name_str = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Error");
            return 1;
        }
    };

    if let Some(parameter) = instance.parameters.get_mut(&name_str) {
        if !param.is_null() {
            unsafe {
                *param = parameter.as_raw_ofx_handle();
            }
        }

        if !property_set.is_null() {
            unsafe {
                *property_set = parameter.get_properties_mut().as_raw_ofx_handle();
            }
        }
    } else {
        error!("Error");
        return 1;
    }

    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_set_get_property_set(
    param_set: OfxParamSetHandle,
    prop_handle: *mut OfxPropertySetHandle,
) -> OfxStatus {
    if param_set.is_null() || prop_handle.is_null() {
        error!("getPropertySet received a NULL handle");
        return kOfxStatErrBadHandle;
    }
    let instance = unsafe {
        BabafxInstance::ref_mut_from_ofx_handle(param_set as OfxImageEffectHandle).unwrap()
    };

    unsafe {
        *prop_handle = instance.get_properties_mut().as_raw_ofx_handle();
    }

    kOfxStatOK
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

    let instance =
        unsafe { instance::ParameterThing::ref_mut_from_ofx_handle(param_handle).unwrap() };

    let or_default_int = |index| {
        *(instance
            .properties
            .ints
            .get("OfxParamPropDefault")
            .unwrap_or(&vec![])
            .get(index)
            .unwrap_or(&0))
    };

    let or_default_double = |index| {
        *(instance
            .properties
            .doubles
            .get("OfxParamPropDefault")
            .unwrap_or(&vec![])
            .get(index)
            .unwrap_or(&0.0))
    };
    match instance.value {
        ParameterValue::Integer(x) => {
            unsafe {
                // Pull two separate pointers off the variadic stack frame
                let x_ptr = args.next_arg::<*mut i32>();

                if x_ptr.is_null() {
                    error!("Error");
                    return 1;
                }

                *x_ptr = x.unwrap_or_else(|| or_default_int(0));
                return 0;
            }
        }
        ParameterValue::Integer2D(value) => {
            unsafe {
                // Pull two separate pointers off the variadic stack frame
                let x_ptr = args.next_arg::<*mut i32>();
                let y_ptr = args.next_arg::<*mut i32>();

                if x_ptr.is_null() || y_ptr.is_null() {
                    error!("Error");
                    return 1;
                }

                let (x, y) = value.unwrap_or_else(|| (or_default_int(0), or_default_int(1)));
                *x_ptr = x;
                *y_ptr = y;
                return 0;
            }
        }
        ParameterValue::Integer3D(value) => {
            unsafe {
                // Pull two separate pointers off the variadic stack frame
                let x_ptr = args.next_arg::<*mut i32>();
                let y_ptr = args.next_arg::<*mut i32>();
                let z_ptr = args.next_arg::<*mut i32>();

                if x_ptr.is_null() || y_ptr.is_null() || z_ptr.is_null() {
                    error!("Error");
                    return 1;
                }

                let (x, y, z) = value
                    .unwrap_or_else(|| (or_default_int(0), or_default_int(1), or_default_int(2)));
                *x_ptr = x;
                *y_ptr = y;
                *z_ptr = z;
                return 0;
            }
        }
        ParameterValue::Double(x) => {
            unsafe {
                // Pull two separate pointers off the variadic stack frame
                let x_ptr = args.next_arg::<*mut f64>();

                if x_ptr.is_null() {
                    error!("Error");
                    return 1;
                }

                *x_ptr = x.unwrap_or_else(|| or_default_double(0));
                return 0;
            }
        }
        ParameterValue::Double2D(value) => {
            unsafe {
                // Pull two separate pointers off the variadic stack frame
                let x_ptr = args.next_arg::<*mut f64>();
                let y_ptr = args.next_arg::<*mut f64>();

                if x_ptr.is_null() || y_ptr.is_null() {
                    error!("Error");
                    return 1;
                }

                let (x, y) = value.unwrap_or_else(|| (or_default_double(0), or_default_double(1)));
                *x_ptr = x;
                *y_ptr = y;
                return 0;
            }
        }
        ParameterValue::Double3D(value) => {
            unsafe {
                // Pull two separate pointers off the variadic stack frame
                let x_ptr = args.next_arg::<*mut f64>();
                let y_ptr = args.next_arg::<*mut f64>();
                let z_ptr = args.next_arg::<*mut f64>();

                if x_ptr.is_null() || y_ptr.is_null() || z_ptr.is_null() {
                    error!("Error");
                    return 1;
                }

                let (x, y, z) = value.unwrap_or_else(|| {
                    (
                        or_default_double(0),
                        or_default_double(1),
                        or_default_double(2),
                    )
                });
                *x_ptr = x;
                *y_ptr = y;
                *z_ptr = z;
                return 0;
            }
        }
        ParameterValue::RGB(value) => {
            unsafe {
                // Pull four separate pointers off the variadic stack frame
                let r_ptr = args.next_arg::<*mut f64>();
                let g_ptr = args.next_arg::<*mut f64>();
                let b_ptr = args.next_arg::<*mut f64>();

                if r_ptr.is_null() || g_ptr.is_null() || b_ptr.is_null() {
                    error!("Error");
                    return 1;
                }

                // Write to each individual scalar pointer target
                let (r, g, b) = value.unwrap_or_else(|| {
                    (
                        or_default_double(0),
                        or_default_double(1),
                        or_default_double(2),
                    )
                });
                *r_ptr = r;
                *g_ptr = g;
                *b_ptr = b;

                return 0;
            }
        }
        ParameterValue::RGBA(value) => {
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
                let (r, g, b, a) = value.unwrap_or_else(|| {
                    (
                        or_default_double(0),
                        or_default_double(1),
                        or_default_double(2),
                        or_default_double(3),
                    )
                });
                *r_ptr = r;
                *g_ptr = g;
                *b_ptr = b;
                *a_ptr = a;

                return 0;
            }
        }
        ParameterValue::None => error!("Uhhh... uninmplemented?"),
    }

    kOfxStatOK
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
