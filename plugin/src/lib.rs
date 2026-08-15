use ffi::str::CStrRef;
use mpv::api::{MpvHandle, MpvPlugin};

mod stmp;

use crate::stmp::api::STMP;

fn picker(key: CStrRef, value: CStrRef) -> bool {
  println!("{}={}", key, value);
  true
}

#[unsafe(no_mangle)]
pub extern "C" fn mpv_open_cplugin(handle: MpvHandle) -> std::ffi::c_int {
  let p = MpvPlugin::new(handle);
  let stmp = STMP::new(handle);

  stmp.register_metadata_stream_picker(picker);
  stmp.register();

  loop {
    let event = p.wait_event(2000.0);

    match event.event_id {
      mpv::types::MpvEventId::FileLoaded => {
        println!("Path: {}", p.get_property_string("path"));
        println!("{}", p.command("sub-add stmp:///home/alexv/.config/mpv/dev/subtitle-translate-mpv-so/test.ass auto select"));
      }
      mpv::types::MpvEventId::PropertyChange => {
        println!("Subtitles: {}", p.get_property_string("sub-text/ass"));
      }
      mpv::types::MpvEventId::Shutdown => return 0,
      mpv::types::MpvEventId::Unknown => {}
    }
  }
}
