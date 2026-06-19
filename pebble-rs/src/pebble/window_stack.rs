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
use crate::pebble::window::WindowRef;

/// Pushes the given window on the window navigation stack, on top of the current topmost window.
pub fn push(window: WindowRef, animated: bool) {
    unsafe {
        pebble_sys::window_stack_push(window.as_ptr(), animated);
    }
}

/// Pops the topmost window on the navigation stack.
/// Returns a safe WindowRef to the popped window, or None if the stack is empty.
pub fn pop(animated: bool) -> Option<WindowRef> {
    unsafe {
        let ptr = pebble_sys::window_stack_pop(animated);
        if ptr.is_null() {
            None
        } else {
            Some(ptr.into())
        }
    }
}

/// Pops all windows.
/// If there are no windows left on the stack, the app will be killed by the system.
pub fn pop_all(animated: bool) {
    unsafe {
        pebble_sys::window_stack_pop_all(animated);
    }
}

/// Removes a specific window from the window stack.
/// Returns true if the window was successfully removed, false otherwise.
pub fn remove(window: WindowRef, animated: bool) -> bool {
    unsafe { pebble_sys::window_stack_remove(window.as_ptr(), animated) }
}

/// Gets the topmost window on the stack that belongs to the app.
/// Returns a safe WindowRef, or None if no app window could be found.
pub fn get_top_window() -> Option<WindowRef> {
    unsafe {
        let ptr = pebble_sys::window_stack_get_top_window();
        if ptr.is_null() {
            None
        } else {
            Some(ptr.into())
        }
    }
}

/// Checks if a specific window is currently on the window stack.
pub fn contains(window: WindowRef) -> bool {
    unsafe { pebble_sys::window_stack_contains_window(window.as_ptr()) }
}
