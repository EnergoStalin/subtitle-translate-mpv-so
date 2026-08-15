use mpv::api::MpvHandle;

mod ffi;


pub struct STMP {
  handle: MpvHandle,
}

impl STMP {
  pub fn new(handle: MpvHandle) -> Self {
    Self { handle }
  }

  pub fn register(&self) {
    unsafe {
      ffi::stmp_register_protocol(self.handle);
    };
  }

  pub fn register_metadata_stream_picker(&self, cb: ffi::StmpMetadataStremPickerCb) {
    ffi::stmp_register_metadata_stream_picker(cb);
  }
}