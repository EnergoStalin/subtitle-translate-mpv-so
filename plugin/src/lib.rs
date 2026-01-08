use mpv::api::{MpvHandle, MpvPlugin};

#[unsafe(no_mangle)]
pub extern "C" fn mpv_open_cplugin(handle: MpvHandle) -> std::os::raw::c_int {
  let p = MpvPlugin::new(handle);
  p.observe_property_string(0, "sub-text/ass");

  loop {
    let event = p.wait_event(2000.0);

    match event.event_id {
      mpv::types::MpvEventId::FileLoaded => {
        println!("Path: {}", p.get_property_string("path"));
      }
      mpv::types::MpvEventId::PropertyChange => {
        println!("Subtitles: {}", p.get_property_string("sub-text/ass"));
      }
      mpv::types::MpvEventId::Shutdown => return 0,
      mpv::types::MpvEventId::Unknown => {}
    }
  }
}
