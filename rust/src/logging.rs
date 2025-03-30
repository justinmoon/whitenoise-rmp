//! Platform-specific logging setup for the counter app
//!
//! This module provides platform-specific logging initialization for different
//! platforms (iOS/macOS, Android, and others)

use log::LevelFilter;

/// Initialize logging based on the current platform.
/// This function should be called early in your app lifecycle.
pub fn init_logging() {
    #[cfg(target_os = "android")]
    {
        android_logger::init_once(
            android_logger::Config::default()
                .with_min_level(log::Level::Info)
                .with_tag("counter-rs"),
        );
        log::info!("Android logging initialized");
    }

    #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "macos")))]
    {
        // Default to env_logger for other platforms
        env_logger::init();
        log::info!("Default logging initialized");
    }
}
