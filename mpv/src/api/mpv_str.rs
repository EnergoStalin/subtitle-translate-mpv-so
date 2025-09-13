use std::{
  ffi::CStr,
  fmt::Display,
  marker::PhantomData,
  os::raw::{c_char, c_void},
};

use crate::ffi::mpv_free;

#[repr(transparent)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct MpvStrOwned(*const c_char);

impl Display for MpvStrOwned {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_str().unwrap_or("NULL"))
  }
}

impl Drop for MpvStrOwned {
  fn drop(&mut self) {
    unsafe {
      if !self.as_ptr().is_null() {
        mpv_free(self.as_ptr() as *const c_void)
      }
    }
  }
}

impl PartialEq<&str> for MpvStrOwned {
  fn eq(&self, other: &&str) -> bool {
    self.as_str().unwrap_or_default() == *other
  }
}

impl MpvStrOwned {
  pub fn as_str(&self) -> Option<&str> {
    if self.as_ptr().is_null() {
      None
    } else {
      Some(unsafe { CStr::from_ptr(self.0).to_str().unwrap() })
    }
  }

  pub fn as_ptr(&self) -> *const c_char {
    self.0
  }
}

#[repr(transparent)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct MpvStr<'a>(*const c_char, PhantomData<&'a c_char>);

impl<'a> MpvStr<'a> {
  pub fn as_str(&self) -> &'a str {
    unsafe { CStr::from_ptr(self.0).to_str().unwrap() }
  }

  pub fn as_ptr(&self) -> *const c_char {
    self.0 as *const c_char
  }

  fn try_from(value: *const c_char) -> Option<Self> {
    if value.is_null() {
      None
    } else {
      Some(Self {
        0: value,
        1: PhantomData,
      })
    }
  }
}

impl<'a> Into<*const c_char> for MpvStr<'a> {
  fn into(self) -> *const c_char {
    self.as_ptr() as *const c_char
  }
}

impl<'a> Into<*const c_void> for MpvStr<'a> {
  fn into(self) -> *const c_void {
    self.as_ptr() as *const c_void
  }
}

impl<'a> Into<String> for MpvStr<'a> {
  fn into(self) -> String {
    unsafe { CStr::from_ptr(self.0) }
      .to_string_lossy()
      .into_owned()
  }
}

impl<'a> PartialEq<&str> for MpvStr<'a> {
  fn eq(&self, other: &&str) -> bool {
    self.as_str() == *other
  }
}

impl<'a> Display for MpvStr<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_str())
  }
}