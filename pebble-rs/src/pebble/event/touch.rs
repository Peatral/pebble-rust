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
use core::ffi::c_void;

/// An idiomatic Rust enum representing a touch event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchEvent {
    Touchdown { x: i16, y: i16 },
    PositionUpdate { x: i16, y: i16 },
    Liftoff { x: i16, y: i16 },
}

impl TouchEvent {
    /// Safely converts the raw C struct into our Rust enum.
    unsafe fn from_raw(raw: &pebble_sys::TouchEvent) -> Option<Self> {
        match raw.type_() {
            pebble_sys::TouchEventType::TouchEvent_Touchdown => {
                Some(Self::Touchdown { x: raw.x, y: raw.y })
            }
            pebble_sys::TouchEventType::TouchEvent_PositionUpdate => {
                Some(Self::PositionUpdate { x: raw.x, y: raw.y })
            }
            pebble_sys::TouchEventType::TouchEvent_Liftoff => {
                Some(Self::Liftoff { x: raw.x, y: raw.y })
            }
        }
    }
}

static TOUCH_HANDLER: GlobalCell<Option<fn(TouchEvent)>> = GlobalCell::new(None);

/// The C trampoline matching `TouchServiceHandler`
extern "C" fn touch_trampoline(event_ptr: *const pebble_sys::TouchEvent, _context: *mut c_void) {
    unsafe {
        if let Some(raw_event) = event_ptr.as_ref() {
            if let Some(rust_event) = TouchEvent::from_raw(raw_event) {
                if let Some(handler) = TOUCH_HANDLER.get() {
                    handler(rust_event);
                }
            }
        }
    }
}

/// Returns true if touch input is currently available and enabled.
pub fn is_enabled() -> bool {
    unsafe { pebble_sys::touch_service_is_enabled() }
}

/// Subscribes to touch events, powering on the sensor.
pub fn subscribe(handler: fn(TouchEvent)) {
    TOUCH_HANDLER.set(Some(handler));
    unsafe {
        pebble_sys::touch_service_subscribe(Some(touch_trampoline), core::ptr::null_mut());
    }
}

/// Unsubscribes from touch events, allowing the sensor to power down.
pub fn unsubscribe() {
    unsafe {
        pebble_sys::touch_service_unsubscribe();
    }
    TOUCH_HANDLER.set(None);
}
