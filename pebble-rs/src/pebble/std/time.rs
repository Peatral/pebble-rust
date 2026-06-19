/*
 * This file is part of pebble-rs.
 * Copyright (c) 2019 RoccoDev
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
use pebble_sys::{time_t, tm};

pub fn get_time() -> time_t {
    unsafe { pebble_sys::time(core::ptr::null_mut()) }
}

pub fn is_clock_24h() -> bool {
    unsafe { pebble_sys::clock_is_24h_style() }
}

pub fn get_local_time(now: time_t) -> tm {
    unsafe { *pebble_sys::localtime(&now) }
}

pub fn get_utc_time(now: time_t) -> tm {
    unsafe { *pebble_sys::gmtime(&now) }
}

pub fn start_of_today() -> time_t {
    unsafe { pebble_sys::time_start_of_today() }
}
