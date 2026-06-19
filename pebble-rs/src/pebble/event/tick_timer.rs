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
use pebble_sys::{TimeUnits, tm};

static USER_HANDLER: GlobalCell<Option<fn(&tm, TimeUnits)>> = GlobalCell::new(None);

extern "C" fn trampoline(tick_time: *mut tm, units_changed: TimeUnits) {
    if let Some(cb) = USER_HANDLER.get() {
        unsafe {
            if let Some(time_ref) = tick_time.as_ref() {
                cb(time_ref, units_changed);
            }
        }
    }
}

pub fn subscribe(tick_units: TimeUnits, handler: fn(&tm, TimeUnits)) {
    USER_HANDLER.set(Some(handler));
    unsafe {
        pebble_sys::tick_timer_service_subscribe(tick_units, Some(trampoline));
    }
}

pub fn unsubscribe() {
    unsafe {
        pebble_sys::tick_timer_service_unsubscribe();
    }
    USER_HANDLER.set(None);
}
