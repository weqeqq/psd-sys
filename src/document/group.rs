#![allow(non_camel_case_types)]

use std::ffi::{c_char, c_int, c_void};

use crate::document::layer::*;
use crate::error::psd_error;

#[repr(C)]
pub struct psd_group(c_void);

unsafe extern "C" {
    pub fn psd_group_new(name: *const c_char) -> *mut psd_group;
    pub fn psd_group_clone(group: *const psd_group) -> *mut psd_group;
    pub fn psd_group_delete(group: *mut psd_group);

    pub fn psd_group_push_layer(group: *mut psd_group, layer: *mut psd_layer) -> psd_error;
    pub fn psd_group_push_group(group: *mut psd_group, group: *mut psd_group) -> psd_error;

    pub fn psd_group_get_name(group: *const psd_group) -> *const c_char;
    pub fn psd_group_set_name(group: *mut psd_group, name: *const c_char);

    pub fn psd_group_empty(group: *const psd_group) -> c_int;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};

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
    fn test_psd_group_new_and_delete() {
        unsafe {
            let group = psd_group_new(CString::new("Group").unwrap().as_ptr());
            assert!(!group.is_null());
            psd_group_delete(group);
        }
    }

    #[test]
    fn test_psd_group_copy() {
        unsafe {
            let group = psd_group_new(CString::new("Group").unwrap().as_ptr());
            assert!(!group.is_null());

            let copy = psd_group_clone(group);
            assert!(!copy.is_null());

            psd_group_delete(group);
            psd_group_delete(copy);
        }
    }

    #[test]
    fn test_psd_group_set_and_get_name() {
        unsafe {
            let group = psd_group_new(CString::new("Group 0").unwrap().as_ptr());
            assert!(!group.is_null());

            psd_group_set_name(group, CString::new("Group 1").unwrap().as_ptr());

            let ret_name_ptr = psd_group_get_name(group);
            assert!(!ret_name_ptr.is_null());

            let ret_name = CStr::from_ptr(ret_name_ptr).to_str().unwrap();
            assert_eq!(ret_name, "Group 1");

            psd_group_delete(group);
        }
    }

    #[test]
    fn test_psd_group_is_empty() {
        unsafe {
            let group = psd_group_new(CString::new("Group").unwrap().as_ptr());
            assert!(!group.is_null());

            let empty = psd_group_empty(group);
            assert_eq!(empty, 1);

            psd_group_delete(group);
        }
    }

    #[test]
    fn test_psd_group_push_layer_and_group() {
        unsafe {
            let parent_group = psd_group_new(CString::new("Group").unwrap().as_ptr());
            assert!(!parent_group.is_null());

            let child_group = psd_group_new(CString::new("ChildGroup").unwrap().as_ptr());
            assert!(!child_group.is_null());

            let layer = psd_layer_new(CString::new("Layer").unwrap().as_ptr());
            assert!(!layer.is_null());

            let err_layer = psd_group_push_layer(parent_group, layer);
            assert_error(err_layer);

            let err_group = psd_group_push_group(parent_group, child_group);
            assert_error(err_group);

            psd_group_delete(parent_group);
        }
    }
}
