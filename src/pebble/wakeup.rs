use crate::pebble::internal::functions::interface;
use crate::pebble::internal::types::{time_t, StatusCode, WakeupId};

// Since Pebble lacks a context pointer for Wakeups, we use a static variable
// to hold the user's Rust callback. This is safe because Pebble is strictly single-threaded.
static mut GLOBAL_WAKEUP_HANDLER: Option<fn(WakeupId, i32)> = None;

/// The C trampoline that routes the event to the user's Rust function.
extern "C" fn wakeup_trampoline(wakeup_id: WakeupId, cookie: i32) {
    unsafe {
        if let Some(handler) = GLOBAL_WAKEUP_HANDLER {
            handler(wakeup_id, cookie);
        }
    }
}

/// Registers a callback to be called when wakeup events occur while the app is running.
pub fn subscribe(handler: fn(WakeupId, i32)) {
    unsafe {
        GLOBAL_WAKEUP_HANDLER = Some(handler);
    }
    interface::wakeup_service_subscribe(wakeup_trampoline);
}

/// Registers a wakeup event that triggers at the specified time.
/// Returns the WakeupId on success, or a StatusCode error on failure.
pub fn schedule(
    timestamp: time_t,
    cookie: i32,
    notify_if_missed: bool,
) -> Result<WakeupId, StatusCode> {
    let result = interface::wakeup_schedule(timestamp, cookie, notify_if_missed);

    // The Pebble API returns negative values for errors
    if result < 0 {
        Err(StatusCode::from(result))
    } else {
        Ok(result)
    }
}

/// Cancels a specific scheduled wakeup event.
pub fn cancel(wakeup_id: WakeupId) {
    interface::wakeup_cancel(wakeup_id);
}

/// Cancels all wakeup events for the app.
pub fn cancel_all() {
    interface::wakeup_cancel_all();
}

/// Retrieves the wakeup event info if the app was launched by a wakeup_event.
/// Returns `Some((WakeupId, cookie))` if launched by a wakeup, or `None` otherwise.
pub fn get_launch_event() -> Option<(WakeupId, i32)> {
    let mut wakeup_id: WakeupId = 0;
    let mut cookie: i32 = 0;

    let was_wakeup = interface::wakeup_get_launch_event(&mut wakeup_id, &mut cookie);

    if was_wakeup {
        Some((wakeup_id, cookie))
    } else {
        None
    }
}

/// Checks if a WakeupId is still scheduled.
/// Returns `Some(time_t)` with the scheduled time if valid, or `None` if it has occurred or was canceled.
pub fn query(wakeup_id: WakeupId) -> Option<time_t> {
    let mut timestamp: time_t = 0;

    let is_scheduled = interface::wakeup_query(wakeup_id, &mut timestamp);

    if is_scheduled {
        Some(timestamp)
    } else {
        None
    }
}
