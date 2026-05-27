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
mod ofx_constants;
mod suites;

use std::ffi::{CStr, c_char, c_void};

use crate::{
    bindings::root::OfxImageEffectHandle,
    instance::{AsPropertySet, BabafxInstance, PropertySet},
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
        .with_span_events(
            tracing_subscriber::fmt::format::FmtSpan::ENTER
                | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
        )
        // .with_max_level(tracing::Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    unsafe {
        // Load ofx file
        let effect_path = std::env::args().skip(1).next().unwrap();
        debug!("effect_path = {effect_path}");
        let lib = libloading::Library::new(&effect_path).unwrap();

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
        let mut host_properties = instance::PropertySet::new();
        host_properties
            .doubles
            .insert("Mreow".to_string(), vec![1.0]);
        let host_properties_handle = host_properties.to_raw_ofx_handle();

        let host_definition = Box::new(bindings::root::OfxHost {
            host: host_properties_handle, // TODO: point to real data
            fetchSuite: Some(host_fetch_suite),
        });

        let host_ptr = Box::into_raw(host_definition);

        // Load each plugin
        for plugin_ptr in plugin_ptrs {
            let plugin = &*plugin_ptr;

            // Set host
            if let Some(set_host) = plugin.setHost {
                trace!(
                    "plugin ({plugin_ptr:?}): set_host(&mut {:?})",
                    host_ptr.as_mut()
                );
                set_host(host_ptr.as_mut().unwrap());
            }

            if let Some(main_entry) = plugin.mainEntry {
                // Try the instance thing
                let mut instance = BabafxInstance::new();
                instance.get_properties_mut().strings.insert(
                    String::from("OfxImageEffectPropContext"),
                    vec![String::from("OfxImageEffectContextGeneral")],
                );
                let instance_handle = instance.to_raw_ofx_handle() as *const c_void;

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
                let mut properties = instance::PropertySet::new();
                properties.strings.insert(
                    String::from("OfxImageEffectPropContext"),
                    vec![String::from("OfxImageEffectContextFilter")],
                );
                let properties_handle = properties.to_raw_ofx_handle();

                trace!("Image Effect Action Describe In Context");
                let status = main_entry(
                    bindings::root::kOfxImageEffectActionDescribeInContext.as_ptr() as *const i8,
                    instance_handle,
                    properties_handle,
                    std::ptr::null_mut(), // TODO: point to real data
                );
                debug!("status = {}", status);

                let _reclaimed = PropertySet::from_ofx_handle(properties_handle);

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
                let output = instance::PropertySet::new();
                let output_handle = output.to_raw_ofx_handle();
                let status = main_entry(
                    bindings::root::kOfxImageEffectActionGetClipPreferences.as_ptr() as *const i8,
                    instance_handle,
                    std::ptr::null_mut(), // TODO: point to real data
                    output_handle,
                    // std::ptr::null_mut(), // TODO: point to real data
                );
                let _output = PropertySet::from_ofx_handle(output_handle);
                debug!("status = {}, output = {:#?}", status, _output);

                trace!("Image Effect Action Get Region Of Definition");
                let mut input = instance::PropertySet::new();
                input.doubles.insert(String::from("OfxPropTime"), vec![1.0]);
                let input_handle = input.to_raw_ofx_handle();
                let output = instance::PropertySet::new();
                let output_handle = output.to_raw_ofx_handle();
                let status = main_entry(
                    bindings::root::kOfxImageEffectActionGetRegionOfDefinition.as_ptr()
                        as *const i8,
                    instance_handle,
                    input_handle,
                    output_handle,
                );
                let _input = PropertySet::from_ofx_handle(input_handle);
                let _output = PropertySet::from_ofx_handle(output_handle);
                debug!("status = {}, output = {:#?}", status, _output);

                trace!("Image Effect Action Get Region Of Interest");
                let mut input = instance::PropertySet::new();
                input.doubles.insert(
                    String::from("OfxImageEffectPropRegionOfInterest"),
                    vec![0.0, 0.0, 0.8, 0.8],
                );
                input.doubles.insert(
                    String::from("OfxImageClipPropRoI_Source"),
                    vec![0.0, 0.0, 0.8, 0.8],
                );
                let input_handle = input.to_raw_ofx_handle();
                let output = instance::PropertySet::new();
                let output_handle = output.to_raw_ofx_handle();
                let status = main_entry(
                    bindings::root::kOfxImageEffectActionGetRegionsOfInterest.as_ptr() as *const i8,
                    instance_handle,
                    input_handle,
                    output_handle,
                );
                let _input = PropertySet::from_ofx_handle(input_handle);
                let _output = PropertySet::from_ofx_handle(output_handle);

                debug!("status = {}, output = {:#?}", status, _output);

                trace!("Image Effect Action Render");
                let mut input = instance::PropertySet::new();
                input.doubles.insert(String::from("OfxPropTime"), vec![1.0]);
                input.doubles.insert(
                    String::from("OfxImageEffectPropRenderScale"),
                    vec![720.0, 480.0],
                );
                input.ints.insert(
                    String::from("OfxImageEffectPropRenderWindow"),
                    vec![0, 0, 720, 480],
                );
                let input_handle = input.to_raw_ofx_handle();
                let output = instance::PropertySet::new();
                let output_handle = output.to_raw_ofx_handle();
                let status = main_entry(
                    bindings::root::kOfxImageEffectActionRender.as_ptr() as *const i8,
                    instance_handle,
                    input_handle,
                    output_handle,
                );
                let _input = PropertySet::from_ofx_handle(input_handle);
                let _output = PropertySet::from_ofx_handle(output_handle);

                debug!("status = {}, output = {:#?}", status, _output);

                // Save?
                let instance = BabafxInstance::ref_mut_from_ofx_handle(
                    instance_handle as OfxImageEffectHandle,
                )
                .unwrap();
                debug!("baba: {:#?}", instance);
                for (name, clip) in instance.clips.iter_mut() {
                    // 1. Retrieve the raw pointer you stored in the Output clip's PropertySet
                    let raw_ptr: *mut c_void = clip
                        .get_properties_mut()
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

                let _reclaimed =
                    BabafxInstance::from_ofx_handle(instance_handle as OfxImageEffectHandle);
            }
        }

        drop(lib);
    }
}
