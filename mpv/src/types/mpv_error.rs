use std::fmt::{self, Display};

enum_from_primitive! {
#[repr(C)]
#[derive(PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum MpvError {
  /**
   * No error happened (used to signal successful operation).
   * Keep in mind that many API functions returning error codes can also
   * return positive values, which also indicate success. API users can
   * hardcode the fact that ">= 0" means success.
   */
  MpvErrorSuccess = 0,
  /**
   * The event ringbuffer is full. This means the client is choked, and can't
   * receive any events. This can happen when too many asynchronous requests
   * have been made, but not answered. Probably never happens in practice,
   * unless the mpv core is frozen for some reason, and the client keeps
   * making asynchronous requests. (Bugs in the client API implementation
   * could also trigger this, e.g. if events become "lost".)
   */
  MpvErrorEventQueueFull = -1,
  /**
   * Memory allocation failed.
   */
  MpvErrorNomem = -2,
  /**
   * The mpv core wasn't configured and initialized yet. See the notes in
   * mpv_create().
   */
  MpvErrorUninitialized = -3,
  /**
   * Generic catch-all error if a parameter is set to an invalid or
   * unsupported value. This is used if there is no better error code.
   */
  MpvErrorInvalidParameter = -4,
  /**
   * Trying to set an option that doesn't exist.
   */
  MpvErrorOptionNotFound = -5,
  /**
   * Trying to set an option using an unsupported MPV_FORMAT.
   */
  MpvErrorOptionFormat = -6,
  /**
   * Setting the option failed. Typically this happens if the provided option
   * value could not be parsed.
   */
  MpvErrorOptionError = -7,
  /**
   * The accessed property doesn't exist.
   */
  MpvErrorPropertyNotFound = -8,
  /**
   * Trying to set or get a property using an unsupported MPV_FORMAT.
   */
  MpvErrorPropertyFormat = -9,
  /**
   * The property exists, but is not available. This usually happens when the
   * associated subsystem is not active, e.g. querying audio parameters while
   * audio is disabled.
   */
  MpvErrorPropertyUnavailable = -10,
  /**
   * Error setting or getting a property.
   */
  MpvErrorPropertyError = -11,
  /**
   * General error when running a command with mpv_command and similar.
   */
  MpvErrorCommand = -12,
  /**
   * Generic error on loading (usually used with mpv_event_end_file.error).
   */
  MpvErrorLoadingFailed = -13,
  /**
   * Initializing the audio output failed.
   */
  MpvErrorAoInitFailed = -14,
  /**
   * Initializing the video output failed.
   */
  MpvErrorVoInitFailed = -15,
  /**
   * There was no audio or video data to play. This also happens if the
   * file was recognized, but did not contain any audio or video streams,
   * or no streams were selected.
   */
  MpvErrorNothingToPlay = -16,
  /**
   * When trying to load the file, the file format could not be determined,
   * or the file was too broken to open it.
   */
  MpvErrorUnknownFormat = -17,
  /**
   * Generic error for signaling that certain system requirements are not
   * fulfilled.
   */
  MpvErrorUnsupported = -18,
  /**
   * The API function which was called is a stub only.
   */
  MpvErrorNotImplemented = -19,
  /**
   * Unspecified error.
   */
  MpvErrorGeneric = -20,
}
}

impl Display for MpvError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(match self {
      MpvError::MpvErrorSuccess => "MPV_ERROR_SUCCESS",
      MpvError::MpvErrorEventQueueFull => "MPV_ERROR_EVENT_QUEUE_FULL",
      MpvError::MpvErrorNomem => "MPV_ERROR_NOMEM",
      MpvError::MpvErrorUninitialized => "MPV_ERROR_UNINITIALIZED",
      MpvError::MpvErrorInvalidParameter => "MPV_ERROR_INVALID_PARAMETER",
      MpvError::MpvErrorOptionNotFound => "MPV_ERROR_OPTION_NOT_FOUND",
      MpvError::MpvErrorOptionFormat => "MPV_ERROR_OPTION_FORMAT",
      MpvError::MpvErrorOptionError => "MPV_ERROR_OPTION_ERROR",
      MpvError::MpvErrorPropertyNotFound => "MPV_ERROR_PROPERTY_NOT_FOUND",
      MpvError::MpvErrorPropertyFormat => "MPV_ERROR_PROPERTY_FORMAT",
      MpvError::MpvErrorPropertyUnavailable => "MPV_ERROR_PROPERTY_UNAVAILABLE",
      MpvError::MpvErrorPropertyError => "MPV_ERROR_PROPERTY_ERROR",
      MpvError::MpvErrorCommand => "MPV_ERROR_COMMAND",
      MpvError::MpvErrorLoadingFailed => "MPV_ERROR_LOADING_FAILED",
      MpvError::MpvErrorAoInitFailed => "MPV_ERROR_AO_INIT_FAILED",
      MpvError::MpvErrorVoInitFailed => "MPV_ERROR_VO_INIT_FAILED",
      MpvError::MpvErrorNothingToPlay => "MPV_ERROR_NOTHING_TO_PLAY",
      MpvError::MpvErrorUnknownFormat => "MPV_ERROR_UNKNOWN_FORMAT",
      MpvError::MpvErrorUnsupported => "MPV_ERROR_UNSUPPORTED",
      MpvError::MpvErrorNotImplemented => "MPV_ERROR_NOT_IMPLEMENTED",
      MpvError::MpvErrorGeneric => "MPV_ERROR_GENERIC",
    })
  }
}