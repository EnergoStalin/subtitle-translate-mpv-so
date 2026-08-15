use std::{
  ffi::{CStr, c_char, c_void},
  fmt::Display,
};

use crate::ffi::mpv_free;

#[repr(transparent)]
#[derive(Debug)]
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
