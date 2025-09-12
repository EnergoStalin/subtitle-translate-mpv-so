use std::fmt;
use std::os::raw::c_void;

use crate::mpv::api::mpv_str::MpvStr;

#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum MpvFormat {
  MpvFormatNone,
  MpvFormatString,
  MpvFormatOsdString,
  MpvFormatFlag,
  MpvFormatInt64,
  MpvFormatDouble,
  MpvFormatNode,
  MpvFormatNodeArray,
  MpvFormatNodeMap,
  MpvFormatByteArray,
}

impl fmt::Display for MpvFormat {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let n = self as *const _ as u32;
    let s = match if n > MpvFormat::MpvFormatByteArray as u32 { &MpvFormat::MpvFormatNone } else { self } {
      MpvFormat::MpvFormatNone => "MPV_FORMAT_NONE",
      MpvFormat::MpvFormatString => "MPV_FORMAT_STRING",
      MpvFormat::MpvFormatOsdString => "MPV_FORMAT_OSD_STRING",
      MpvFormat::MpvFormatFlag => "MPV_FORMAT_FLAG",
      MpvFormat::MpvFormatInt64 => "MPV_FORMAT_INT64",
      MpvFormat::MpvFormatDouble => "MPV_FORMAT_DOUBLE",
      MpvFormat::MpvFormatNode => "MPV_FORMAT_NODE",
      MpvFormat::MpvFormatNodeArray => "MPV_FORMAT_NODE_ARRAY",
      MpvFormat::MpvFormatNodeMap => "MPV_FORMAT_NODE_MAP",
      MpvFormat::MpvFormatByteArray => "MPV_FORMAT_BYTE_ARRAY",
    };
    f.write_str(s)
  }
}

pub trait ToRawMpv: Sized {
  const FORMAT: MpvFormat;

  fn to_raw(&self) -> *mut c_void;
}

impl ToRawMpv for i64 {
  const FORMAT: MpvFormat = MpvFormat::MpvFormatInt64;

  fn to_raw(&self) -> *mut c_void {
    self as *const i64 as *mut c_void
  }
}

impl ToRawMpv for f64 {
  const FORMAT: MpvFormat = MpvFormat::MpvFormatDouble;

  fn to_raw(&self) -> *mut c_void {
    self as *const f64 as *mut c_void
  }
}

impl ToRawMpv for bool {
  const FORMAT: MpvFormat = MpvFormat::MpvFormatFlag;

  fn to_raw(&self) -> *mut c_void {
    self as *const bool as *mut c_void
  }
}

impl ToRawMpv for *mut c_void {
  const FORMAT: MpvFormat = MpvFormat::MpvFormatNone;

  fn to_raw(&self) -> *mut c_void {
    *self
  }
}

pub trait FromRawMpv {
  fn from_raw(ptr: *mut c_void) -> Self;
}

impl FromRawMpv for i64 {
  fn from_raw(ptr: *mut c_void) -> Self {
    unsafe { *(ptr as *mut Self) }
  }
}

impl FromRawMpv for f64 {
  fn from_raw(ptr: *mut c_void) -> Self {
    unsafe { *(ptr as *mut Self) }
  }
}

impl<'a> FromRawMpv for MpvStr<'a> {
  fn from_raw(ptr: *mut c_void) -> Self {
    Self::from(ptr)
  }
}