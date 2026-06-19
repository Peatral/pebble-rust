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
use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::Cell;
use core::ffi::c_void;

/// Pauses the current thread for a specified amount of milliseconds.
pub fn psleep(millis: i32) {
    unsafe {
        pebble_sys::psleep(millis);
    }
}

/// Internal context passed to the C API via the void pointer.
struct TimerContext {
    /// The user's actual Rust closure
    callback: Box<dyn FnMut()>,
    /// A shared flag to track if the timer has fired
    executed: Rc<Cell<bool>>,
}

/// The C trampoline that catches the callback and executes the Rust closure.
extern "C" fn timer_trampoline(data: *mut c_void) {
    if data.is_null() {
        return;
    }

    let mut context = unsafe { Box::from_raw(data as *mut TimerContext) };

    context.executed.set(true);

    (context.callback)();
}

/// A safe Rust wrapper around the C AppTimer handle.
pub struct AppTimer {
    handle: *mut pebble_sys::AppTimer,
    context: *mut TimerContext,
    executed: Rc<Cell<bool>>,
}

impl AppTimer {
    /// Registers a timer that executes a closure after `timeout_ms`.
    pub fn register<F>(timeout_ms: u32, callback: F) -> Self
    where
        F: FnMut() + 'static,
    {
        let executed = Rc::new(Cell::new(false));

        let context = Box::new(TimerContext {
            callback: Box::new(callback),
            executed: Rc::clone(&executed),
        });

        let context_ptr = Box::into_raw(context);

        unsafe {
            let handle = pebble_sys::app_timer_register(
                timeout_ms,
                Some(timer_trampoline),
                context_ptr as *mut c_void,
            );

            Self {
                handle,
                context: context_ptr,
                executed,
            }
        }
    }

    /// Reschedules an already running timer.
    /// Returns `true` if rescheduled, `false` if the timer has already elapsed.
    pub fn reschedule(&self, new_timeout_ms: u32) -> bool {
        if self.executed.get() {
            return false;
        }
        unsafe { pebble_sys::app_timer_reschedule(self.handle, new_timeout_ms) }
    }

    /// Cancels the timer. Consumes the struct so it cannot be used again.
    pub fn cancel(self) {}
}

impl Drop for AppTimer {
    fn drop(&mut self) {
        if !self.executed.get() {
            unsafe {
                pebble_sys::app_timer_cancel(self.handle);

                let _ = Box::from_raw(self.context);
            }

            self.executed.set(true);
        }
    }
}
