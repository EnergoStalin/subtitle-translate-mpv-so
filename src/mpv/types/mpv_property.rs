use std::os::raw::{c_char, c_void};

use crate::mpv::{
  api::mpv_str::MpvStr,
  types::{FromRawMpv, MpvFormat},
};

#[repr(C)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy)]
pub struct MpvPropertyData {
  pub name: *const c_char,
  pub format: MpvFormat,
  pub data: *mut c_void,
}

impl MpvPropertyData {
  pub fn from_ptr<'a>(value: *const c_void) -> &'a Self {
    unsafe { &*(value as *const MpvPropertyData) }
  }
}

impl From<*const c_void> for MpvPropertyData {
  fn from(value: *const c_void) -> Self {
    unsafe { *(value as *const MpvPropertyData) }
  }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct MpvValue<'a> {
  pub value: *mut c_void,
  pub format: MpvFormat,
  _marker: std::marker::PhantomData<&'a c_void>,
}

impl<'a> MpvValue<'a> {
  pub fn new(format: MpvFormat, value: *mut c_void) -> Self {
    Self {
      value,
      format,
      _marker: std::marker::PhantomData,
    }
  }

  pub fn as_type<T: FromRawMpv>(&self) -> T {
    T::from_raw(self.value)
  }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct MpvProperty<'a> {
  pub name: MpvStr<'a>,
  pub data: MpvValue<'a>,
}

impl<'a> MpvProperty<'a> {
  pub fn from_ptr(value: *const c_void) -> Self {
    MpvProperty::from_data(MpvPropertyData::from_ptr(value))
  }

  pub fn from_data(value: &MpvPropertyData) -> Self {
    Self {
      name: MpvStr::from(value.name),
      data: MpvValue::new(value.format, value.data),
    }
  }
}

impl<'a> std::fmt::Display for MpvProperty<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let namep = self.name.as_ptr() as usize;
    write!(f, "\n[Rust] Namep: 0x{:012x}\n", namep)?;
    f.write_str("[Rust] Name: ")?;
    if namep > 0x00000ffff {
      f.write_str(self.name.as_str())?;
    } else {
      f.write_str("Not Pointer")?;
    }
    f.write_str("\n[Rust] Format: ")?;
    self.data.format.fmt(f)?;
    f.write_str("\n[Rust] Data: ")?;
    if self.data.value.is_null() || self.data.format == MpvFormat::MpvFormatNone {
      f.write_str("NULL")
    } else {
      write!(f, "0x{:012x}", self.data.value as usize)
    }
  }
}

}