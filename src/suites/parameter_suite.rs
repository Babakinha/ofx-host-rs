use crate::bindings::root::{self, OfxBytes};
use crate::bindings::root::{
    OfxParamHandle, OfxParamSetHandle, OfxPropertySetHandle, OfxRangeD, OfxStatus, OfxTime,
};
use crate::instance::{
    self, AsPropertySet, ParameterSet, ParameterThing, ParameterValue, PropertySet,
};
use crate::log_utils::c_str_to_str;
use crate::ofx_constants::{
    kOfxStatErrBadHandle, kOfxStatErrUnsupported, kOfxStatFailed, kOfxStatOK,
};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint};
use tracing::error;
use tracing::{instrument, trace, warn};

#[instrument(level = "trace", ret(level = "trace"), fields(name = c_str_to_str(name), param_type = c_str_to_str(param_type)))]
unsafe extern "C" fn param_define(
    param_set: OfxParamSetHandle,
    param_type: *const c_char,
    name: *const c_char,
    property_set: *mut OfxPropertySetHandle,
) -> OfxStatus {
    let instance = unsafe { ParameterSet::ref_mut_from_ofx_handle(param_set).unwrap() };

    let c_str = unsafe { CStr::from_ptr(param_type) };
    let param_type_str = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Error");
            return kOfxStatFailed;
        }
    };

    let c_str = unsafe { CStr::from_ptr(name) };
    let name_str = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Error");
            return kOfxStatFailed;
        }
    };

    instance.parameters.insert(
        name_str.clone(),
        crate::instance::ParameterThing {
            name: name_str.clone(),
            value: match param_type_str.as_str() {
                "OfxParamTypeBoolean" => instance::ParameterValue::Boolean(None),
                "OfxParamTypeInteger" => instance::ParameterValue::Integer(None),
                "OfxParamTypeInteger2D" => instance::ParameterValue::Integer2D(None),
                "OfxParamTypeInteger3D" => instance::ParameterValue::Integer3D(None),
                "OfxParamTypeDouble" => instance::ParameterValue::Double(None),
                "OfxParamTypeDouble2D" => instance::ParameterValue::Double2D(None),
                "OfxParamTypeDouble3D" => instance::ParameterValue::Double3D(None),
                "OfxParamTypeRGB" => instance::ParameterValue::RGB(None),
                "OfxParamTypeRGBA" => instance::ParameterValue::RGBA(None),
                "OfxParamTypeBytes" => instance::ParameterValue::Bytes(None),
                "OfxParamTypePage" => instance::ParameterValue::Page(None),
                "OfxParamTypeGroup" => instance::ParameterValue::Group(None),
                "OfxParamTypeChoice" => instance::ParameterValue::Choice(None),
                "OfxParamTypeStrChoice" => instance::ParameterValue::StrChoice(None),
                "OfxParamTypePushButton" => instance::ParameterValue::PushButton(None),
                "OfxParamTypeCustom" => instance::ParameterValue::Custom(None),

                param_type => {
                    error!("{param_type} not implemented yet!");
                    instance::ParameterValue::None
                }
            },
            properties: Box::new(PropertySet::new()),
        },
    );

    if !property_set.is_null() {
        unsafe {
            //TODO: Thecnically works since i put the Box<> but... its not using Box::into_raw or
            //smthn soooo... ehhh
            *property_set = instance
                .parameters
                .get_mut(&name_str)
                .unwrap()
                .get_properties_mut()
                .as_raw_ofx_handle();
        }
    }

    return kOfxStatOK;
}

#[instrument(level = "trace", ret(level = "trace"), fields(name = c_str_to_str(name)))]
unsafe extern "C" fn param_get_handle(
    param_set: OfxParamSetHandle,
    name: *const c_char,
    param: *mut OfxParamHandle,
    property_set: *mut OfxPropertySetHandle,
) -> OfxStatus {
    let instance = unsafe { ParameterSet::ref_mut_from_ofx_handle(param_set).unwrap() };

    let c_str = unsafe { CStr::from_ptr(name) };
    let name_str = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Error");
            return kOfxStatFailed;
        }
    };

    if let Some(parameter) = instance.parameters.get_mut(&name_str) {
        if !param.is_null() {
            unsafe {
                *param = parameter.as_raw_ofx_handle();
                trace!("{:#?}", *param)
            }
        }

        if !property_set.is_null() {
            unsafe {
                *property_set = parameter.get_properties_mut().as_raw_ofx_handle();
                trace!("{:#?}", *property_set)
            }
        }
    } else {
        error!("Error!");
        return kOfxStatFailed;
    }

    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_set_get_property_set(
    param_set: OfxParamSetHandle,
    prop_handle: *mut OfxPropertySetHandle,
) -> OfxStatus {
    if param_set.is_null() || prop_handle.is_null() {
        error!("Error!");
        return kOfxStatErrBadHandle;
    }
    let instance = unsafe { ParameterSet::ref_mut_from_ofx_handle(param_set).unwrap() };

    unsafe {
        *prop_handle = instance.get_properties_mut().as_raw_ofx_handle();
        trace!("*prop_handle: {:#?}", *prop_handle);
    }

    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_get_property_set(
    param: OfxParamHandle,
    prop_handle: *mut OfxPropertySetHandle,
) -> OfxStatus {
    let instance = unsafe { ParameterThing::ref_mut_from_ofx_handle(param).unwrap() };
    unsafe {
        *prop_handle = instance.get_properties_mut().as_raw_ofx_handle();
        trace!("*prop_handle: {:#?}", *prop_handle);
    }

    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_get_value(_param_handle: OfxParamHandle, _: ...) -> OfxStatus {
    error!("param_get_value not implemented");
    kOfxStatErrUnsupported
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
    let or_default_string = |index: usize| {
        instance
            .properties
            .strings
            .get("OfxParamPropDefault")
            .unwrap_or(&vec![])
            .get(index)
            .unwrap_or(&String::new())
            .clone()
    };
    match &instance.value {
        ParameterValue::Boolean(value) => unsafe {
            let x_ptr = args.next_arg::<*mut bool>();

            if x_ptr.is_null() {
                error!("Error");
                return kOfxStatFailed;
            }

            *x_ptr = value.unwrap_or_else(|| or_default_int(0) == 1);
        },
        ParameterValue::Integer(x) => unsafe {
            let x_ptr = args.next_arg::<*mut i32>();

            if x_ptr.is_null() {
                error!("Error");
                return kOfxStatFailed;
            }

            *x_ptr = x.unwrap_or_else(|| or_default_int(0));
        },
        ParameterValue::Integer2D(value) => unsafe {
            let x_ptr = args.next_arg::<*mut i32>();
            let y_ptr = args.next_arg::<*mut i32>();

            if x_ptr.is_null() || y_ptr.is_null() {
                error!("Error");
                return kOfxStatFailed;
            }

            let (x, y) = value.unwrap_or_else(|| (or_default_int(0), or_default_int(1)));
            *x_ptr = x;
            *y_ptr = y;
        },
        ParameterValue::Integer3D(value) => unsafe {
            let x_ptr = args.next_arg::<*mut i32>();
            let y_ptr = args.next_arg::<*mut i32>();
            let z_ptr = args.next_arg::<*mut i32>();

            if x_ptr.is_null() || y_ptr.is_null() || z_ptr.is_null() {
                error!("Error");
                return kOfxStatFailed;
            }

            let (x, y, z) =
                value.unwrap_or_else(|| (or_default_int(0), or_default_int(1), or_default_int(2)));
            *x_ptr = x;
            *y_ptr = y;
            *z_ptr = z;
        },
        ParameterValue::Double(x) => unsafe {
            let x_ptr = args.next_arg::<*mut f64>();

            if x_ptr.is_null() {
                error!("Error");
                return kOfxStatFailed;
            }

            *x_ptr = x.unwrap_or_else(|| or_default_double(0));
        },
        ParameterValue::Double2D(value) => unsafe {
            let x_ptr = args.next_arg::<*mut f64>();
            let y_ptr = args.next_arg::<*mut f64>();

            if x_ptr.is_null() || y_ptr.is_null() {
                error!("Error");
                return kOfxStatFailed;
            }

            let (mut x, mut y) =
                value.unwrap_or_else(|| (or_default_double(0), or_default_double(1)));

            let double_type = instance
                .properties
                .strings
                .get("OfxParamPropDoubleType")
                .and_then(|v| v.first())
                .map(|s| s.as_str())
                .unwrap_or("");

            let default_coord_system = instance
                .properties
                .strings
                .get("OfxParamPropDefaultCoordinateSystem")
                .and_then(|v| v.first())
                .map(|s| s.as_str())
                .unwrap_or("");

            if double_type == "OfxParamDoubleTypeXYAbsolute"
                && default_coord_system == "OfxParamCoordinatesNormalised"
            {
                // TODO: Grab projects or clip resolution
                let project_w = 720 as f64;
                let project_h = 480 as f64;

                x *= project_w;
                y *= project_h;
            }

            *x_ptr = x;
            *y_ptr = y;
        },

        ParameterValue::Double3D(value) => unsafe {
            let x_ptr = args.next_arg::<*mut f64>();
            let y_ptr = args.next_arg::<*mut f64>();
            let z_ptr = args.next_arg::<*mut f64>();

            if x_ptr.is_null() || y_ptr.is_null() || z_ptr.is_null() {
                error!("Error");
                return kOfxStatFailed;
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
        },
        ParameterValue::RGB(value) => unsafe {
            let r_ptr = args.next_arg::<*mut f64>();
            let g_ptr = args.next_arg::<*mut f64>();
            let b_ptr = args.next_arg::<*mut f64>();

            if r_ptr.is_null() || g_ptr.is_null() || b_ptr.is_null() {
                error!("Error");
                return kOfxStatFailed;
            }

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
        },
        ParameterValue::RGBA(value) => unsafe {
            let r_ptr = args.next_arg::<*mut f64>();
            let g_ptr = args.next_arg::<*mut f64>();
            let b_ptr = args.next_arg::<*mut f64>();
            let a_ptr = args.next_arg::<*mut f64>();

            if r_ptr.is_null() || g_ptr.is_null() || b_ptr.is_null() || a_ptr.is_null() {
                error!("Error");
                return kOfxStatFailed;
            }

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
        },

        ParameterValue::String(value) => unsafe {
            let x_ptr = args.next_arg::<*mut *mut c_char>();

            if x_ptr.is_null() {
                error!("Error");
                return kOfxStatFailed;
            }

            let str = match value.as_ref() {
                Some(string) => string.clone(),
                None => or_default_string(0),
            };
            let c_str = CString::new(str).unwrap();

            *x_ptr = c_str.into_raw();
        },

        ParameterValue::Bytes(value) => unsafe {
            let x_ptr = args.next_arg::<*mut OfxBytes>();

            if x_ptr.is_null() {
                error!("Error");
                return kOfxStatFailed;
            }

            // TODO: Default?
            let slice = value.as_ref().unwrap();
            *x_ptr = OfxBytes {
                data: slice.as_ptr(),
                length: slice.len(),
            };
        },

        ParameterValue::Choice(x) => unsafe {
            let x_ptr = args.next_arg::<*mut i32>();

            if x_ptr.is_null() {
                error!("Error");
                return kOfxStatFailed;
            }

            *x_ptr = x.unwrap_or_else(|| or_default_int(0));
        },
        ParameterValue::None => {
            error!("Uhhh... uninmplemented?");
            return kOfxStatFailed;
        }
        value => {
            error!("{value:?} Uhhh... uninmplemented?");
            return kOfxStatFailed;
        }
    }

    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_get_derivative(
    _param_handle: OfxParamHandle,
    _time: OfxTime,
    _: ...
) -> OfxStatus {
    error!("param_get_derivative not implemented");
    kOfxStatErrUnsupported
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_get_integral(
    _param_handle: OfxParamHandle,
    _time1: OfxTime,
    _time2: OfxTime,
    _: ...
) -> OfxStatus {
    error!("param_get_integral not implemented");
    kOfxStatErrUnsupported
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_set_value(_param_handle: OfxParamHandle, _: ...) -> OfxStatus {
    error!("param_set_value not implemented");
    kOfxStatErrUnsupported
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_set_value_at_time(
    _param_handle: OfxParamHandle,
    _time: OfxTime,
    _: ...
) -> OfxStatus {
    error!("param_set_value_at_time not implemented");
    kOfxStatErrUnsupported
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_get_num_keys(
    _param_handle: OfxParamHandle,
    _number_of_keys: *mut c_uint,
) -> OfxStatus {
    error!("param_get_num_keys not implemented");
    kOfxStatErrUnsupported
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_get_key_time(
    _param_handle: OfxParamHandle,
    _nth_key: c_uint,
    _time: *mut OfxTime,
) -> OfxStatus {
    error!("param_get_key_time not implemented");
    kOfxStatErrUnsupported
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_get_key_index(
    _param_handle: OfxParamHandle,
    _time: OfxTime,
    _direction: c_int,
    _index: *mut c_int,
) -> OfxStatus {
    error!("param_get_key_index not implemented");
    kOfxStatErrUnsupported
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_delete_key(_param_handle: OfxParamHandle, _time: OfxTime) -> OfxStatus {
    error!("param_delete_key not implemented");
    kOfxStatErrUnsupported
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_delete_all_keys(_param_handle: OfxParamHandle) -> OfxStatus {
    error!("param_delete_all_keys not implemented");
    kOfxStatErrUnsupported
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_copy(
    _param_to: OfxParamHandle,
    _param_from: OfxParamHandle,
    _dst_offset: OfxTime,
    _frame_range: *const OfxRangeD,
) -> OfxStatus {
    error!("param_copy not implemented");
    kOfxStatErrUnsupported
}

#[instrument(level = "trace", ret(level = "trace"), fields(name = c_str_to_str(_name)))]
unsafe extern "C" fn param_edit_begin(
    _param_set: OfxParamSetHandle,
    _name: *const c_char,
) -> OfxStatus {
    error!("param_edit_begin not implemented");
    kOfxStatErrUnsupported
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn param_edit_end(_param_set: OfxParamSetHandle) -> OfxStatus {
    error!("param_edit_end not implemented");
    kOfxStatErrUnsupported
}

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
