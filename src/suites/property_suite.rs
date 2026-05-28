use crate::{
    bindings::root::{OfxPropertySetHandle, OfxPropertySuiteV1, OfxStatus},
    instance::{self},
    log_utils::c_str_to_str,
    ofx_constants::{kOfxStatErrBadHandle, kOfxStatFailed, kOfxStatOK},
};
use std::ffi::{CStr, CString, c_char, c_int, c_void};
use tracing::{error};
use tracing::instrument;
use tracing::trace;
use tracing::warn;

#[instrument(level = "trace", ret(level = "trace"), fields(property = c_str_to_str(property)))]
unsafe extern "C" fn prop_set_pointer(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    index: c_int,
    value: *mut c_void,
) -> OfxStatus {
    if properties.is_null() || property.is_null() || index < 0 {
        error!("Error {:?} {:?} {:?}", property, properties, index);
        return kOfxStatErrBadHandle;
    }

    let instance = unsafe { instance::PropertySet::ref_mut_from_ofx_handle(properties).unwrap() };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Error");
            return kOfxStatFailed;
        }
    };

    let idx = index as usize;
    let entry = instance
        .pointers
        .entry(prop_key.clone())
        .or_insert_with(Vec::new);

    // Ensure the vector is large enough to accommodate the incoming index
    if idx + 1 > entry.len() {
        trace!("Resizing: {} to {}", entry.len(), idx + 1);
        entry.resize(idx + 1, std::ptr::null_mut());
    }
    entry[idx] = value;

    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"), fields(property = c_str_to_str(property), value = c_str_to_str(value)))]
unsafe extern "C" fn prop_set_string(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    index: c_int,
    value: *const c_char,
) -> OfxStatus {
    if properties.is_null() || property.is_null() || index < 0 {
        error!("Error");
        return kOfxStatErrBadHandle;
    }

    let instance = unsafe { instance::PropertySet::ref_mut_from_ofx_handle(properties).unwrap() };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Error");
            return kOfxStatFailed;
        }
    };

    let c_str = unsafe { CStr::from_ptr(value) };
    let prop_value = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Error");
            return kOfxStatFailed;
        }
    };

    let idx = index as usize;
    let entry = instance
        .strings
        .entry(prop_key.clone())
        .or_insert_with(Vec::new);

    // Ensure the vector is large enough to accommodate the incoming index
    if idx + 1 > entry.len() {
        trace!("Resizing: {} to {}", entry.len(), idx + 1);
        entry.resize(idx + 1, String::new());
    }
    entry[idx] = prop_value;

    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"), fields(property = c_str_to_str(property)))]
unsafe extern "C" fn prop_set_double(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    index: c_int,
    value: f64,
) -> OfxStatus {
    if properties.is_null() || property.is_null() || index < 0 {
        error!("Error");
        return kOfxStatErrBadHandle;
    }

    let instance = unsafe { instance::PropertySet::ref_mut_from_ofx_handle(properties).unwrap() };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Error");
            return kOfxStatFailed;
        }
    };

    let idx = index as usize;
    let entry = instance.doubles.entry(prop_key).or_insert_with(Vec::new);

    // Ensure the vector is large enough to accommodate the incoming index
    if idx + 1 > entry.len() {
        trace!("Resizing: {} to {}", entry.len(), idx + 1);
        entry.resize(idx + 1, 0_f64);
    }
    entry[idx] = value;

    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"), fields(property = c_str_to_str(property)))]
unsafe extern "C" fn prop_set_int(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    index: c_int,
    value: c_int,
) -> OfxStatus {
    if properties.is_null() || property.is_null() || index < 0 {
        error!("Error");
        return kOfxStatErrBadHandle;
    }

    let instance = unsafe { instance::PropertySet::ref_mut_from_ofx_handle(properties).unwrap() };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Error");
            return kOfxStatFailed;
        }
    };

    let idx = index as usize;
    let entry = instance.ints.entry(prop_key).or_insert_with(Vec::new);

    // Ensure the vector is large enough to accommodate the incoming index
    if idx + 1 > entry.len() {
        trace!("Resizing: {} to {}", entry.len(), idx + 1);
        entry.resize(idx + 1, 0);
    }
    entry[idx] = value;

    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"), fields(property = c_str_to_str(property)))]
unsafe extern "C" fn prop_set_pointer_n(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    count: c_int,
    value: *const *mut c_void,
) -> OfxStatus {
    if properties.is_null() || property.is_null() || count < 0 {
        error!("Error");
        return kOfxStatErrBadHandle;
    }

    let instance = unsafe { instance::PropertySet::ref_mut_from_ofx_handle(properties).unwrap() };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Error");
            return kOfxStatFailed;
        }
    };

    let entry = instance.pointers.entry(prop_key).or_insert_with(Vec::new);

    let incoming_values = unsafe { std::slice::from_raw_parts(value, count as usize) };

    // Ensure the vector is large enough to accommodate the incoming index
    if incoming_values.len() > entry.len() {
        trace!("Resizing: {} to {}", entry.len(), incoming_values.len());
        entry.resize(incoming_values.len(), std::ptr::null_mut());
    }

    entry[..incoming_values.len()].copy_from_slice(incoming_values);

    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"), fields(property = c_str_to_str(_property)))]
unsafe extern "C" fn prop_set_string_n(
    _properties: OfxPropertySetHandle,
    _property: *const c_char,
    _count: c_int,
    _value: *const *const c_char,
) -> OfxStatus {
    error!("propSetStringN not implemented");
    2
}

#[instrument(level = "trace", ret(level = "trace"), fields(property = c_str_to_str(property)))]
unsafe extern "C" fn prop_set_double_n(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    count: c_int,
    value: *const f64,
) -> OfxStatus {
    if properties.is_null() || property.is_null() || count < 0 {
        error!("Error");
        return kOfxStatErrBadHandle;
    }

    let instance = unsafe { instance::PropertySet::ref_mut_from_ofx_handle(properties).unwrap() };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Error");
            return kOfxStatFailed;
        }
    };

    let entry = instance.doubles.entry(prop_key).or_insert_with(Vec::new);

    let incoming_values = unsafe { std::slice::from_raw_parts(value, count as usize) };

    // Ensure the vector is large enough to accommodate the incoming index
    if incoming_values.len() > entry.len() {
        trace!("Resizing: {} to {}", entry.len(), incoming_values.len());
        entry.resize(incoming_values.len(), 0_f64);
    }

    entry[..incoming_values.len()].copy_from_slice(incoming_values);

    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"), fields(property = c_str_to_str(property)))]
unsafe extern "C" fn prop_set_int_n(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    count: c_int,
    value: *const c_int,
) -> OfxStatus {
    if properties.is_null() || property.is_null() || count < 0 {
        error!("Error");
        return kOfxStatErrBadHandle;
    }

    let instance = unsafe { instance::PropertySet::ref_mut_from_ofx_handle(properties).unwrap() };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Error");
            return kOfxStatFailed;
        }
    };

    let entry = instance.ints.entry(prop_key).or_insert_with(Vec::new);

    let incoming_values = unsafe { std::slice::from_raw_parts(value, count as usize) };

    // Ensure the vector is large enough to accommodate the incoming index
    if incoming_values.len() > entry.len() {
        trace!("Resizing: {} to {}", entry.len(), incoming_values.len());
        entry.resize(incoming_values.len(), 0);
    }

    entry[..incoming_values.len()].copy_from_slice(incoming_values);

    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"), fields(property = c_str_to_str(property)))]
unsafe extern "C" fn prop_get_pointer(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    index: c_int,
    value: *mut *mut c_void,
) -> OfxStatus {
    if properties.is_null() || property.is_null() || index < 0 {
        error!("Error");
        return kOfxStatErrBadHandle;
    }

    let instance = unsafe { instance::PropertySet::ref_mut_from_ofx_handle(properties).unwrap() };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Error");
            return kOfxStatFailed;
        }
    };

    let idx = index as usize;
    let entry = instance
        .pointers
        .entry(prop_key.clone())
        .or_insert_with(Vec::new);

    let entry_value = entry.get_mut(idx);
    match entry_value {
        Some(pointer) => unsafe {
            *value = pointer.clone();
        },
        None => {
            error!("Error");
            return kOfxStatFailed;
        }
    }

    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"), fields(property = c_str_to_str(property)))]
unsafe extern "C" fn prop_get_string(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    index: c_int,
    value: *mut *mut c_char,
) -> OfxStatus {
    if properties.is_null() || property.is_null() || index < 0 {
        error!("Error");
        return kOfxStatErrBadHandle;
    }

    let instance = unsafe { instance::PropertySet::ref_mut_from_ofx_handle(properties).unwrap() };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Error");
            return kOfxStatFailed;
        }
    };

    let idx = index as usize;
    let entry = instance
        .strings
        .entry(prop_key.clone())
        .or_insert_with(Vec::new);

    let entry_value = entry.get_mut(idx);
    match entry_value {
        Some(string) => unsafe {
            let c_string = CString::new(string.as_bytes()).unwrap().into_raw();
            *value = c_string as *mut i8;
        },
        None => {
            error!("Error");
            return kOfxStatFailed;
        }
    }

    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"), fields(property = c_str_to_str(property)))]
unsafe extern "C" fn prop_get_double(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    index: c_int,
    value: *mut f64,
) -> OfxStatus {
    if properties.is_null() || property.is_null() || index < 0 {
        error!("Error");
        return kOfxStatErrBadHandle;
    }

    let instance = unsafe { instance::PropertySet::ref_mut_from_ofx_handle(properties).unwrap() };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Error");
            return kOfxStatFailed;
        }
    };

    let idx = index as usize;
    let entry = instance
        .doubles
        .entry(prop_key.clone())
        .or_insert_with(Vec::new);

    let entry_value = entry.get_mut(idx);
    match entry_value {
        Some(double) => unsafe {
            *value = double.clone();
        },
        None => {
            error!("Error");
            return kOfxStatFailed;
        }
    }

    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"), fields(property = c_str_to_str(property)))]
unsafe extern "C" fn prop_get_int(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    index: c_int,
    value: *mut c_int,
) -> OfxStatus {
    if properties.is_null() || property.is_null() || index < 0 {
        error!("Error");
        return kOfxStatErrBadHandle;
    }

    let instance = unsafe { instance::PropertySet::ref_mut_from_ofx_handle(properties).unwrap() };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Error");
            return kOfxStatFailed;
        }
    };

    let idx = index as usize;
    let entry = instance
        .ints
        .entry(prop_key.clone())
        .or_insert_with(Vec::new);

    let entry_value = entry.get_mut(idx);
    match entry_value {
        Some(int) => unsafe {
            *value = int.clone();
        },
        None => {
            error!("Error");
            return kOfxStatFailed;
        }
    }

    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"), fields(property = c_str_to_str(property)))]
unsafe extern "C" fn prop_get_pointer_n(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    count: c_int,
    value: *mut *mut c_void,
) -> OfxStatus {
    if properties.is_null() || property.is_null() {
        error!("Error");
        return kOfxStatErrBadHandle;
    }

    let instance = unsafe { instance::PropertySet::ref_mut_from_ofx_handle(properties).unwrap() };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Error");
            return kOfxStatFailed;
        }
    };

    let entry = instance
        .pointers
        .entry(prop_key.clone())
        .or_insert_with(Vec::new);

    if count as usize > entry.len() {
        warn!("Resizing: {} to {}", entry.len(), count);
        entry.resize(count as usize, std::ptr::null_mut());
    }

    let outgoing_array = unsafe { std::slice::from_raw_parts_mut(value, count as usize) };
    outgoing_array.copy_from_slice(&entry[..count as usize]);

    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"), fields(property = c_str_to_str(_property)))]
unsafe extern "C" fn prop_get_string_n(
    _properties: OfxPropertySetHandle,
    _property: *const c_char,
    _count: c_int,
    _value: *mut *mut c_char,
) -> OfxStatus {
    error!("propGetStringN not implemented");
    2
}

#[instrument(level = "trace", ret(level = "trace"), fields(property = c_str_to_str(property)))]
unsafe extern "C" fn prop_get_double_n(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    count: c_int,
    value: *mut f64,
) -> OfxStatus {
    if properties.is_null() || property.is_null() {
        error!("Error");
        return kOfxStatErrBadHandle;
    }

    let instance = unsafe { instance::PropertySet::ref_mut_from_ofx_handle(properties).unwrap() };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Error");
            return kOfxStatFailed;
        }
    };

    let entry = instance
        .doubles
        .entry(prop_key.clone())
        .or_insert_with(Vec::new);

    if count as usize > entry.len() {
        warn!("Resizing: {} to {}", entry.len(), count);
        entry.resize(count as usize, 0_f64);
    }

    let outgoing_array = unsafe { std::slice::from_raw_parts_mut(value, count as usize) };
    outgoing_array.copy_from_slice(&entry[..count as usize]);

    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"), fields(property = c_str_to_str(property)))]
unsafe extern "C" fn prop_get_int_n(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    count: c_int,
    value: *mut c_int,
) -> OfxStatus {
    if properties.is_null() || property.is_null() {
        error!("Error");
        return kOfxStatErrBadHandle;
    }

    let instance = unsafe { instance::PropertySet::ref_mut_from_ofx_handle(properties).unwrap() };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Error");
            return kOfxStatFailed;
        }
    };

    let entry = instance
        .ints
        .entry(prop_key.clone())
        .or_insert_with(Vec::new);

    if count as usize > entry.len() {
        warn!("Resizing: {} to {}", entry.len(), count);
        entry.resize(count as usize, 0);
    }

    let outgoing_array = unsafe { std::slice::from_raw_parts_mut(value, count as usize) };
    outgoing_array.copy_from_slice(&entry[..count as usize]);

    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"), fields(property = c_str_to_str(_property)))]
unsafe extern "C" fn prop_reset(
    _properties: OfxPropertySetHandle,
    _property: *const c_char,
) -> OfxStatus {
    error!("propReset not implemented");
    2
}

#[instrument(level = "trace", ret(level = "trace"), fields(property = c_str_to_str(_property)))]
unsafe extern "C" fn prop_get_dimension(
    _properties: OfxPropertySetHandle,
    _property: *const c_char,
    _count: *mut c_int,
) -> OfxStatus {
    error!("propGetDimension not implemented");
    2
}

#[instrument(level = "trace", ret(level = "trace"))]
pub fn property_suite() -> OfxPropertySuiteV1 {
    OfxPropertySuiteV1 {
        propSetPointer: Some(prop_set_pointer),
        propSetString: Some(prop_set_string),
        propSetDouble: Some(prop_set_double),
        propSetInt: Some(prop_set_int),
        propSetPointerN: Some(prop_set_pointer_n),
        propSetStringN: Some(prop_set_string_n),
        propSetDoubleN: Some(prop_set_double_n),
        propSetIntN: Some(prop_set_int_n),
        propGetPointer: Some(prop_get_pointer),
        propGetString: Some(prop_get_string),
        propGetDouble: Some(prop_get_double),
        propGetInt: Some(prop_get_int),
        propGetPointerN: Some(prop_get_pointer_n),
        propGetStringN: Some(prop_get_string_n),
        propGetDoubleN: Some(prop_get_double_n),
        propGetIntN: Some(prop_get_int_n),
        propReset: Some(prop_reset),
        propGetDimension: Some(prop_get_dimension),
    }
}
