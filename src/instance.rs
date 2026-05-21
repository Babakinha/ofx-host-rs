use std::{collections::HashMap, ffi::{c_int, c_void}};

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
}

impl Default for PropertySet {
    fn default() -> Self {
        Self::new()
    }
}

pub trait AsPropertySet {
    fn get_properties(&self) -> &PropertySet;
    fn get_properties_mut(&mut self) -> &mut PropertySet;
}

#[derive(Debug)]
pub struct ParameterThing {
    pub name: String,
    pub param_type: String,
    pub properties: PropertySet,
}

impl ParameterThing {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            param_type: String::new(),
            properties: PropertySet::new(),
        }
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

#[derive(Debug)]
pub struct ClipThing {
    pub name: String,
    pub properties: PropertySet,
}

impl ClipThing {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            properties: PropertySet::new(),
        }
    }
}

impl Default for ClipThing {
    fn default() -> Self {
        Self::new()
    }
}

impl AsPropertySet for ClipThing {
    fn get_properties(&self) -> &PropertySet {
        &self.properties
    }
    fn get_properties_mut(&mut self) -> &mut PropertySet {
        &mut self.properties
    }
}
#[derive(Debug)]
pub struct StandalonePropertySet {
    pub properties: PropertySet,
}

impl StandalonePropertySet {
    pub fn new() -> Self {
        Self {
            properties: PropertySet::new(),
        }
    }
}

impl AsPropertySet for StandalonePropertySet {
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
    pub parameters: HashMap<String, Box<OfxHandle>>,
    pub clips: HashMap<String, Box<OfxHandle>>,
}

impl BabafxInstance {
    pub fn new() -> Self {
        Self {
            properties: PropertySet::new(),
            parameters: HashMap::new(),
            clips: HashMap::new(),
        }
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

#[derive(Debug)]
pub enum OfxHandleTarget {
    StandalonePropertySet(StandalonePropertySet),
    ParameterThing(ParameterThing),
    ClipThing(ClipThing),
    BabaFx(BabafxInstance),
}

#[derive(Debug)]
pub struct OfxHandle {
    pub target: OfxHandleTarget,
}

impl OfxHandle {
    pub fn get_propeties_mut(&mut self) -> &mut PropertySet {
        match &mut self.target {
            OfxHandleTarget::StandalonePropertySet(property_set) => {
                property_set.get_properties_mut()
            }
            OfxHandleTarget::BabaFx(babafx_instance) => babafx_instance.get_properties_mut(),
            OfxHandleTarget::ParameterThing(parameter_thing) => parameter_thing.get_properties_mut(),
            OfxHandleTarget::ClipThing(clip_thing) => clip_thing.get_properties_mut(),
        }
    }
}
