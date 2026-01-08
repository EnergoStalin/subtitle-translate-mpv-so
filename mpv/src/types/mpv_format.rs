use std::ffi::c_void;

enum_from_primitive! {
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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
