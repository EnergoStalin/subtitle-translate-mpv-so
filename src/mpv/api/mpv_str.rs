use std::{
  ffi::CStr,
  fmt::Display,
  os::raw::{c_char, c_void},
};

#[cfg_attr(debug_assertions, derive(Debug))]
struct MpvStrInner {
  ptr: *const c_char,
}

impl MpvStrInner {
  pub fn as_str(&self) -> &str {
    unsafe { if self.ptr.is_null() { "NULL" } else { std::ffi::CStr::from_ptr(self.ptr).to_str().unwrap() } }
  }

  pub fn as_ptr(&self) -> *mut c_void {
    self.ptr as *mut c_void
  }
}

impl Into<*const c_char> for MpvStrInner {
  fn into(self) -> *const c_char {
    self.as_ptr() as *const c_char
  }
}

impl Into<*const c_void> for MpvStrInner {
  fn into(self) -> *const c_void {
    self.as_ptr()
  }
}

impl PartialEq<&str> for MpvStrInner {
  fn eq(&self, other: &&str) -> bool {
    self.as_str() == *other
  }
}

impl Display for MpvStrInner {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_str())
  }
}

impl From<*const c_char> for MpvStrInner {
  fn from(ptr: *const c_char) -> Self {
    Self {
      ptr: if (ptr as usize) == 1 {
        std::ptr::null()
      } else {
        ptr
      },
    }
  }
}

impl From<*mut c_void> for MpvStrInner {
  fn from(ptr: *mut c_void) -> Self {
    Self {
      ptr: (ptr as *const c_char),
    }
  }
}

impl Into<String> for MpvStrInner {
  fn into(self) -> String {
    unsafe { CStr::from_ptr(self.ptr) }
      .to_string_lossy()
      .into_owned()
  }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct MpvStr<'a> {
  inner: MpvStrInner,
  _marker: std::marker::PhantomData<&'a c_char>,
}

impl<'a> MpvStr<'a> {
  pub fn as_str(&'a self) -> &'a str {
    self.inner.as_str()
  }
  pub fn as_ptr(&'a self) -> *mut c_void {
    self.inner.as_ptr()
  }
}

impl<'a> Display for MpvStr<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.inner.fmt(f)
  }
}

impl<'a> From<*const c_char> for MpvStr<'a> {
  fn from(ptr: *const c_char) -> Self {
    Self {
      inner: ptr.into(),
      _marker: std::marker::PhantomData,
    }
  }
}

impl<'a> From<*mut c_void> for MpvStr<'a> {
  fn from(ptr: *mut c_void) -> Self {
    Self {
      inner: ptr.into(),
      _marker: std::marker::PhantomData,
    }
  }
}

impl<'a> Into<String> for MpvStr<'a> {
  fn into(self) -> String {
    self.inner.into()
  }
}

impl<'a> Into<*const c_char> for MpvStr<'a> {
  fn into(self) -> *const c_char {
    self.inner.into()
  }
}

impl<'a> Into<*const c_void> for MpvStr<'a> {
  fn into(self) -> *const c_void {
    self.inner.into()
  }
}

impl<'a> PartialEq<&str> for &MpvStr<'a> {
  fn eq(&self, other: &&str) -> bool {
    self.inner.eq(other)
  }
}