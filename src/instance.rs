use std::{
    collections::HashMap,
    ffi::{c_int, c_void},
};

use crate::bindings::root::{
    OfxImageClipHandle, OfxImageEffectHandle, OfxParamHandle, OfxParamSetHandle,
    OfxPropertySetHandle,
};

#[derive(Debug)]
pub struct PropertySet {
    pub strings: HashMap<String, Vec<String>>,
    pub ints: HashMap<String, Vec<c_int>>,
    pub pointers: HashMap<String, Vec<*mut c_void>>,
    pub doubles: HashMap<String, Vec<f64>>,
}

impl PropertySet {
    pub fn new() -> Self {
        Self {
            strings: HashMap::new(),
            ints: HashMap::new(),
            doubles: HashMap::new(),
            pointers: HashMap::new(),
        }
    }

    pub unsafe fn from_ofx_handle<'a>(ptr: OfxPropertySetHandle) -> Box<Self> {
        unsafe { Box::from_raw(ptr as *mut Self) }
    }

    pub unsafe fn ref_mut_from_ofx_handle<'a>(ptr: OfxPropertySetHandle) -> Option<&'a mut Self> {
        unsafe { (ptr as *mut Self).as_mut() }
    }

    pub fn to_raw_ofx_handle(self: Self) -> OfxPropertySetHandle {
        Box::into_raw(Box::new(self)) as OfxPropertySetHandle
    }

    pub unsafe fn as_raw_ofx_handle(&mut self) -> OfxPropertySetHandle {
        self as *mut Self as OfxPropertySetHandle
    }
}

impl Default for PropertySet {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
pub trait AsPropertySet {
    fn get_properties(&self) -> &PropertySet;
    fn get_properties_mut(&mut self) -> &mut PropertySet;
}

// https://openfx.readthedocs.io/en/latest/Reference/api/file/ofxParam_8h.html
#[allow(dead_code)]
#[derive(Debug)]
pub enum ParameterValue {
    Boolean(Option<bool>),
    Integer(Option<i32>),
    Integer2D(Option<(i32, i32)>),
    Integer3D(Option<(i32, i32, i32)>),

    Double(Option<f64>),
    Double2D(Option<(f64, f64)>),
    Double3D(Option<(f64, f64, f64)>),

    RGB(Option<(f64, f64, f64)>),
    RGBA(Option<(f64, f64, f64, f64)>),

    String(Option<String>),

    Bytes(Option<Vec<u8>>),

    Page(Option<()>),
    Group(Option<()>),

    Choice(Option<i32>),
    StrChoice(Option<()>),

    PushButton(Option<()>),

    Custom(Option<()>),

    None,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ParameterThing {
    pub name: String,
    pub value: ParameterValue,
    pub properties: Box<PropertySet>,
}

#[allow(dead_code)]
impl ParameterThing {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            value: ParameterValue::None,
            properties: Box::new(PropertySet::new()),
        }
    }

    pub unsafe fn from_ofx_handle<'a>(ptr: OfxParamHandle) -> Box<Self> {
        unsafe { Box::from_raw(ptr as *mut Self) }
    }

    pub unsafe fn ref_mut_from_ofx_handle<'a>(ptr: OfxParamHandle) -> Option<&'a mut Self> {
        unsafe { (ptr as *mut Self).as_mut() }
    }

    pub fn to_raw_ofx_handle(self: Self) -> OfxParamHandle {
        Box::into_raw(Box::new(self)) as OfxParamHandle
    }

    pub unsafe fn as_raw_ofx_handle(&mut self) -> OfxParamHandle {
        self as *mut Self as OfxParamHandle
    }
}

impl Default for ParameterThing {
    fn default() -> Self {
        Self::new()
    }
}

impl AsPropertySet for ParameterThing {
    fn get_properties(&self) -> &PropertySet {
        &self.properties
    }
    fn get_properties_mut(&mut self) -> &mut PropertySet {
        &mut self.properties
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ImageClip {
    pub name: String,
    pub properties: PropertySet,
}

#[allow(dead_code)]
impl ImageClip {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            properties: PropertySet::new(),
        }
    }

    pub unsafe fn from_ofx_handle<'a>(ptr: OfxImageClipHandle) -> Box<Self> {
        unsafe { Box::from_raw(ptr as *mut Self) }
    }

    pub unsafe fn ref_mut_from_ofx_handle<'a>(ptr: OfxImageClipHandle) -> Option<&'a mut Self> {
        unsafe { (ptr as *mut Self).as_mut() }
    }

    pub fn to_raw_ofx_handle(self: Self) -> OfxImageClipHandle {
        Box::into_raw(Box::new(self)) as OfxImageClipHandle
    }

    pub unsafe fn as_raw_ofx_handle(&mut self) -> OfxImageClipHandle {
        self as *mut Self as OfxImageClipHandle
    }
}

impl Default for ImageClip {
    fn default() -> Self {
        Self::new()
    }
}

impl AsPropertySet for ImageClip {
    fn get_properties(&self) -> &PropertySet {
        &self.properties
    }
    fn get_properties_mut(&mut self) -> &mut PropertySet {
        &mut self.properties
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ParameterSet {
    pub parameters: HashMap<String, ParameterThing>,
    pub properties: PropertySet,
}
#[allow(dead_code)]
impl ParameterSet {
    pub fn new() -> Self {
        Self {
            parameters: HashMap::new(),
            properties: PropertySet::new(),
        }
    }

    pub unsafe fn from_ofx_handle<'a>(ptr: OfxParamSetHandle) -> Box<Self> {
        unsafe { Box::from_raw(ptr as *mut Self) }
    }

    pub unsafe fn ref_mut_from_ofx_handle<'a>(ptr: OfxParamSetHandle) -> Option<&'a mut Self> {
        unsafe { (ptr as *mut Self).as_mut() }
    }

    pub fn to_raw_ofx_handle(self: Self) -> OfxParamSetHandle {
        Box::into_raw(Box::new(self)) as OfxParamSetHandle
    }

    pub unsafe fn as_raw_ofx_handle(&mut self) -> OfxParamSetHandle {
        self as *mut Self as OfxParamSetHandle
    }
}

impl Default for ParameterSet {
    fn default() -> Self {
        Self::new()
    }
}

impl AsPropertySet for ParameterSet {
    fn get_properties(&self) -> &PropertySet {
        &self.properties
    }
    fn get_properties_mut(&mut self) -> &mut PropertySet {
        &mut self.properties
    }
}

#[derive(Debug)]
pub struct BabafxInstance {
    pub properties: PropertySet,
    pub parameters: ParameterSet,
    pub clips: HashMap<String, ImageClip>,
}

#[allow(dead_code)]
impl BabafxInstance {
    pub fn new() -> Self {
        Self {
            properties: PropertySet::new(),
            parameters: ParameterSet::new(),
            clips: HashMap::new(),
        }
    }

    pub unsafe fn from_ofx_handle<'a>(ptr: OfxImageEffectHandle) -> Box<Self> {
        unsafe { Box::from_raw(ptr as *mut Self) }
    }

    pub unsafe fn ref_mut_from_ofx_handle<'a>(ptr: OfxImageEffectHandle) -> Option<&'a mut Self> {
        unsafe { (ptr as *mut Self).as_mut() }
    }

    pub fn to_raw_ofx_handle(self: Self) -> OfxImageEffectHandle {
        Box::into_raw(Box::new(self)) as OfxImageEffectHandle
    }

    pub unsafe fn as_raw_ofx_handle(&mut self) -> OfxImageEffectHandle {
        self as *mut Self as OfxImageEffectHandle
    }
}

impl AsPropertySet for BabafxInstance {
    fn get_properties(&self) -> &PropertySet {
        &self.properties
    }
    fn get_properties_mut(&mut self) -> &mut PropertySet {
        &mut self.properties
    }
}
