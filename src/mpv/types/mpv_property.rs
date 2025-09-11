use std::os::raw::{c_char, c_void};

use crate::mpv::types::MpvFormat;

#[repr(C)]
#[derive(Debug)]
pub struct MpvProperty {
  name: *const c_char,
  format: MpvFormat,
  data: *const c_void
}