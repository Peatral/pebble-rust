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
use pebble_sys::VibePattern;

pub fn cancel() {
    unsafe { pebble_sys::vibes_cancel() }
}
pub fn short_pulse() {
    unsafe { pebble_sys::vibes_short_pulse() }
}

pub fn long_pulse() {
    unsafe { pebble_sys::vibes_long_pulse() }
}

pub fn double_pulse() {
    unsafe { pebble_sys::vibes_double_pulse() }
}

pub fn enqueue_custom_pattern(durations: &'static [u32]) {
    let pattern = VibePattern {
        durations: durations.as_ptr(),
        num_segments: durations.len() as u32,
    };
    unsafe { pebble_sys::vibes_enqueue_custom_pattern(pattern) }
}
