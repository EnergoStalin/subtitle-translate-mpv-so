use mpv::api::{MpvHandle, MpvPlugin};

#[unsafe(no_mangle)]
pub extern "C" fn mpv_open_cplugin(handle: MpvHandle) -> std::os::raw::c_int {
  let plugin = MpvPlugin::new(handle);
  plugin.observe_property_string(0, "sub-text/ass");

  loop {
    let event = plugin.wait_event(2000.0);
    let eid = event.event_id;

    match eid {
      mpv::types::MpvEventId::FileLoaded => {
        println!("{}", plugin.get_property_string("path"));
        println!("{}", plugin.get_property_string("track-list"));
      }
      mpv::types::MpvEventId::Shutdown => return 0,
      mpv::types::MpvEventId::PropertyChange => { }
    }
  }
}