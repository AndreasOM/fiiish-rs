#[cfg(target_os = "macos")]
mod music_apple;
#[cfg(target_os = "macos")]
pub use music_apple::MusicApple as Music;

#[cfg(target_os = "windows")]
mod music_stub;
#[cfg(target_os = "windows")]
pub use music_stub::MusicStub as Music;

#[cfg(target_os = "linux")]
mod music_stub;
#[cfg(target_os = "linux")]
pub use music_stub::MusicStub as Music;
