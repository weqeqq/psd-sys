#![allow(non_camel_case_types)]

use std::ffi::{c_char, c_uchar, c_uint, c_void};

use crate::error::psd_error;

#[repr(C)]
pub struct psd_layer(c_void);

unsafe extern "C" {
    pub fn psd_layer_new(name: *const c_char) -> *mut psd_layer;
    pub fn psd_layer_copy(psd_layer: *const psd_layer) -> *mut psd_layer;
    pub fn psd_layer_delete(psd_layer: *mut psd_layer);

    pub fn psd_layer_set_image(
        layer: *mut psd_layer,
        buffer: *const c_uchar,
        row_count: c_uint,
        column_count: c_uint,
    ) -> psd_error;
    pub fn psd_layer_set_image_from_file(layer: *mut psd_layer, path: *const c_char) -> psd_error;

    pub fn psd_layer_set_offset(layer: *mut psd_layer, x_offset: c_uint, y_offset: c_uint);
    pub fn psd_layer_get_top(layer: *const psd_layer) -> c_uint;
    pub fn psd_layer_get_left(layer: *const psd_layer) -> c_uint;
    pub fn psd_layer_get_bottom(layer: *const psd_layer) -> c_uint;
    pub fn psd_layer_get_right(layer: *const psd_layer) -> c_uint;

    pub fn psd_layer_get_name(layer: *const psd_layer) -> *const c_char;
    pub fn psd_layer_set_name(layer: *mut psd_layer, name: *const c_char);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};

    #[test]
    fn test_psd_layer_new_and_delete() {
        let name = CString::new("Layer").unwrap();
        unsafe {
            let layer = psd_layer_new(name.as_ptr());
            assert!(!layer.is_null());
            psd_layer_delete(layer);
        }
    }
    #[test]
    fn test_psd_layer_copy() {
        let name = CString::new("Layer").unwrap();
        unsafe {
            let layer = psd_layer_new(name.as_ptr());
            assert!(!layer.is_null());
            let copy = psd_layer_copy(layer);
            assert!(!copy.is_null());
            psd_layer_delete(layer);
            psd_layer_delete(copy);
        }
    }
    #[test]
    fn test_psd_layer_set_name_and_get_name() {
        let original_name = CString::new("Original").unwrap();
        let new_name = CString::new("NewName").unwrap();
        unsafe {
            let layer = psd_layer_new(original_name.as_ptr());
            assert!(!layer.is_null());

            psd_layer_set_name(layer, new_name.as_ptr());

            let ret_name_ptr = psd_layer_get_name(layer);
            assert!(!ret_name_ptr.is_null());

            let ret_name_cstr = std::ffi::CStr::from_ptr(ret_name_ptr);
            let ret_name_str = ret_name_cstr.to_str().unwrap();

            assert_eq!(ret_name_str, "NewName");
            psd_layer_delete(layer);
        }
    }
    #[test]
    fn test_psd_layer_set_offset_and_get() {
        let name = CString::new("OffsetLayer").unwrap();
        unsafe {
            let layer = psd_layer_new(name.as_ptr());
            assert!(!layer.is_null());

            psd_layer_set_offset(layer, 10, 20);

            let left = psd_layer_get_left(layer);
            let top = psd_layer_get_top(layer);

            assert!(left >= 10);
            assert!(top >= 20);

            psd_layer_delete(layer);
        }
    }
    fn assert_error(err: psd_error) {
        assert_eq!(
            err.status,
            0,
            "psd_error status is not 0, message: {}",
            unsafe {
                if err.message.is_null() {
                    "<null>"
                } else {
                    CStr::from_ptr(err.message)
                        .to_str()
                        .unwrap_or("<invalid utf8>")
                }
            }
        );
    }
    #[test]
    fn test_psd_layer_set_image() {
        let name = CString::new("Layer").unwrap();
        let image_data: [u8; 16] = [
            0x80, 0x00, 0x00, 0xff, 0x80, 0x00, 0x00, 0xff, 0x80, 0x00, 0x00, 0xff, 0x80, 0x00,
            0x00, 0xff,
        ];
        unsafe {
            let layer = psd_layer_new(name.as_ptr());
            assert!(!layer.is_null());
            let err = psd_layer_set_image(layer, image_data.as_ptr(), 2, 2);
            assert_error(err);

            psd_layer_delete(layer);
        }
    }
}
