#[cfg(target_os = "macos")]
mod sound_apple;
#[cfg(target_os = "macos")]
pub use sound_apple::SoundApple as Sound;

#[cfg(target_os = "windows")]
mod sound_stub;
#[cfg(target_os = "windows")]
pub use sound_stub::SoundStub as Sound;

#[cfg(target_os = "linux")]
mod sound_stub;
#[cfg(target_os = "linux")]
pub use sound_stub::SoundStub as Sound;
