use crate::enum_primitive::FromPrimitive;

use std::ffi::CString;
use std::os::raw::c_void;

pub mod mpv_str;

use crate::api::mpv_str::MpvStrOwned;
use crate::ffi::{mpv_get_property_string, mpv_observe_property, mpv_wait_event};
use crate::types::{MpvError, MpvEvent, MpvFormat};

pub type MpvHandle = *const c_void;

pub struct MpvPlugin(MpvHandle);

impl MpvPlugin {
  pub fn new(handle: MpvHandle) -> Self {
    Self { 0: handle }
  }

  pub fn wait_event(&self, timeout: f64) -> &'static MpvEvent {
    unsafe { mpv_wait_event(self.0, timeout).as_ref().unwrap() }
  }

  pub fn get_property_string(&self, name: &str) -> MpvStrOwned {
    unsafe { mpv_get_property_string(self.0, CString::new(name).unwrap().as_ptr()) }
  }

  pub fn observe_property_string(&self, reply_userdata: u64, name: &str) -> MpvError {
    self.observe_property(reply_userdata, name, MpvFormat::MpvFormatString)
  }

  pub fn observe_property(&self, reply_userdata: u64, name: &str, format: MpvFormat) -> MpvError {
    unsafe {
      MpvError::from_i32(mpv_observe_property(
        self.0,
        reply_userdata,
        CString::new(name).unwrap().as_ptr(),
        format,
      ))
      .unwrap_or(MpvError::MpvErrorGeneric)
    }
  }
}
