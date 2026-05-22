use crate::{
    bindings::root::{OfxPropertySetHandle, OfxPropertySuiteV1, OfxStatus, kOfxStatOK},
    instance::{OfxHandle, PropertySet},
};
use std::ffi::{CStr, CString, c_char, c_int, c_void};

unsafe extern "C" fn prop_set_pointer(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    index: c_int,
    value: *mut c_void,
) -> OfxStatus {
    dbg!("prop_set_pointer");
    if properties.is_null() || property.is_null() || index < 0 {
            dbg!("Error", property, properties, index);
        return 4; // kOfxStatErrBadHandle / kOfxStatErrBadIndex
    }

    let instance_ptr = properties as *mut OfxHandle;
    let instance = unsafe { &mut *instance_ptr };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            dbg!("Error");
            return 1;
        } // kOfxStatFailed
    };

    let idx = index as usize;
    dbg!(&instance, &idx, &prop_key, value);
    let entry = instance
        .get_propeties_mut()
        .pointers
        .entry(prop_key.clone())
        .or_insert_with(Vec::new);

    // Ensure the vector is large enough to accommodate the incoming index
    if idx >= entry.len() {
        entry.resize(idx + 1, std::ptr::null_mut());
    }
    entry[idx] = value;

    kOfxStatOK as i32
}

unsafe extern "C" fn prop_set_string(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    index: c_int,
    value: *const c_char,
) -> OfxStatus {
    dbg!("prop_set_string");
    if properties.is_null() || property.is_null() || index < 0 {
            dbg!("Error", property, properties, index);
        return 4; // kOfxStatErrBadHandle / kOfxStatErrBadIndex
    }

    let instance_ptr = properties as *mut OfxHandle;
    let instance = unsafe { &mut *instance_ptr };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            dbg!("Error");
            return 1;
        }
    };

    let c_str = unsafe { CStr::from_ptr(value) };
    let prop_value = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            dbg!("Error");
            return 1;
        }
    };

    dbg!(&prop_key, &index);
    let idx = index as usize;
    let entry = instance
        .get_propeties_mut()
        .strings
        .entry(prop_key.clone())
        .or_insert_with(Vec::new);

    // Ensure the vector is large enough to accommodate the incoming index
    if idx >= entry.len() {
        entry.resize(idx + 1, String::new());
    }
    entry[idx] = prop_value;

    kOfxStatOK as i32
}

unsafe extern "C" fn prop_set_double(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    index: c_int,
    value: f64,
) -> OfxStatus {
    dbg!("prop_set_double");
    if properties.is_null() || property.is_null() || index < 0 {
            dbg!("Error", property, properties, index);
        return 4; // kOfxStatErrBadHandle / kOfxStatErrBadIndex
    }

    let instance_ptr = properties as *mut OfxHandle;
    let instance = unsafe { &mut *instance_ptr };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            dbg!("Error");
            return 1; // kOfxStatFailed
        }
    };

    dbg!(&prop_key, &index);
    let idx = index as usize;
    let entry = instance
        .get_propeties_mut()
        .doubles
        .entry(prop_key)
        .or_insert_with(Vec::new);

    // Ensure the vector is large enough to accommodate the incoming index
    if idx >= entry.len() {
        entry.resize(idx + 1, 0_f64);
    }
    entry[idx] = value;

    kOfxStatOK as i32
}

unsafe extern "C" fn prop_set_int(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    index: c_int,
    value: c_int,
) -> OfxStatus {
    dbg!("prop_set_int");
    if properties.is_null() || property.is_null() || index < 0 {
            dbg!("Error", property, properties, index);
        return 4; // kOfxStatErrBadHandle / kOfxStatErrBadIndex
    }

    let instance_ptr = properties as *mut OfxHandle;
    let instance = unsafe { &mut *instance_ptr };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            dbg!("Error");
            return 1; // kOfxStatFailed
        }
    };

    dbg!(&prop_key, &index);
    let idx = index as usize;
    let entry = instance
        .get_propeties_mut()
        .ints
        .entry(prop_key)
        .or_insert_with(Vec::new);

    // Ensure the vector is large enough to accommodate the incoming index
    if idx >= entry.len() {
        entry.resize(idx + 1, 0);
    }
    entry[idx] = value;

    kOfxStatOK as i32
}

unsafe extern "C" fn prop_set_pointer_n(
    _properties: OfxPropertySetHandle,
    _property: *const c_char,
    _count: c_int,
    _value: *const *mut c_void,
) -> OfxStatus {
    dbg!("prop_set_pointer_n");
    eprintln!("propSetPointerN not implemented");
    2
}

unsafe extern "C" fn prop_set_string_n(
    _properties: OfxPropertySetHandle,
    _property: *const c_char,
    _count: c_int,
    _value: *const *const c_char,
) -> OfxStatus {
    dbg!("prop_set_string_n");
    eprintln!("propSetStringN not implemented");
    2
}

unsafe extern "C" fn prop_set_double_n(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    count: c_int,
    value: *const f64,
) -> OfxStatus {
    dbg!("prop_set_double_n");
    if properties.is_null() || property.is_null() || count < 0 {
            dbg!("Error", property, properties, count);
        return 4; // kOfxStatErrBadHandle / kOfxStatErrBadIndex
    }

    let instance_ptr = properties as *mut OfxHandle;
    let instance = unsafe { &mut *instance_ptr };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            dbg!("Error");
            return 1; // kOfxStatFailed
        }
    };

    dbg!(&instance, &prop_key);
    let entry = instance
        .get_propeties_mut()
        .doubles
        .entry(prop_key)
        .or_insert_with(Vec::new);

    let incoming_values = unsafe { std::slice::from_raw_parts(value, count as usize) };

    // Ensure the vector is large enough to accommodate the incoming index
    if entry.len() < incoming_values.len() {
        entry.resize(incoming_values.len(), 0_f64);
    }

    entry[..incoming_values.len()].copy_from_slice(incoming_values);

    kOfxStatOK as i32
}

unsafe extern "C" fn prop_set_int_n(
    _properties: OfxPropertySetHandle,
    _property: *const c_char,
    _count: c_int,
    _value: *const c_int,
) -> OfxStatus {
    dbg!("prop_set_int_n");
    eprintln!("propSetIntN not implemented");
    2
}

unsafe extern "C" fn prop_get_pointer(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    index: c_int,
    value: *mut *mut c_void,
) -> OfxStatus {
    dbg!("prop_get_pointer");
    if properties.is_null() || property.is_null() || index < 0 {
            dbg!("Error", property, properties, index);
        return 4; // kOfxStatErrBadHandle / kOfxStatErrBadIndex
    }

    let instance_ptr = properties as *mut OfxHandle;
    let instance = unsafe { &mut *instance_ptr };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            dbg!("Error");
            return 1; // kOfxStatFailed
        }
    };

    dbg!(&prop_key, &index);
    let idx = index as usize;
    let entry = instance
        .get_propeties_mut()
        .pointers
        .entry(prop_key.clone())
        .or_insert_with(Vec::new);

    let entry_value = entry.get_mut(idx);
    match entry_value {
        Some(pointer) => unsafe {
            *value = pointer.clone();
        },
        None => {
            dbg!(instance, index, prop_key);
            dbg!("Error");
            return 1; // kOfxStatFailed
        }
    }

    kOfxStatOK as i32
}

unsafe extern "C" fn prop_get_string(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    index: c_int,
    value: *mut *mut c_char,
) -> OfxStatus {
    dbg!("prop_get_string");
    if properties.is_null() || property.is_null() || index < 0 {
            dbg!("Error", property, properties, index);
        return 4; // kOfxStatErrBadHandle / kOfxStatErrBadIndex
    }

    let instance_ptr = properties as *mut OfxHandle;
    let instance = unsafe { &mut *instance_ptr };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            dbg!("Error");
            return 1; // kOfxStatFailed
        }
    };
    dbg!(&instance, &prop_key, &index);

    dbg!(&prop_key, &index);
    let idx = index as usize;
    let entry = instance
        .get_propeties_mut()
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
            dbg!("Error");
            return 1; // kOfxStatFailed
        }
    }

    kOfxStatOK as i32
}

unsafe extern "C" fn prop_get_double(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    index: c_int,
    value: *mut f64,
) -> OfxStatus {
    dbg!("prop_get_double");
    if properties.is_null() || property.is_null() || index < 0 {
            dbg!("Error", property, properties, index);
        return 4; // kOfxStatErrBadHandle / kOfxStatErrBadIndex
    }

    let instance_ptr = properties as *mut OfxHandle;
    let instance = unsafe { &mut *instance_ptr };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            dbg!("Error");
            return 1; // kOfxStatFailed
        }
    };

    dbg!(&instance, index, &prop_key);
    let idx = index as usize;
    let entry = instance
        .get_propeties_mut()
        .doubles
        .entry(prop_key.clone())
        .or_insert_with(Vec::new);

    let entry_value = entry.get_mut(idx);
    match entry_value {
        Some(double) => unsafe {
            *value = double.clone();
        },
        None => {
            dbg!("Error");
            return 1; // kOfxStatFailed
        }
    }

    kOfxStatOK as i32
}

unsafe extern "C" fn prop_get_int(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    index: c_int,
    value: *mut c_int,
) -> OfxStatus {
    dbg!("prop_get_int");
    if properties.is_null() || property.is_null() || index < 0 {
            dbg!("Error", property, properties, index);
        return 4; // kOfxStatErrBadHandle / kOfxStatErrBadIndex
    }

    let instance_ptr = properties as *mut OfxHandle;
    let instance = unsafe { &mut *instance_ptr };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            dbg!("Error");
            return 1; // kOfxStatFailed
        }
    };

    dbg!(&instance, index, &prop_key);
    let idx = index as usize;
    let entry = instance
        .get_propeties_mut()
        .ints
        .entry(prop_key.clone())
        .or_insert_with(Vec::new);

    let entry_value = entry.get_mut(idx);
    match entry_value {
        Some(int) => unsafe {
            *value = int.clone();
        },
        None => {
            dbg!("Error");
            return 1; // kOfxStatFailed
        }
    }

    kOfxStatOK as i32
}

unsafe extern "C" fn prop_get_pointer_n(
    _properties: OfxPropertySetHandle,
    _property: *const c_char,
    _count: c_int,
    _value: *mut *mut c_void,
) -> OfxStatus {
    dbg!("prop_get_pointer_n");
    eprintln!("propGetPointerN not implemented");
    2
}

unsafe extern "C" fn prop_get_string_n(
    _properties: OfxPropertySetHandle,
    _property: *const c_char,
    _count: c_int,
    _value: *mut *mut c_char,
) -> OfxStatus {
    dbg!("prop_get_string_n");
    eprintln!("propGetStringN not implemented");
    2
}

unsafe extern "C" fn prop_get_double_n(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    count: c_int,
    value: *mut f64,
) -> OfxStatus {
    dbg!("prop_get_double_n");
    if properties.is_null() || property.is_null() {
            dbg!("Error", property, properties, count);
        return 4; // kOfxStatErrBadHandle / kOfxStatErrBadIndex
    }

    let instance_ptr = properties as *mut OfxHandle;
    let instance = unsafe { &mut *instance_ptr };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            dbg!("Error");
            return 1; // kOfxStatFailed
        }
    };

    dbg!(&instance, &prop_key);
    let entry = instance
        .get_propeties_mut()
        .doubles
        .entry(prop_key.clone())
        .or_insert_with(Vec::new);

    if count as usize >= entry.len() {
        entry.resize(count as usize, 0_f64);
    }

    let outgoing_array = unsafe { std::slice::from_raw_parts_mut(value, count as usize) };
    outgoing_array.copy_from_slice(&entry[..count as usize]);

    kOfxStatOK as i32
}

unsafe extern "C" fn prop_get_int_n(
    properties: OfxPropertySetHandle,
    property: *const c_char,
    count: c_int,
    value: *mut c_int,
) -> OfxStatus {
    dbg!("prop_get_int_n");
    if properties.is_null() || property.is_null() {
            dbg!("Error", property, properties, count);
        return 4; // kOfxStatErrBadHandle / kOfxStatErrBadIndex
    }

    let instance_ptr = properties as *mut OfxHandle;
    let instance = unsafe { &mut *instance_ptr };

    let c_str = unsafe { CStr::from_ptr(property) };
    let prop_key = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            dbg!("Error");
            return 1; // kOfxStatFailed
        }
    };

    dbg!(&instance, &prop_key);
    let entry = instance
        .get_propeties_mut()
        .ints
        .entry(prop_key.clone())
        .or_insert_with(Vec::new);

    if count as usize >= entry.len() {
        entry.resize(count as usize, 0);
    }

    let outgoing_array = unsafe { std::slice::from_raw_parts_mut(value, count as usize) };
    outgoing_array.copy_from_slice(&entry[..count as usize]);

    kOfxStatOK as i32
}

unsafe extern "C" fn prop_reset(
    _properties: OfxPropertySetHandle,
    _property: *const c_char,
) -> OfxStatus {
    dbg!("prop_reset");
    eprintln!("propReset not implemented");
    2
}

unsafe extern "C" fn prop_get_dimension(
    _properties: OfxPropertySetHandle,
    _property: *const c_char,
    _count: *mut c_int,
) -> OfxStatus {
    dbg!("prop_get_dimension");
    eprintln!("propGetDimension not implemented");
    2
}

pub fn property_suite() -> OfxPropertySuiteV1 {
    dbg!("property_suite");
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
