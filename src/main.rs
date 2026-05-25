#![feature(c_variadic)]
use libloading::Symbol;
use tracing::{debug, error, instrument, trace};

pub mod bindings {
    #![allow(unsafe_op_in_unsafe_fn)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod instance;
mod log_utils;
mod suites;

use std::ffi::{CStr, c_char, c_void};

use crate::{
    bindings::root::OfxPropertySetStruct,
    instance::{AsPropertySet, BabafxInstance, OfxHandle},
    log_utils::c_str_to_str,
    suites::{
        image_effect_suite::image_effect_suite, interact_suite::interact_suite,
        memory_suite::memory_suite, message_suite::message_suite,
        multithread_suite::multithread_suite, parameter_suite::parameter_suite,
        property_suite::property_suite,
    },
};

#[instrument(level = "trace", ret(level = "trace"), fields(suite_name = c_str_to_str(suite_name)))]
unsafe extern "C" fn host_fetch_suite(
    _host: *mut bindings::root::OfxPropertySetStruct,
    suite_name: *const c_char,
    suite_version: i32,
) -> *const c_void {
    if suite_name.is_null() {
        return std::ptr::null();
    }

    let name = unsafe { CStr::from_ptr(suite_name).to_str().unwrap_or("") };
    match (name, suite_version) {
        ("OfxImageEffectSuite", 1) => {
            let suite = image_effect_suite();
            let boxed = Box::new(suite);
            let leaked_ref = Box::leak(boxed);
            let suite_ptr = leaked_ref as *const _ as *const c_void;
            suite_ptr
        }
        ("OfxPropertySuite", 1) => {
            let suite = property_suite();
            let boxed = Box::new(suite);
            let leaked_ref = Box::leak(boxed);
            let suite_ptr = leaked_ref as *const _ as *const c_void;
            suite_ptr
        }
        ("OfxParameterSuite", 1) => {
            let suite = parameter_suite();
            let boxed = Box::new(suite);
            let leaked_ref = Box::leak(boxed);
            let suite_ptr = leaked_ref as *const _ as *const c_void;
            suite_ptr
        }
        ("OfxMemorySuite", 1) => {
            let suite = memory_suite();
            let boxed = Box::new(suite);
            let leaked_ref = Box::leak(boxed);
            let suite_ptr = leaked_ref as *const _ as *const c_void;
            suite_ptr
        }
        ("OfxMultiThreadSuite", 1) => {
            let suite = multithread_suite();
            let boxed = Box::new(suite);
            let leaked_ref = Box::leak(boxed);
            let suite_ptr = leaked_ref as *const _ as *const c_void;
            suite_ptr
        }
        ("OfxMessageSuite", 1) => {
            let suite = message_suite();
            let boxed = Box::new(suite);
            let leaked_ref = Box::leak(boxed);
            let suite_ptr = leaked_ref as *const _ as *const c_void;
            suite_ptr
        }
        ("OfxInteractSuite", 1) => {
            let suite = interact_suite();
            let boxed = Box::new(suite);
            let leaked_ref = Box::leak(boxed);
            let suite_ptr = leaked_ref as *const _ as *const c_void;
            suite_ptr
        }
        (name, suite_version) => {
            error!("Not implemented: (\"{name}\", {suite_version})");

            std::ptr::null()
        }
    }
}

fn main() {
    let env_filter = tracing_subscriber::filter::EnvFilter::from_default_env();
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(env_filter)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        // .with_max_level(tracing::Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    unsafe {
        // Load ofx file
        let lib = libloading::Library::new(
            "/home/babakinha/dev/clones/openfx/build/Examples/example-Rectangle.ofx",
        )
        .unwrap();

        // Get plugins
        let get_num_plugins: Symbol<unsafe extern "C" fn() -> i32> =
            lib.get(b"OfxGetNumberOfPlugins\0").unwrap();

        trace!("OfxGetNumberOfPlugins");
        let num_plugins = get_num_plugins();
        debug!("get_num_plugins() = {num_plugins}");

        // Fetch each plugin
        let get_plugin: Symbol<unsafe extern "C" fn(i32) -> *mut bindings::root::OfxPlugin> =
            lib.get(b"OfxGetPlugin\0").unwrap();

        let mut plugin_ptrs = Vec::new();
        for plugin_id in 0..num_plugins {
            let plugin_ptr = get_plugin(plugin_id);
            trace!("OfxGetPlugin({plugin_id}) = {plugin_ptr:?}");
            plugin_ptrs.push(plugin_ptr);
        }

        // Instantiate our Host struct
        let mut host_definition = bindings::root::OfxHost {
            host: std::ptr::null_mut(), // TODO: point to real data
            fetchSuite: Some(host_fetch_suite),
        };

        // Load each plugin
        for plugin_ptr in plugin_ptrs {
            let plugin = &*plugin_ptr;

            // Set host
            if let Some(set_host) = plugin.setHost {
                trace!("plugin ({plugin_ptr:?}): set_host(&mut {host_definition:?})");
                set_host(&mut host_definition);
            }

            if let Some(main_entry) = plugin.mainEntry {
                // Try the instance thing
                let mut instance = Box::new(instance::OfxHandle {
                    target: instance::OfxHandleTarget::BabaFx(BabafxInstance::new()),
                });

                if let instance::OfxHandleTarget::BabaFx(babafx) = &mut instance.target {
                    babafx.get_properties_mut().strings.insert(
                        String::from("OfxImageEffectPropContext"),
                        vec![String::from("OfxImageEffectContextGeneral")],
                    );
                }

                let instance_handle = Box::into_raw(instance) as *const c_void;

                debug!("instance_handle = {:?}", instance_handle);

                // Trigger Load Action
                trace!("Action Load");
                let status = main_entry(
                    bindings::root::kOfxActionLoad.as_ptr() as *const i8,
                    instance_handle,
                    std::ptr::null_mut(), // TODO: point to real data
                    std::ptr::null_mut(), // TODO: point to real data
                );

                debug!("status = {}", status);

                // Trigger Image Effect Action Describe
                trace!("Action Describe");
                let status = main_entry(
                    bindings::root::kOfxActionDescribe.as_ptr() as *const i8,
                    instance_handle,
                    std::ptr::null_mut(), // TODO: point to real data
                    std::ptr::null_mut(), // TODO: point to real data
                );
                debug!("status = {}", status);

                // Trigger Image Effect Action Describe
                let mut properties = Box::new(instance::OfxHandle {
                    target: instance::OfxHandleTarget::StandalonePropertySet(
                        instance::StandalonePropertySet::new(),
                    ),
                });
                if let instance::OfxHandleTarget::StandalonePropertySet(prop) =
                    &mut properties.target
                {
                    prop.get_properties_mut().strings.insert(
                        String::from("OfxImageEffectPropContext"),
                        vec![String::from("OfxImageEffectContextFilter")],
                    );
                }
                let properties_handle = Box::into_raw(properties) as *mut OfxPropertySetStruct;

                trace!("Image Effect Action Describe In Context");
                let status = main_entry(
                    bindings::root::kOfxImageEffectActionDescribeInContext.as_ptr() as *const i8,
                    instance_handle,
                    properties_handle,
                    std::ptr::null_mut(), // TODO: point to real data
                );
                debug!("status = {}", status);

                let _reclaimed = Box::from_raw(properties_handle as *mut OfxHandle);

                // Trigger Action Create Instance
                trace!("Action Create Instance");
                let status = main_entry(
                    bindings::root::kOfxActionCreateInstance.as_ptr() as *const i8,
                    instance_handle,
                    std::ptr::null_mut(), // TODO: point to real data
                    std::ptr::null_mut(), // TODO: point to real data
                );
                debug!("status = {}", status);

                trace!("Image Effect Action Get Clip Preferences");
                let output = Box::new(instance::OfxHandle {
                    target: instance::OfxHandleTarget::StandalonePropertySet(
                        instance::StandalonePropertySet::new(),
                    ),
                });
                let output_handle = Box::into_raw(output) as *mut OfxPropertySetStruct;
                let status = main_entry(
                    bindings::root::kOfxImageEffectActionGetClipPreferences.as_ptr() as *const i8,
                    instance_handle,
                    std::ptr::null_mut(), // TODO: point to real data
                    output_handle,
                    // std::ptr::null_mut(), // TODO: point to real data
                );
                debug!(
                    "status = {}, output = {:#?}",
                    status,
                    Box::from_raw(output_handle as *mut OfxHandle)
                );

                trace!("Image Effect Action Get Region Of Definition");
                let mut input = Box::new(instance::OfxHandle {
                    target: instance::OfxHandleTarget::StandalonePropertySet(
                        instance::StandalonePropertySet::new(),
                    ),
                });
                if let instance::OfxHandleTarget::StandalonePropertySet(inp) = &mut input.target {
                    inp.get_properties_mut()
                        .doubles
                        .insert(String::from("OfxPropTime"), vec![1.0]);
                }
                let input_handle = Box::into_raw(input) as *mut OfxPropertySetStruct;
                let output = Box::new(instance::OfxHandle {
                    target: instance::OfxHandleTarget::StandalonePropertySet(
                        instance::StandalonePropertySet::new(),
                    ),
                });
                let output_handle = Box::into_raw(output) as *mut OfxPropertySetStruct;
                let status = main_entry(
                    bindings::root::kOfxImageEffectActionGetRegionOfDefinition.as_ptr()
                        as *const i8,
                    instance_handle,
                    input_handle,
                    output_handle,
                );
                debug!(
                    "status = {}, output = {:#?}",
                    status,
                    Box::from_raw(output_handle as *mut OfxHandle)
                );

                trace!("Image Effect Action Get Region Of Interest");
                let mut input = Box::new(instance::OfxHandle {
                    target: instance::OfxHandleTarget::StandalonePropertySet(
                        instance::StandalonePropertySet::new(),
                    ),
                });
                if let instance::OfxHandleTarget::StandalonePropertySet(inp) = &mut input.target {
                    inp.get_properties_mut().doubles.insert(
                        String::from("OfxImageEffectPropRegionOfInterest"),
                        vec![0.0, 0.0, 0.8, 0.8],
                    );
                    inp.get_properties_mut().doubles.insert(
                        String::from("OfxImageClipPropRoI_Source"),
                        vec![0.0, 0.0, 0.8, 0.8],
                    );
                }
                let input_handle = Box::into_raw(input) as *mut OfxPropertySetStruct;
                let output = Box::new(instance::OfxHandle {
                    target: instance::OfxHandleTarget::StandalonePropertySet(
                        instance::StandalonePropertySet::new(),
                    ),
                });
                let output_handle = Box::into_raw(output) as *mut OfxPropertySetStruct;
                let status = main_entry(
                    bindings::root::kOfxImageEffectActionGetRegionsOfInterest.as_ptr() as *const i8,
                    instance_handle,
                    input_handle,
                    output_handle,
                );
                debug!(
                    "status = {}, output = {:#?}",
                    status,
                    Box::from_raw(output_handle as *mut OfxHandle)
                );

                trace!("Image Effect Action Render");
                let mut input = Box::new(instance::OfxHandle {
                    target: instance::OfxHandleTarget::StandalonePropertySet(
                        instance::StandalonePropertySet::new(),
                    ),
                });
                if let instance::OfxHandleTarget::StandalonePropertySet(inp) = &mut input.target {
                    inp.get_properties_mut()
                        .doubles
                        .insert(String::from("OfxPropTime"), vec![1.0]);
                    inp.get_properties_mut().doubles.insert(
                        String::from("OfxImageEffectPropRenderScale"),
                        vec![720.0, 480.0],
                    );
                    inp.get_properties_mut().ints.insert(
                        String::from("OfxImageEffectPropRenderWindow"),
                        vec![0, 0, 720, 480],
                    );
                }
                let input_handle = Box::into_raw(input) as *mut OfxPropertySetStruct;
                let output = Box::new(instance::OfxHandle {
                    target: instance::OfxHandleTarget::StandalonePropertySet(
                        instance::StandalonePropertySet::new(),
                    ),
                });
                let output_handle = Box::into_raw(output) as *mut OfxPropertySetStruct;
                let status = main_entry(
                    bindings::root::kOfxImageEffectActionRender.as_ptr() as *const i8,
                    instance_handle,
                    input_handle,
                    output_handle,
                );
                debug!(
                    "status = {}, output = {:#?}",
                    status,
                    Box::from_raw(output_handle as *mut OfxHandle)
                );

                // Save?
                let instance = &mut *(instance_handle as *mut OfxHandle);
                if let instance::OfxHandleTarget::BabaFx(baba) = &mut instance.target {
                    debug!("baba: {:#?}", &baba);
                    for (name, clip) in baba.clips.iter_mut() {
                        // 1. Retrieve the raw pointer you stored in the Output clip's PropertySet
                        let raw_ptr: *mut c_void = clip
                            .get_propeties_mut()
                            .pointers
                            .get("OfxImagePropData")
                            .unwrap()[0];

                        // 2. Define your dimensions (must match what you passed to OfxImagePropBounds)
                        let width = 720;
                        let height = 480;
                        let total_bytes = width * height * 4;

                        // 3. Reconstruct a safe Rust slice over the memory to work with it
                        let pixel_slice: &[u8] =
                            std::slice::from_raw_parts(raw_ptr as *const u8, total_bytes);

                        let binding = pixel_slice.to_vec();
                        // 2. Wrap the vector in an RgbaImage (which is an alias for ImageBuffer<Rgba<u8>, Vec<u8>>)
                        if let Some(image) =
                            image::RgbaImage::from_raw(width as u32, height as u32, binding)
                        {
                            // 3. Save it to disk
                            if let Err(e) = image.save(format!("{name}.png")) {
                                error!("Failed to save image: {}", e);
                            }
                        } else {
                            error!("Container was not big enough for the specified dimensions.");
                        }
                    }
                }

                trace!("Action Destroy Instance");
                let status = main_entry(
                    bindings::root::kOfxActionDestroyInstance.as_ptr() as *const i8,
                    instance_handle,
                    std::ptr::null_mut(), // TODO: point to real data
                    std::ptr::null_mut(), // TODO: point to real data
                );
                debug!("status = {}", status);

                trace!("Action Unload");
                let status = main_entry(
                    bindings::root::kOfxActionUnload.as_ptr() as *const i8,
                    instance_handle,
                    std::ptr::null_mut(), // TODO: point to real data
                    std::ptr::null_mut(), // TODO: point to real data
                );
                debug!("status = {}", status);

                let _reclaimed = Box::from_raw(instance_handle as *mut OfxHandle);
            }
        }

        drop(lib);
    }
}
