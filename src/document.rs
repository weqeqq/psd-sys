#![allow(non_camel_case_types)]

pub mod group;
pub mod layer;

use crate::document::group::*;
use crate::document::layer::*;

use crate::error::psd_error;
use std::ffi::c_char;
use std::ffi::c_void;

#[repr(C)]
pub struct psd_document(c_void);

unsafe extern "C" {
    pub fn psd_document_new() -> *mut psd_document;
    pub fn psd_document_delete(document: *mut psd_document);
    pub fn psd_document_copy(document: *const psd_document) -> *mut psd_document;
    pub fn psd_document_save(document: *const psd_document, path: *const c_char) -> psd_error;
    pub fn psd_document_push_layer(document: *mut psd_document, layer: *mut psd_layer)
    -> psd_error;
    pub fn psd_document_push_group(document: *mut psd_document, group: *mut psd_group)
    -> psd_error;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        ffi::{CStr, CString},
        fs::remove_file,
    };

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
    fn test_psd_document_new_and_delete() {
        unsafe {
            let document = psd_document_new();
            assert!(!document.is_null());
            psd_document_delete(document);
        }
    }
    #[test]
    fn test_psd_document_copy() {
        unsafe {
            let document = psd_document_new();
            assert!(!document.is_null());
            let copy = psd_document_copy(document);
            assert!(!copy.is_null());
            psd_document_delete(document);
            psd_document_delete(copy);
        }
    }
    #[test]
    fn test_psd_document_save() {
        unsafe {
            let document = psd_document_new();
            assert!(!document.is_null());
            let layer = psd_layer_new(CString::new("Layer").unwrap().as_ptr());
            psd_document_push_layer(document, layer);
            let error = psd_document_save(document, CString::new("output.psd").unwrap().as_ptr());
            assert_error(error);
            psd_document_delete(document);
            remove_file("output.psd").unwrap();
        }
    }
}
