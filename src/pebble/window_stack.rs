use crate::pebble::internal::functions::interface;
use crate::pebble::window::{Window, WindowDelegate, WindowRef};

/// Pushes the given window on the window navigation stack, on top of the current topmost window.
pub fn push<T: WindowDelegate>(window: &Window<T>, animated: bool) {
    interface::window_stack_push(window.as_ptr(), animated);
}

/// Pops the topmost window on the navigation stack.
/// Returns a safe WindowRef to the popped window, or None if the stack is empty.
pub fn pop(animated: bool) -> Option<WindowRef> {
    let ptr = interface::window_stack_pop(animated);
    if ptr.is_null() {
        None
    } else {
        Some(WindowRef::from_ptr(ptr))
    }
}

/// Pops all windows.
/// If there are no windows left on the stack, the app will be killed by the system.
pub fn pop_all(animated: bool) {
    interface::window_stack_pop_all(animated);
}

/// Removes a specific window from the window stack.
/// Returns true if the window was successfully removed, false otherwise.
pub fn remove<T: WindowDelegate>(window: &Window<T>, animated: bool) -> bool {
    interface::window_stack_remove(window.as_ptr(), animated)
}

/// Gets the topmost window on the stack that belongs to the app.
/// Returns a safe WindowRef, or None if no app window could be found.
pub fn get_top_window() -> Option<WindowRef> {
    let ptr = interface::window_stack_get_top_window();
    if ptr.is_null() {
        None
    } else {
        Some(WindowRef::from_ptr(ptr))
    }
}

/// Checks if a specific window is currently on the window stack.
pub fn contains<T: WindowDelegate>(window: &Window<T>) -> bool {
    interface::window_stack_contains_window(window.as_ptr())
}
