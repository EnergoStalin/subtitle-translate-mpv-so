use std::ffi::{c_int, c_void};

use crate::api::mpv_str::MpvStr;

#[repr(C)]
#[derive(Debug)]
pub struct MpvEventProperty<'a> {
  pub name: MpvStr<'a>,
  pub format: c_int,
  pub data: *const c_void,
}
