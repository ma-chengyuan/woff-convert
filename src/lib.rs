use wasm_bindgen::prelude::*;

extern "C" {
    fn woff2_MaxWOFF2CompressedSize(data: *const u8, length: usize) -> usize;

    fn woff2_ConvertTTFToWOFF2(
        data: *const u8,
        length: usize,
        result: *mut u8,
        result_length: *mut usize,
        quality: i32,
    ) -> bool;

    fn woff2_ComputeWOFF2FinalSize(data: *const u8, length: usize) -> usize;

    fn woff2_ConvertWOFF2ToTTF(
        result: *mut u8,
        result_length: usize,
        data: *const u8,
        length: usize,
    ) -> bool;

    fn woff2_ConvertWOFF2ToTTFString(
        data: *const u8,
        length: usize,
        result_length: *mut usize,
    ) -> *mut u8;

    fn woff2_ConvertWOFF2ToTTFStringFinalize(result: *mut u8, result_length: usize, s: *mut u8);
}

#[derive(Debug, Clone, Copy)]
pub struct Woff2Error;

impl From<Woff2Error> for JsValue {
    fn from(_: Woff2Error) -> JsValue {
        JsValue::from_str("Woff2Error")
    }
}

/// Converts a TTF font to WOFF2 format.
#[wasm_bindgen]
pub fn convert_ttf_to_woff2(data: &[u8], quality: i32) -> Result<Vec<u8>, Woff2Error> {
    unsafe {
        let mut result_length = woff2_MaxWOFF2CompressedSize(data.as_ptr(), data.len());
        let mut result = vec![0; result_length];
        let success = woff2_ConvertTTFToWOFF2(
            data.as_ptr(),
            data.len(),
            result.as_mut_ptr(),
            &mut result_length,
            quality,
        );
        if !success {
            return Err(Woff2Error);
        }
        result.truncate(result_length);
        Ok(result)
    }
}

/// Converts a WOFF2 font to TTF format.
#[wasm_bindgen]
pub fn convert_woff2_to_ttf(data: &[u8]) -> Result<Vec<u8>, Woff2Error> {
    unsafe {
        let result_length = woff2_ComputeWOFF2FinalSize(data.as_ptr(), data.len());
        let mut result = vec![0; result_length];
        let success = woff2_ConvertWOFF2ToTTF(
            result.as_mut_ptr(),
            result_length,
            data.as_ptr(),
            data.len(),
        );
        if !success {
            // The approach above infers final size of the decompressed TTF font
            // by inspecting the WOFF2 header. This may not be accurate in all
            // cases. The approach below is more error-tolerant, as it allows
            // the decompressor to allocate memory as needed.
            // See https://github.com/fontforge/fontforge/issues/5101 for
            // context, and https://github.com/fontforge/fontforge/pull/5160 for
            // the fix.
            let mut result_length = 0usize;
            let result_string =
                woff2_ConvertWOFF2ToTTFString(data.as_ptr(), data.len(), &mut result_length);
            if result_string.is_null() {
                return Err(Woff2Error);
            }
            result.resize(result_length, 0);
            woff2_ConvertWOFF2ToTTFStringFinalize(
                result.as_mut_ptr(),
                result_length,
                result_string,
            );
        }
        Ok(result)
    }
}

// Need to polyfill malloc and free
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
    let layout = std::alloc::Layout::from_size_align(4 + size, 4).unwrap();
    let ptr = std::alloc::alloc(layout);
    *(ptr as *mut u32) = size as u32;
    ptr.add(4)
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }
    let ptr = ptr.sub(4);
    let size = *(ptr as *const u32) as usize;
    let layout = std::alloc::Layout::from_size_align(4 + size, 4).unwrap();
    std::alloc::dealloc(ptr, layout);
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub unsafe extern "C" fn exit(code: i32) {
    panic!("exit({})", code);
}
