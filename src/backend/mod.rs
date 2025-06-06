// This module is not public

// TODO: improve feature selection (make sure that there is always exactly one implementation, or enable dynamic backend selection)
// TODO: allow to disable build dependency on ALSA

#[cfg(target_os = "windows")]
mod winrt;
#[cfg(target_os = "windows")]
pub use self::winrt::*;

#[cfg(target_os = "macos")]
mod coremidi;
#[cfg(target_os = "macos")]
pub use self::coremidi::*;

#[cfg(target_os = "ios")]
mod coremidi;
#[cfg(target_os = "ios")]
pub use self::coremidi::*;

#[cfg(target_os = "linux")]
mod alsa;
#[cfg(target_os = "linux")]
pub use self::alsa::*;

#[cfg(target_arch = "wasm32")]
mod webmidi;
#[cfg(target_arch = "wasm32")]
pub use self::webmidi::*;
