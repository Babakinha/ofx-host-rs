use crate::bindings::root::{
    self, OfxImageClipStruct, OfxParamSetStruct, OfxPropertySetStruct, kOfxImageComponentRGBA,
    kOfxStatOK,
};
use crate::bindings::root::{
    OfxImageClipHandle, OfxImageEffectHandle, OfxImageMemoryHandle, OfxParamSetHandle,
    OfxPropertySetHandle, OfxRectD, OfxStatus, OfxTime,
};
use crate::instance::{self, AsPropertySet, OfxHandle, PropertySet};
use std::alloc::{Layout, alloc, dealloc};
use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};

const IMAGE_ALIGNMENT: usize = 16;

// Structure stored transparently before the allocated image pointer to track state
#[repr(C)]
struct ImageMemoryHeader {
    total_size: usize,
    lock_count: usize,
}

// ==========================================
// 1. INSTANCE PROPERTY & PARAMETER HOOKS
// ==========================================

unsafe extern "C" fn get_property_set(
    image_effect: OfxImageEffectHandle,
    prop_handle: *mut OfxPropertySetHandle,
) -> OfxStatus {
    dbg!("get_property_set");
    if image_effect.is_null() || prop_handle.is_null() {
        eprintln!("getPropertySet received a NULL handle");
        return 4; // kOfxStatErrBadHandle
    }

    // TODO: uhm... idk if this is right...
    let instance_ptr = image_effect as *mut OfxHandle;
    // let instance = unsafe { &mut *instance_ptr };

    // let prop_set_ptr = instance.get_propeties_mut() as *mut PropertySet;
    unsafe {
        *prop_handle = instance_ptr as *mut OfxPropertySetStruct;
    }

    kOfxStatOK as i32
}

unsafe extern "C" fn get_param_set(
    image_effect: OfxImageEffectHandle,
    param_set: *mut OfxParamSetHandle,
) -> OfxStatus {
    dbg!("get_param_set");
    if image_effect.is_null() || param_set.is_null() {
        eprintln!("getParameterSet received a NULL handle");
        return 4; // kOfxStatErrBadHandle
    }

    // TODO: uhm... idk if this is right...
    let instance_ptr = image_effect as *mut OfxHandle;
    // let instance = unsafe { &mut *instance_ptr };

    // let prop_set_ptr = instance.get_propeties_mut() as *mut PropertySet;
    unsafe {
        *param_set = instance_ptr as *mut OfxParamSetStruct;
    }

    kOfxStatOK as i32
}

// ==========================================
// 2. VIDEO CLIP LIFE-CYCLE & RENDERING
// ==========================================

unsafe extern "C" fn clip_define(
    image_effect: OfxImageEffectHandle,
    name: *const c_char,
    property_set: *mut OfxPropertySetHandle,
) -> OfxStatus {
    dbg!("clip_define");
    let instance_ptr = image_effect as *mut OfxHandle;
    let instance = unsafe { &mut *instance_ptr };

    match &mut instance.target {
        crate::instance::OfxHandleTarget::BabaFx(babafx_instance) => {
            let c_str = unsafe { CStr::from_ptr(name) };
            let name_str = match c_str.to_str() {
                Ok(s) => s.to_string(),
                Err(_) => {
                    dbg!("Error");
                    return 1;
                } // kOfxStatFailed
            };
            dbg!(&instance_ptr, &name_str, &property_set);

            babafx_instance.clips.insert(
                name_str.clone(),
                Box::new(OfxHandle {
                    target: crate::instance::OfxHandleTarget::ClipThing(
                        crate::instance::ClipThing {
                            name: name_str.clone(),
                            properties: {
                                let width: i32 = 720;
                                let height: i32 = 480;
                                let bytes_per_pixel = 4;
                                let total_bytes =
                                    width as usize * height as usize * bytes_per_pixel as usize; // 1,382,400 bytes

                                let mut set = PropertySet::new();

                                // --- Your Existing Setup ---
                                set.strings.insert(
                                    "OfxImageEffectPropPreMultiplication".to_string(),
                                    vec!["OfxImageOpaque".to_string()],
                                );
                                set.strings.insert(
                                    "OfxImageEffectPropPixelDepth".to_string(),
                                    vec!["OfxBitDepthByte".to_string()],
                                );
                                set.strings.insert(
                                    "OfxImageEffectPropComponents".to_string(),
                                    vec!["OfxImageComponentRGBA".to_string()],
                                );

                                // --- The Missing Pieces ---

                                // 1. Image Data Pointer
                                let pixel_buffer = image::ImageReader::open("input.png")
                                    .unwrap()
                                    .decode()
                                    .unwrap()
                                    .to_rgba8()
                                    .into_raw();

                                let pixel_buffer = pixel_buffer.into_boxed_slice();
                                set.pointers.insert(
                                    "OfxImagePropData".to_string(),
                                    vec![Box::into_raw(pixel_buffer) as *mut c_void],
                                );

                                // 2. Image Bounds [x1, y1, x2, y2]
                                set.ints.insert(
                                    "OfxImagePropBounds".to_string(),
                                    vec![0, 0, width, height],
                                );

                                // 3. Row Bytes (Stride)
                                set.ints.insert(
                                    "OfxImagePropRowBytes".to_string(),
                                    vec![width * bytes_per_pixel],
                                );

                                set.doubles.insert(
                                    "OfxImagePropPixelAspectRatio".to_string(),
                                    vec![1.0], // Square pixels
                                );

                                // 4. Interlacing Field Type
                                set.strings.insert(
                                    "OfxImagePropField".to_string(),
                                    vec!["OfxFieldNone".to_string()],
                                );

                                // 5. Unique Frame Identifier (Cache key)
                                set.strings.insert(
                                    "OfxImagePropUniqueIdentifier".to_string(),
                                    vec!["host_frame_0001".to_string()],
                                );

                                set
                            },
                        },
                    ),
                }),
            );

            if !property_set.is_null() {
                unsafe {
                    *property_set = babafx_instance.clips.get_mut(&name_str).unwrap().as_mut()
                        as *mut _ as *mut OfxPropertySetStruct;
                }
            }

            return 0;
        }
        _ => {
            dbg!("Error");
            return 1;
        } // kOfxStatFailed
    }
}

unsafe extern "C" fn clip_get_handle(
    image_effect: OfxImageEffectHandle,
    name: *const c_char,
    clip: *mut OfxImageClipHandle,
    property_set: *mut OfxPropertySetHandle,
) -> OfxStatus {
    dbg!("clip_get_handle");
    let instance_ptr = image_effect as *mut OfxHandle;
    let instance = unsafe { &mut *instance_ptr };

    let c_str = unsafe { CStr::from_ptr(name) };
    let name_str = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            dbg!("Error");
            return 1;
        } // kOfxStatFailed
    };

    dbg!(&instance_ptr, &name_str, &clip, &property_set);

    if let instance::OfxHandleTarget::BabaFx(babafx) = &mut instance.target {
        if let Some(clip_instance) = babafx.clips.get_mut(&name_str) {
            if !clip.is_null() {
                unsafe {
                    *clip = clip_instance.as_mut() as *mut _ as *mut OfxImageClipStruct;
                }
            }

            if !property_set.is_null() {
                unsafe {
                    // TODO: Techinically will work... i think?
                    *property_set = clip_instance.as_mut() as *mut _ as *mut OfxPropertySetStruct;
                }
            }
        } else {
            dbg!("Error");
            return 1;
        }
    } else {
        dbg!("Error");
        return 1; // Error
    }

    0
}

unsafe extern "C" fn clip_get_property_set(
    clip: OfxImageClipHandle,
    prop_handle: *mut OfxPropertySetHandle,
) -> OfxStatus {
    dbg!("clip_get_property_set");

    // TODO: uhm... idk if this is right...
    let instance_ptr = clip as *mut OfxHandle;
    // let instance = unsafe { &mut *instance_ptr };
    dbg!(&instance_ptr, &prop_handle);

    // let prop_set_ptr = instance.get_propeties_mut() as *mut PropertySet;
    unsafe {
        *prop_handle = instance_ptr as *mut OfxPropertySetStruct;
    }

    kOfxStatOK as i32
}

unsafe extern "C" fn clip_get_image(
    clip: OfxImageClipHandle,
    _time: OfxTime,
    _region: *const OfxRectD,
    image_handle: *mut OfxPropertySetHandle,
) -> OfxStatus {
    dbg!("clip_get_image");
    eprintln!("clip_get_image half implemented");
    let instance_ptr = clip as *mut OfxHandle;
    let instance = unsafe { &mut *instance_ptr };

    dbg!(&clip, &_time, &_region, &image_handle);

    if let instance::OfxHandleTarget::ClipThing(_clip) = &mut instance.target {
        unsafe {
            *image_handle = instance_ptr as *mut OfxPropertySetStruct;
        }
    } else {
        dbg!("Error");
        return 1;
    }

    0
}

unsafe extern "C" fn clip_release_image(image_handle: OfxPropertySetHandle) -> OfxStatus {
    dbg!("clip_release_image");
    eprintln!("clip_release_image half implemented");
    let instance_ptr = image_handle as *mut OfxHandle;
    let instance = unsafe { &mut *instance_ptr };
    dbg!(&instance_ptr);

    // 1. Retrieve the raw pointer you stored in the Output clip's PropertySet
    let raw_ptr: *mut c_void = instance
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
        unsafe { std::slice::from_raw_parts(raw_ptr as *const u8, total_bytes) };

    let binding = pixel_slice.to_vec();
    // 2. Wrap the vector in an RgbaImage (which is an alias for ImageBuffer<Rgba<u8>, Vec<u8>>)
    if let Some(image) = image::RgbaImage::from_raw(width as u32, height as u32, binding) {
        // 3. Save it to disk
        if let Err(e) = image.save("test.png") {
            eprintln!("Failed to save image: {}", e);
        }
    } else {
        eprintln!("Container was not big enough for the specified dimensions.");
    }
    0
}

unsafe extern "C" fn clip_get_region_of_definition(
    clip: OfxImageClipHandle,
    _time: OfxTime,
    bounds: *mut OfxRectD,
) -> OfxStatus {
    dbg!("clip_get_region_of_definition");

    if clip.is_null() || bounds.is_null() {
        dbg!(clip, bounds);
        return 1; // kOfxStatErrBadHandle / kOfxStatErrBadToctx
    }


    let clip_handle = clip as *mut OfxHandle;
    let instance = unsafe { &*clip_handle };

    dbg!(&clip_handle, &_time, &bounds);

    if let instance::OfxHandleTarget::ClipThing(clip_thing) = &instance.target {
        // Look up the integer bounds you stored in this clip when you created it
        if let Some(int_bounds) = clip_thing.properties.ints.get("OfxImagePropBounds") {
            // int_bounds is typically [x1, y1, x2, y2] -> [0, 0, 720, 480]
            dbg!(&int_bounds);
            unsafe {
                (*bounds).x1 = int_bounds[0] as f64;
                (*bounds).y1 = int_bounds[1] as f64;
                (*bounds).x2 = int_bounds[2] as f64;
                (*bounds).y2 = int_bounds[3] as f64;
            }
            return 0; // kOfxStatOK
        }
    }

    dbg!("Fallback");
    // Fallback if the clip doesn't have bounds populated yet
    unsafe {
        (*bounds).x1 = 0.0;
        (*bounds).y1 = 0.0;
        (*bounds).x2 = 720.0;
        (*bounds).y2 = 480.0;
    }

    0
}

// ==========================================
// 3. EXECUTION CONTROL
// ==========================================

unsafe extern "C" fn abort(_image_effect: OfxImageEffectHandle) -> c_int {
    dbg!("abort");
    // Return 0 indicating the effect processing thread should continue running safely.
    0
}

// ==========================================
// 4. ALIGNED CUSTOM IMAGE POOL ALLOCATOR
// ==========================================

unsafe extern "C" fn image_memory_alloc(
    _instance_handle: OfxImageEffectHandle,
    n_bytes: usize,
    memory_handle: *mut OfxImageMemoryHandle,
) -> OfxStatus {
    dbg!("image_memory_alloc");
    if memory_handle.is_null() || n_bytes == 0 {
        {
            dbg!("Error");
            return 1;
        } // kOfxStatFailed
    }

    let header_size = std::mem::size_of::<ImageMemoryHeader>();
    // Padding ensures that the user data pointer following our header remains 16-byte aligned
    let padding = if header_size % IMAGE_ALIGNMENT == 0 {
        header_size
    } else {
        ((header_size / IMAGE_ALIGNMENT) + 1) * IMAGE_ALIGNMENT
    };

    let total_size = n_bytes + padding;

    match Layout::from_size_align(total_size, IMAGE_ALIGNMENT) {
        Ok(layout) => {
            let raw_ptr = alloc(layout);
            if raw_ptr.is_null() {
                *memory_handle = std::ptr::null_mut();
                return 3; // kOfxStatErrMemory
            }

            // Write metadata structure header
            let header_ptr = raw_ptr as *mut ImageMemoryHeader;
            unsafe {
                *header_ptr = ImageMemoryHeader {
                    total_size,
                    lock_count: 0,
                };
            }

            // The memory handle returned back to the host system context
            *memory_handle = raw_ptr as OfxImageMemoryHandle;
            0 // kOfxStatOK
        }
        Err(_) => {
            *memory_handle = std::ptr::null_mut();
            1
        }
    }
}

unsafe extern "C" fn image_memory_free(memory_handle: OfxImageMemoryHandle) -> OfxStatus {
    dbg!("image_memory_free");
    if memory_handle.is_null() {
        return 0; // kOfxStatOK
    }

    let raw_ptr = memory_handle as *mut u8;
    let header_ptr = raw_ptr as *mut ImageMemoryHeader;
    let total_size = unsafe { (*header_ptr).total_size };

    if let Ok(layout) = Layout::from_size_align(total_size, IMAGE_ALIGNMENT) {
        dealloc(raw_ptr, layout);
        0 // kOfxStatOK
    } else {
        4 // kOfxStatErrBadHandle
    }
}

unsafe extern "C" fn image_memory_lock(
    memory_handle: OfxImageMemoryHandle,
    returned_ptr: *mut *mut c_void,
) -> OfxStatus {
    dbg!("image_memory_lock");
    if memory_handle.is_null() || returned_ptr.is_null() {
        return 4; // kOfxStatErrBadHandle
    }

    let raw_ptr = memory_handle as *mut u8;
    let header_ptr = raw_ptr as *mut ImageMemoryHeader;

    unsafe {
        (*header_ptr).lock_count += 1;
    }

    let header_size = std::mem::size_of::<ImageMemoryHeader>();
    let padding = if header_size % IMAGE_ALIGNMENT == 0 {
        header_size
    } else {
        ((header_size / IMAGE_ALIGNMENT) + 1) * IMAGE_ALIGNMENT
    };

    // Return the aligned client address offset past the metadata layout tracking area
    unsafe {
        *returned_ptr = raw_ptr.add(padding) as *mut c_void;
    }
    0
}

unsafe extern "C" fn image_memory_unlock(memory_handle: OfxImageMemoryHandle) -> OfxStatus {
    dbg!("image_memory_unlock");
    if memory_handle.is_null() {
        return 4;
    }

    let raw_ptr = memory_handle as *mut u8;
    let header_ptr = raw_ptr as *mut ImageMemoryHeader;

    unsafe {
        if (*header_ptr).lock_count > 0 {
            (*header_ptr).lock_count -= 1;
        }
    }
    0
}

// ==========================================
// SUITE BUILDER
// ==========================================

pub fn image_effect_suite() -> root::OfxImageEffectSuiteV1 {
    dbg!("image_effect_suite");
    root::OfxImageEffectSuiteV1 {
        getPropertySet: Some(get_property_set),
        getParamSet: Some(get_param_set),
        clipDefine: Some(clip_define),
        clipGetHandle: Some(clip_get_handle),
        clipGetPropertySet: Some(clip_get_property_set),
        clipGetImage: Some(clip_get_image),
        clipReleaseImage: Some(clip_release_image),
        clipGetRegionOfDefinition: Some(clip_get_region_of_definition),
        abort: Some(abort),
        imageMemoryAlloc: Some(image_memory_alloc),
        imageMemoryFree: Some(image_memory_free),
        imageMemoryLock: Some(image_memory_lock),
        imageMemoryUnlock: Some(image_memory_unlock),
    }
}
