use std::{ffi::{CStr, CString, c_char, c_void}, fmt::Display, marker::PhantomData};

#[repr(transparent)]
#[derive(Debug)]
pub struct CStrRef<'a>(*const c_char, PhantomData<&'a c_char>);

impl<'a> std::ops::Deref for CStrRef<'a> {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    self.as_str()
  }
}

impl<'a> CStrRef<'a> {
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

impl<'a> From<*const c_char> for CStrRef<'a> {
  fn from(value: *const c_char) -> Self {
    Self(value, Default::default())
  }
}

impl<'a> From<*const c_void> for CStrRef<'a> {
  fn from(value: *const c_void) -> Self {
    Self::try_from_void(value).unwrap()
  }
}

impl<'a> From<String> for CStrRef<'a> {
  fn from(value: String) -> Self {
    Self(CString::new(value).unwrap().as_ptr(), Default::default())
  }
}

impl<'a> PartialEq<&str> for CStrRef<'a> {
  fn eq(&self, other: &&str) -> bool {
    self.as_str() == *other
  }
}

impl<'a> Display for CStrRef<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_str())
  }
}