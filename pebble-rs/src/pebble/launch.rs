use pebble_sys::AppLaunchReason;

/// Provides the method used to launch the current application.
pub fn get_reason() -> AppLaunchReason {
    AppLaunchReason::from(unsafe { pebble_sys::launch_reason() })
}

/// Gets the argument passed to the app when it was launched.
///
/// This is typically used when an application is opened via a timeline pin action.
/// Returns `0` if the app was not launched from a Launch App action.
pub fn get_args() -> u32 {
    unsafe { pebble_sys::launch_get_args() }
}
