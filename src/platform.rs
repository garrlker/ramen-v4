#[cfg_attr(feature = "nightly-docs", doc(cfg(target_os = "windows")))]
#[cfg_attr(not(feature = "nightly-docs"), cfg(target_os = "windows"))]
pub mod win32;

#[cfg(target_os = "windows")]
pub(crate) use win32 as imp;
