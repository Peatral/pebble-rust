/*
 * This file is part of pebble-rs.
 * Copyright (c) 2026 Peatral
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */
use crate::types::GlobalCell;
use pebble_sys::{StatusCode, WakeupId, time_t};

static GLOBAL_WAKEUP_HANDLER: GlobalCell<Option<fn(WakeupId, i32)>> = GlobalCell::new(None);

/// The C trampoline that routes the event to the user's Rust function.
extern "C" fn wakeup_trampoline(wakeup_id: WakeupId, cookie: i32) {
    if let Some(handler) = GLOBAL_WAKEUP_HANDLER.get() {
        handler(wakeup_id, cookie);
    }
}

/// Registers a callback to be called when wakeup events occur while the app is running.
pub fn subscribe(handler: fn(WakeupId, i32)) {
    GLOBAL_WAKEUP_HANDLER.set(Some(handler));
    unsafe {
        pebble_sys::wakeup_service_subscribe(Some(wakeup_trampoline));
    }
}

/// Unsubscribes from the wakeup service, preventing the callback from firing.
pub fn unsubscribe() {
    unsafe {
        pebble_sys::wakeup_service_subscribe(None);
    }
    GLOBAL_WAKEUP_HANDLER.set(None);
}

/// Registers a wakeup event that triggers at the specified time.
/// Returns the WakeupId on success, or a StatusCode error on failure.
pub fn schedule(
    timestamp: time_t,
    cookie: i32,
    notify_if_missed: bool,
) -> Result<WakeupId, StatusCode> {
    unsafe {
        let result = pebble_sys::wakeup_schedule(timestamp, cookie, notify_if_missed);

        if result < 0 {
            Err(pebble_sys::StatusCode(result))
        } else {
            Ok(result)
        }
    }
}

/// Cancels a specific scheduled wakeup event.
pub fn cancel(wakeup_id: WakeupId) {
    unsafe {
        pebble_sys::wakeup_cancel(wakeup_id);
    }
}

/// Cancels all wakeup events for the app.
pub fn cancel_all() {
    unsafe {
        pebble_sys::wakeup_cancel_all();
    }
}

/// Retrieves the wakeup event info if the app was launched by a wakeup_event.
/// Returns `Some((WakeupId, cookie))` if launched by a wakeup, or `None` otherwise.
pub fn get_launch_event() -> Option<(WakeupId, i32)> {
    let mut wakeup_id: WakeupId = 0;
    let mut cookie: i32 = 0;

    unsafe {
        let was_wakeup = pebble_sys::wakeup_get_launch_event(&mut wakeup_id, &mut cookie);

        if was_wakeup {
            Some((wakeup_id, cookie))
        } else {
            None
        }
    }
}

/// Checks if a WakeupId is still scheduled.
/// Returns `Some(time_t)` with the scheduled time if valid, or `None` if it has occurred or was canceled.
pub fn query(wakeup_id: WakeupId) -> Option<time_t> {
    let mut timestamp: time_t = 0;

    unsafe {
        let is_scheduled = pebble_sys::wakeup_query(wakeup_id, &mut timestamp);

        if is_scheduled { Some(timestamp) } else { None }
    }
}
