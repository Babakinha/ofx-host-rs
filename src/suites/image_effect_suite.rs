use crate::bindings::root::{self, OfxParamSetStruct};
use crate::bindings::root::{
    OfxImageClipHandle, OfxImageEffectHandle, OfxImageMemoryHandle, OfxParamSetHandle,
    OfxPropertySetHandle, OfxRectD, OfxStatus, OfxTime,
};
use crate::instance::{self, AsPropertySet, BabafxInstance, ImageClip, PropertySet};
use crate::log_utils::c_str_to_str;
use crate::ofx_constants::{kOfxStatErrBadHandle, kOfxStatErrMemory, kOfxStatFailed, kOfxStatOK};
use std::alloc::{Layout, alloc, dealloc};
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};
use tracing::{error, instrument, warn};

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

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn get_property_set(
    image_effect: OfxImageEffectHandle,
    prop_handle: *mut OfxPropertySetHandle,
) -> OfxStatus {
    if image_effect.is_null() || prop_handle.is_null() {
        error!("getPropertySet received a NULL handle");
        return kOfxStatErrBadHandle;
    }
    let instance = unsafe { BabafxInstance::ref_mut_from_ofx_handle(image_effect).unwrap() };

    unsafe {
        *prop_handle = instance.get_properties_mut().as_raw_ofx_handle();
    }

    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn get_param_set(
    image_effect: OfxImageEffectHandle,
    param_set: *mut OfxParamSetHandle,
) -> OfxStatus {
    if image_effect.is_null() || param_set.is_null() {
        error!("getParameterSet received a NULL handle");
        return kOfxStatErrBadHandle;
    }

    // TODO: uhm... idk if this is right...
    unsafe {
        *param_set = image_effect as *mut OfxParamSetStruct;
    }

    kOfxStatOK
}

// ==========================================
// 2. VIDEO CLIP LIFE-CYCLE & RENDERING
// ==========================================

#[instrument(level = "trace", ret(level = "trace"), fields(name = c_str_to_str(name)))]
unsafe extern "C" fn clip_define(
    image_effect: OfxImageEffectHandle,
    name: *const c_char,
    property_set: *mut OfxPropertySetHandle,
) -> OfxStatus {
    let instance = unsafe { BabafxInstance::ref_mut_from_ofx_handle(image_effect).unwrap() };

    let c_str = unsafe { CStr::from_ptr(name) };
    let name_str = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Error");
            return kOfxStatFailed;
        }
    };

    instance.clips.insert(
        name_str.clone(),
        crate::instance::ImageClip {
            name: name_str.clone(),
            properties: {
                let width: i32 = 720;
                let height: i32 = 480;
                let bytes_per_pixel = 4;
                let _total_bytes = width as usize * height as usize * bytes_per_pixel as usize;

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
                set.ints
                    .insert("OfxImagePropBounds".to_string(), vec![0, 0, width, height]);

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
    );

    if !property_set.is_null() {
        unsafe {
            *property_set = instance
                .clips
                .get_mut(&name_str)
                .unwrap()
                .get_properties_mut()
                .as_raw_ofx_handle();
        }
    }

    return kOfxStatOK;
}

#[instrument(level = "trace", ret(level = "trace"), fields(name = c_str_to_str(name)))]
unsafe extern "C" fn clip_get_handle(
    image_effect: OfxImageEffectHandle,
    name: *const c_char,
    clip: *mut OfxImageClipHandle,
    property_set: *mut OfxPropertySetHandle,
) -> OfxStatus {
    let instance = unsafe { BabafxInstance::ref_mut_from_ofx_handle(image_effect).unwrap() };

    let c_str = unsafe { CStr::from_ptr(name) };
    let name_str = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Error");
            return kOfxStatFailed;
        }
    };

    if let Some(clip_instance) = instance.clips.get_mut(&name_str) {
        if !clip.is_null() {
            unsafe {
                *clip = clip_instance.as_raw_ofx_handle();
            }
        }

        if !property_set.is_null() {
            unsafe {
                *property_set = clip_instance.get_properties_mut().as_raw_ofx_handle();
            }
        }
    } else {
        error!("Error");
        return kOfxStatFailed;
    }

    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn clip_get_property_set(
    clip: OfxImageClipHandle,
    prop_handle: *mut OfxPropertySetHandle,
) -> OfxStatus {
    if clip.is_null() || prop_handle.is_null() {
        error!("getPropertySet received a NULL handle");
        return kOfxStatErrBadHandle;
    }
    let instance = unsafe { ImageClip::ref_mut_from_ofx_handle(clip).unwrap() };

    unsafe {
        *prop_handle = instance.get_properties_mut().as_raw_ofx_handle();
    }

    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn clip_get_image(
    clip: OfxImageClipHandle,
    _time: OfxTime,
    _region: *const OfxRectD,
    image_handle: *mut OfxPropertySetHandle,
) -> OfxStatus {
    warn!("clip_get_image half implemented");
    let instance = unsafe { ImageClip::ref_mut_from_ofx_handle(clip).unwrap() };

    unsafe {
        *image_handle = instance.get_properties_mut().as_raw_ofx_handle();
    }

    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn clip_release_image(image_handle: OfxPropertySetHandle) -> OfxStatus {
    warn!("clip_release_image half implemented");
    let instance = unsafe { instance::PropertySet::ref_mut_from_ofx_handle(image_handle).unwrap() };

    // 1. Retrieve the raw pointer you stored in the Output clip's PropertySet
    let raw_ptr: *mut c_void = instance.pointers.get("OfxImagePropData").unwrap()[0];

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
            error!("Failed to save image: {}", e);
        }
    } else {
        error!("Container was not big enough for the specified dimensions.");
    }
    kOfxStatOK
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn clip_get_region_of_definition(
    clip: OfxImageClipHandle,
    _time: OfxTime,
    bounds: *mut OfxRectD,
) -> OfxStatus {
    if clip.is_null() || bounds.is_null() {
        error!("Error");
        return kOfxStatFailed;
    }

    let instance = unsafe { ImageClip::ref_mut_from_ofx_handle(clip).unwrap() };

    if let Some(int_bounds) = instance.properties.ints.get("OfxImagePropBounds") {
        // int_bounds is typically [x1, y1, x2, y2] -> [0, 0, 720, 480]
        unsafe {
            (*bounds).x1 = int_bounds[0] as f64;
            (*bounds).y1 = int_bounds[1] as f64;
            (*bounds).x2 = int_bounds[2] as f64;
            (*bounds).y2 = int_bounds[3] as f64;
        }
        return kOfxStatOK;
    }

    warn!("Fallback");
    // Fallback if the clip doesn't have bounds populated yet
    unsafe {
        (*bounds).x1 = 0.0;
        (*bounds).y1 = 0.0;
        (*bounds).x2 = 720.0;
        (*bounds).y2 = 480.0;
    }

    kOfxStatOK
}

// ==========================================
// 3. EXECUTION CONTROL
// ==========================================

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn abort(_image_effect: OfxImageEffectHandle) -> c_int {
    // Return 0 indicating the effect processing thread should continue running safely.
    error!("abort not implemented");
    0
}

// ==========================================
// 4. ALIGNED CUSTOM IMAGE POOL ALLOCATOR
// ==========================================

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn image_memory_alloc(
    _instance_handle: OfxImageEffectHandle,
    n_bytes: usize,
    memory_handle: *mut OfxImageMemoryHandle,
) -> OfxStatus {
    if memory_handle.is_null() || n_bytes == 0 {
        {
            error!("Error");
            return kOfxStatFailed;
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
            unsafe {
                let raw_ptr = alloc(layout);
                if raw_ptr.is_null() {
                    *memory_handle = std::ptr::null_mut();
                    return kOfxStatErrMemory;
                }

                // Write metadata structure header
                let header_ptr = raw_ptr as *mut ImageMemoryHeader;
                *header_ptr = ImageMemoryHeader {
                    total_size,
                    lock_count: 0,
                };

                // The memory handle returned back to the host system context
                *memory_handle = raw_ptr as OfxImageMemoryHandle;
            }
            return kOfxStatOK;
        }
        Err(_) => {
            unsafe {
                *memory_handle = std::ptr::null_mut();
            }
            kOfxStatFailed
        }
    }
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn image_memory_free(memory_handle: OfxImageMemoryHandle) -> OfxStatus {
    if memory_handle.is_null() {
        return kOfxStatOK;
    }

    let raw_ptr = memory_handle as *mut u8;
    let header_ptr = raw_ptr as *mut ImageMemoryHeader;
    let total_size = unsafe { (*header_ptr).total_size };

    if let Ok(layout) = Layout::from_size_align(total_size, IMAGE_ALIGNMENT) {
        unsafe {
            dealloc(raw_ptr, layout);
        }
        kOfxStatOK
    } else {
        kOfxStatErrBadHandle
    }
}

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn image_memory_lock(
    memory_handle: OfxImageMemoryHandle,
    returned_ptr: *mut *mut c_void,
) -> OfxStatus {
    if memory_handle.is_null() || returned_ptr.is_null() {
        return kOfxStatErrBadHandle;
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

#[instrument(level = "trace", ret(level = "trace"))]
unsafe extern "C" fn image_memory_unlock(memory_handle: OfxImageMemoryHandle) -> OfxStatus {
    if memory_handle.is_null() {
        return kOfxStatErrBadHandle;
    }

    let raw_ptr = memory_handle as *mut u8;
    let header_ptr = raw_ptr as *mut ImageMemoryHeader;

    unsafe {
        if (*header_ptr).lock_count > 0 {
            (*header_ptr).lock_count -= 1;
        }
    }
    kOfxStatOK
}

// ==========================================
// SUITE BUILDER
// ==========================================

#[instrument(level = "trace", ret(level = "trace"))]
pub fn image_effect_suite() -> root::OfxImageEffectSuiteV1 {
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
