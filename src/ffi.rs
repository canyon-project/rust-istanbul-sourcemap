use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use crate::transform_istanbul_coverage;

/// Transform Istanbul coverage data (C FFI interface)
/// 
/// # Safety
/// 
/// This function is unsafe because it dereferences raw pointers.
/// The caller must ensure that:
/// - `input` is a valid null-terminated C string
/// - The returned pointer must be freed using `free_string`
#[no_mangle]
pub unsafe extern "C" fn transform_coverage_ffi(input: *const c_char) -> *mut c_char {
    if input.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = match CStr::from_ptr(input).to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    match transform_istanbul_coverage(c_str) {
        Ok(result) => {
            match CString::new(result) {
                Ok(c_string) => c_string.into_raw(),
                Err(_) => std::ptr::null_mut(),
            }
        }
        Err(_) => std::ptr::null_mut(),
    }
}

/// Free a string allocated by the library
/// 
/// # Safety
/// 
/// This function is unsafe because it takes ownership of a raw pointer.
/// The caller must ensure that:
/// - `ptr` was allocated by this library
/// - `ptr` is not used after calling this function
#[no_mangle]
pub unsafe extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = CString::from_raw(ptr);
    }
}

/// Get the library version
#[no_mangle]
pub extern "C" fn get_version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr() as *const c_char
}