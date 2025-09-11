use std::{os::raw::{c_char, c_void}};

use crate::mpv::{
  api::mpv_str::MpvStr,
  types::{FromRawMpv, MpvFormat},
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MpvEventPropertyData {
  pub name: *const c_char,
  pub format: MpvFormat,
  pub data: *mut c_void,
}

impl MpvEventPropertyData {
  pub fn from_ptr<'a>(value: *const c_void) -> &'a Self {
    unsafe { &*(value as *const MpvEventPropertyData) }
  }
}

impl From<*const c_void> for MpvEventPropertyData {
  fn from(value: *const c_void) -> Self {
    unsafe { *(value as *const MpvEventPropertyData) }
  }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct MpvValue<'a> {
  pub value: *const c_void,
  pub format: MpvFormat,
  _marker: std::marker::PhantomData<&'a c_void>,
}

impl<'a> MpvValue<'a> {
  pub fn new(format: MpvFormat, value: *const c_void) -> Self {
    Self {
      value,
      format,
      _marker: std::marker::PhantomData,
    }
  }

  pub fn as_type<T: FromRawMpv<'a>>(&self) -> &'a T {
    T::from_raw(self.value)
  }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct MpvEventProperty<'a> {
  pub name: MpvStr<'a>,
  pub data: MpvValue<'a>,
}

impl<'a> MpvEventProperty<'a> {
  pub fn from_ptr(value: *const c_void) -> Self {
    MpvEventProperty::from_data(MpvEventPropertyData::from_ptr(value))
  }

  pub fn from_data(value: &MpvEventPropertyData) -> Self {
    Self {
      name: MpvStr::from(value.name),
      data: MpvValue::new(value.format, value.data),
    }
  }
}

impl<'a> std::fmt::Display for MpvEventProperty<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("Name: ")?;
    println!("{:x}", self.name.as_ptr() as usize);
    f.write_str(self.name.as_str())?;
    f.write_str("\nFormat: ")?;
    self.data.format.fmt(f)?;
    f.write_str("\nData: ")?;
    if self.data.value.is_null() {
      f.write_str("NULL")
    } else {
      write!(f, "{:x}", self.data.value as usize)
    }
  }
}