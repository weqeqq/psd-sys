#![allow(non_camel_case_types)]

use std::ffi::{c_char, c_int};

#[repr(C)]
pub struct psd_error {
    pub status: c_int,
    pub message: *const c_char,
}
