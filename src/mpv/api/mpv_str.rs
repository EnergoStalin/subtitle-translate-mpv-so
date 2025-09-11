use std::{
  ffi::{CStr, c_char},
  fmt::Display,
};

use crate::mpv::ffi::mpv_free;

pub struct MpvStr {
  ptr: *const c_char,
}

impl MpvStr {
  pub fn as_str(&self) -> &str {
    unsafe { std::ffi::CStr::from_ptr(self.ptr).to_str().unwrap() }
  }
}

impl Display for MpvStr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_str())
  }
}

impl Drop for MpvStr {
  fn drop(&mut self) {
    unsafe {
      mpv_free(self.ptr as *const _);
    }
  }
}

impl From<*mut c_char> for MpvStr {
  fn from(ptr: *mut c_char) -> Self {
    MpvStr { ptr }
  }
}

impl Into<String> for MpvStr {
  fn into(self) -> String {
    unsafe { CStr::from_ptr(self.ptr) }
      .to_string_lossy()
      .into_owned()
  }
}
