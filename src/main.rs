#![feature(c_variadic)]
#![feature(box_as_ptr)]
use libloading::Symbol;

pub mod bindings {
    #![allow(unsafe_op_in_unsafe_fn)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod instance;
mod suites;

use std::ffi::{CStr, c_char, c_void};

use crate::{
    bindings::root::OfxPropertySetStruct, instance::{AsPropertySet, BabafxInstance, OfxHandle}, suites::{
        image_effect_suite::image_effect_suite, interact_suite::interact_suite,
        memory_suite::memory_suite, message_suite::message_suite,
        multithread_suite::multithread_suite, parameter_suite::parameter_suite,
        property_suite::property_suite,
    }
};

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
            eprintln!("Not implemented: (\"{name}\", {suite_version})");

            std::ptr::null()
        }
    }
}

fn main() {
    unsafe {
        // Load ofx file
        let lib = libloading::Library::new("./example-Rectangle.ofx").unwrap();

        // Get plugins
        let get_num_plugins: Symbol<unsafe extern "C" fn() -> i32> =
            lib.get(b"OfxGetNumberOfPlugins\0").unwrap();

        let num_plugins = get_num_plugins();
        println!("Found {} plugins in this binary", num_plugins);

        // Fetch each plugin
        let get_plugin: Symbol<unsafe extern "C" fn(i32) -> *mut bindings::root::OfxPlugin> =
            lib.get(b"OfxGetPlugin\0").unwrap();

        let mut plugin_ptrs = Vec::new();
        for plugin_id in 0..num_plugins {
            let plugin_ptr = get_plugin(plugin_id);
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
                set_host(&mut host_definition);
            }

            if let Some(main_entry) = plugin.mainEntry {
                // Try the instance thing
                let mut instance = Box::new(instance::OfxHandle {
                    target: instance::OfxHandleTarget::BabaFx(BabafxInstance::new()),
                });

                if let instance::OfxHandleTarget::BabaFx(babafx) = &mut instance.target {
                    babafx.get_properties_mut().strings.insert(String::from("OfxImageEffectPropContext"), vec![String::from("OfxImageEffectContextFilter")]);

                }

                let instance_handle = Box::into_raw(instance) as *const c_void;

                dbg!(instance_handle);

                // Trigger Load Action
                dbg!("Action Load");
                let status = main_entry(
                    bindings::root::kOfxActionLoad.as_ptr() as *const i8,
                    instance_handle,
                    std::ptr::null_mut(), // TODO: point to real data
                    std::ptr::null_mut(), // TODO: point to real data
                );

                dbg!(status);

                // Trigger Image Effect Action Describe
                dbg!("Action Describe");
                let status = main_entry(
                    bindings::root::kOfxActionDescribe.as_ptr() as *const i8,
                    instance_handle,
                    std::ptr::null_mut(), // TODO: point to real data
                    std::ptr::null_mut(), // TODO: point to real data
                );
                dbg!(status);

                // Trigger Image Effect Action Describe
                let properties = Box::new(instance::OfxHandle {
                    target: instance::OfxHandleTarget::StandalonePropertySet(instance::StandalonePropertySet::new()),
                });
                let properties_handle = Box::into_raw(properties) as *mut OfxPropertySetStruct;

                dbg!("Image Effect Action Describe In Context");
                let status = main_entry(
                    bindings::root::kOfxImageEffectActionDescribeInContext.as_ptr() as *const i8,
                    instance_handle,
                    properties_handle,
                    std::ptr::null_mut(), // TODO: point to real data
                );
                dbg!(status);

                let _reclaimed = Box::from_raw(properties_handle as *mut OfxHandle);

                // Trigger Action Create Instance
                dbg!("Action Create Instance");
                let status = main_entry(
                    bindings::root::kOfxActionCreateInstance.as_ptr() as *const i8,
                    instance_handle,
                    std::ptr::null_mut(), // TODO: point to real data
                    std::ptr::null_mut(), // TODO: point to real data
                );
                dbg!(status);


                dbg!("Image Effect Action Get Clip Preferences");
                let status = main_entry(
                    bindings::root::kOfxImageEffectActionGetClipPreferences.as_ptr() as *const i8,
                    instance_handle,
                    std::ptr::null_mut(), // TODO: point to real data
                    std::ptr::null_mut(), // TODO: point to real data
                );
                dbg!(status);

                dbg!("Image Effect Action Get Region Of Definition");
                let status = main_entry(
                    bindings::root::kOfxImageEffectActionGetRegionOfDefinition.as_ptr() as *const i8,
                    instance_handle,
                    std::ptr::null_mut(), // TODO: point to real data
                    std::ptr::null_mut(), // TODO: point to real data
                );
                dbg!(status);

                dbg!("Image Effect Action Get Region Of Interest");
                let status = main_entry(
                    bindings::root::kOfxImageEffectActionGetRegionsOfInterest.as_ptr() as *const i8,
                    instance_handle,
                    std::ptr::null_mut(), // TODO: point to real data
                    std::ptr::null_mut(), // TODO: point to real data
                );
                dbg!(status);

                dbg!("Image Effect Action Render");
                let status = main_entry(
                    bindings::root::kOfxImageEffectActionRender.as_ptr() as *const i8,
                    instance_handle,
                    std::ptr::null_mut(), // TODO: point to real data
                    std::ptr::null_mut(), // TODO: point to real data
                );
                dbg!(status);

                dbg!("Action Destroy Instance");
                let status = main_entry(
                    bindings::root::kOfxActionDestroyInstance.as_ptr() as *const i8,
                    instance_handle,
                    std::ptr::null_mut(), // TODO: point to real data
                    std::ptr::null_mut(), // TODO: point to real data
                );
                dbg!(status);

                dbg!("Action Unload");
                let status = main_entry(
                    bindings::root::kOfxActionUnload.as_ptr() as *const i8,
                    instance_handle,
                    std::ptr::null_mut(), // TODO: point to real data
                    std::ptr::null_mut(), // TODO: point to real data
                );
                dbg!(status);

                let _reclaimed = Box::from_raw(instance_handle as *mut OfxHandle);
            }
        }

        drop(lib);
    }
}
