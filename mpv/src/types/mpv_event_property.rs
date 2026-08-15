use std::ffi::{c_int, c_void};

use ffi::str::CStrRef;

#[repr(C)]
#[derive(Debug)]
pub struct MpvEventProperty<'a> {
  pub name: CStrRef<'a>,
  pub format: c_int,
  pub data: *const c_void,
}