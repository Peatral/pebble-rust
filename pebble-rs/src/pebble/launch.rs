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
use pebble_sys::AppLaunchReason;

/// Provides the method used to launch the current application.
pub fn get_reason() -> AppLaunchReason {
    unsafe { pebble_sys::launch_reason() }
}

/// Gets the argument passed to the app when it was launched.
///
/// This is typically used when an application is opened via a timeline pin action.
/// Returns `0` if the app was not launched from a Launch App action.
pub fn get_args() -> u32 {
    unsafe { pebble_sys::launch_get_args() }
}
