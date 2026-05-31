use crate::pebble::internal::functions::interface;
use crate::pebble::internal::types;
use crate::pebble::internal::types::c_void;
use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::Cell;

/// Pauses the current thread for a specified amount of milliseconds.
pub fn psleep(millis: i32) {
    interface::psleep(millis);
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
    handle: *mut types::AppTimer,
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

        let handle =
            interface::app_timer_register(timeout_ms, timer_trampoline, context_ptr as *mut c_void);

        Self {
            handle,
            context: context_ptr,
            executed,
        }
    }

    /// Reschedules an already running timer.
    /// Returns `true` if rescheduled, `false` if the timer has already elapsed.
    pub fn reschedule(&self, new_timeout_ms: u32) -> bool {
        if self.executed.get() {
            return false;
        }
        interface::app_timer_reschedule(self.handle, new_timeout_ms)
    }

    /// Cancels the timer. Consumes the struct so it cannot be used again.
    pub fn cancel(self) {
        if !self.executed.get() {
            interface::app_timer_cancel(self.handle);

            unsafe {
                let _ = Box::from_raw(self.context);
            }

            self.executed.set(true);
        }
    }
}
