use std::{
  ffi::{c_char, c_void, CStr, CString},
  fmt::Display,
  marker::PhantomData,
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

impl From<*const c_char> for MpvStrOwned {
  fn from(value: *const c_char) -> Self {
    Self(value)
  }
}

impl From<*const c_void> for MpvStrOwned {
  fn from(value: *const c_void) -> Self {
    Self::from(value as *const c_char)
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

  pub fn try_from_void(value: *const c_void) -> Option<Self> {
    Self::try_from(value as *const c_char)
  }

  pub fn try_from(value: *const c_char) -> Option<Self> {
    if value.is_null() {
      None
    } else {
      Some(Self(value, PhantomData))
    }
  }
}

impl<'a> From<*const c_char> for MpvStr<'a> {
  fn from(value: *const c_char) -> Self {
    Self(value, Default::default())
  }
}

impl<'a> From<*const c_void> for MpvStr<'a> {
  fn from(value: *const c_void) -> Self {
    Self::try_from_void(value).unwrap()
  }
}

impl<'a> From<String> for MpvStr<'a> {
  fn from(value: String) -> Self {
    Self(CString::new(value).unwrap().as_ptr(), Default::default())
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
