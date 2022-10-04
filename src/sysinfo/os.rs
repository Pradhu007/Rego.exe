//! You shouldn't need to edit this file
pub mod linux;
pub mod windows;

#[cfg(target_os = "linux")]
pub use linux as target;

#[cfg(target_family = "windows")]
pub(crate) use windows as target;
