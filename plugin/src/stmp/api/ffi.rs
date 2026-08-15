use std::ffi::{CStr, c_char, c_int};

use ffi::str::CStrRef;
use mpv::api::MpvHandle;

#[link(name = "stmp", kind = "static")]
unsafe extern "C" {
  pub fn stmp_register_protocol(handle: MpvHandle) -> c_int;
}

pub type StmpMetadataStremPickerCb = fn(key: CStrRef, value: CStrRef) -> bool;

pub struct STMPCallbacks {
  pub metadata_stream_picker: Option<StmpMetadataStremPickerCb>,
}

static mut STMP_CALLBACKS: STMPCallbacks = STMPCallbacks {
  metadata_stream_picker: None,
};

pub fn stmp_register_metadata_stream_picker(cb: StmpMetadataStremPickerCb) {
  unsafe { STMP_CALLBACKS.metadata_stream_picker = Some(cb) }
}

#[unsafe(no_mangle)]
pub extern "C" fn stmp_metadata_stream_picker(key: *const c_char, value: *const c_char) -> c_int {
  println!(
    "{}={}",
    unsafe { CStr::from_ptr(key) }.to_str().unwrap(),
    unsafe { CStr::from_ptr(value) }.to_str().unwrap()
  );
  if let Some(cb) = unsafe { STMP_CALLBACKS.metadata_stream_picker } {
    i32::from(cb(CStrRef::from(key), CStrRef::from(value)))
  } else {
    0
  }
}