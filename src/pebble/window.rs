/*
 * This file is part of pebble-rust.
 * Copyright (c) 2019 RoccoDev
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

use crate::pebble::internal::functions::interface;
use crate::pebble::internal::types;
use crate::pebble::internal::types::c_void;
use crate::pebble::layer::Layer;
use crate::pebble::types::GColor;
use crate::WindowPtr;
use alloc::boxed::Box;

/// A safe, non-owning reference to a Window.
/// Used inside callbacks to interact with the window safely without triggering Drop.
pub struct WindowRef {
    internal: WindowPtr,
}

impl WindowRef {
    pub fn push(&self, animate: bool) {
        interface::window_stack_push(self.internal, animate);
    }

    pub fn set_background_color(&self, color: GColor) {
        interface::window_set_background_color(self.internal, color);
    }

    pub fn get_root_layer(&self) -> Layer {
        let layer_ptr = interface::window_get_root_layer(self.internal);
        Layer::from_raw(layer_ptr)
    }
}

/// Safe Rust trait for Window lifecycle events
pub trait WindowDelegate: Sized {
    fn load(&self, _window: WindowRef) {}
    fn unload(&self, _window: WindowRef) {}
    fn appear(&self, _window: WindowRef) {}
    fn disappear(&self, _window: WindowRef) {}
}

pub struct Window<T: WindowDelegate> {
    internal: *mut types::Window,
    delegate: Box<T>,
}

impl<T: WindowDelegate> Window<T> {
    pub fn new(delegate: T) -> Self {
        let internal = interface::window_create();

        let window = Window {
            internal,
            delegate: Box::new(delegate),
        };

        let context_ptr = &*window.delegate as *const T as *mut c_void;
        interface::window_set_user_data(window.internal, context_ptr);

        let handlers = types::WindowHandlers {
            load: Some(trampoline_load::<T>),
            unload: Some(trampoline_unload::<T>),
            appear: Some(trampoline_appear::<T>),
            disappear: Some(trampoline_disappear::<T>),
        };
        interface::window_set_window_handlers(window.internal, handlers);

        window
    }

    pub fn push(&self, animate: bool) {
        interface::window_stack_push(self.internal, animate);
    }

    pub fn set_background_color(&self, color: GColor) {
        interface::window_set_background_color(self.internal, color);
    }

    pub fn get_root_layer(&self) -> Layer {
        let layer_ptr = interface::window_get_root_layer(self.internal);
        Layer::from_raw(layer_ptr)
    }
}

impl<T: WindowDelegate> Drop for Window<T> {
    fn drop(&mut self) {
        interface::window_destroy(self.internal);
    }
}

extern "C" fn trampoline_load<T: WindowDelegate>(window_ptr: WindowPtr) {
    unsafe {
        let user_data: *mut c_void = interface::window_get_user_data(window_ptr);
        if !user_data.is_null() {
            let delegate = &*(user_data as *const T);
            delegate.load(WindowRef {
                internal: window_ptr,
            });
        }
    }
}

extern "C" fn trampoline_unload<T: WindowDelegate>(window_ptr: WindowPtr) {
    unsafe {
        let user_data: *mut c_void = interface::window_get_user_data(window_ptr);
        if !user_data.is_null() {
            let delegate = &*(user_data as *const T);
            delegate.unload(WindowRef {
                internal: window_ptr,
            });
        }
    }
}

extern "C" fn trampoline_appear<T: WindowDelegate>(window_ptr: WindowPtr) {
    unsafe {
        let user_data: *mut c_void = interface::window_get_user_data(window_ptr);
        if !user_data.is_null() {
            let delegate = &*(user_data as *const T);
            delegate.appear(WindowRef {
                internal: window_ptr,
            });
        }
    }
}

extern "C" fn trampoline_disappear<T: WindowDelegate>(window_ptr: WindowPtr) {
    unsafe {
        let user_data: *mut c_void = interface::window_get_user_data(window_ptr);
        if !user_data.is_null() {
            let delegate = &*(user_data as *const T);
            delegate.disappear(WindowRef {
                internal: window_ptr,
            });
        }
    }
}
