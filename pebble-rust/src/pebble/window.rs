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

use crate::layer::LayerMut;
use crate::pebble::internal::functions::interface;
use crate::pebble::internal::types;
use crate::pebble::types::GColor;
use alloc::boxed::Box;
use core::ffi::c_void;
use core::ops::Deref;

/// A safe, non-owning reference to a Window.
/// Used inside callbacks to interact with the window safely without triggering Drop.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct WindowRef {
    pub(crate) internal: *mut types::Window,
}

impl WindowRef {
    pub fn as_ptr(&self) -> *mut types::Window {
        self.internal
    }

    pub fn set_background_color(&self, color: GColor) {
        interface::window_set_background_color(self.internal, color);
    }

    pub fn get_root_layer(&self) -> LayerMut {
        let layer_ptr = interface::window_get_root_layer(self.internal);
        layer_ptr.into()
    }
}

impl From<*mut types::Window> for WindowRef {
    fn from(internal: *mut types::Window) -> Self {
        Self { internal }
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
    window_ref: WindowRef,
    delegate: Box<T>,
}

impl<T: WindowDelegate> Window<T> {
    pub fn new(delegate: T) -> Self {
        let internal = interface::window_create();

        let window = Window {
            window_ref: internal.into(),
            delegate: Box::new(delegate),
        };

        let context_ptr = &*window.delegate as *const T as *mut c_void;

        interface::window_set_user_data(window.window_ref.internal, context_ptr);

        let handlers = types::WindowHandlers {
            load: Some(trampoline_load::<T>),
            unload: Some(trampoline_unload::<T>),
            appear: Some(trampoline_appear::<T>),
            disappear: Some(trampoline_disappear::<T>),
        };
        interface::window_set_window_handlers(window.window_ref.internal, handlers);

        window
    }
}

impl<T: WindowDelegate> Deref for Window<T> {
    type Target = WindowRef;

    fn deref(&self) -> &Self::Target {
        &self.window_ref
    }
}

impl<T: WindowDelegate> Drop for Window<T> {
    fn drop(&mut self) {
        interface::window_destroy(self.window_ref.internal);
    }
}

extern "C" fn trampoline_load<T: WindowDelegate>(window_ptr: *mut types::Window) {
    unsafe {
        let user_data: *mut c_void = interface::window_get_user_data(window_ptr);
        if !user_data.is_null() {
            let delegate = &*(user_data as *const T);
            delegate.load(window_ptr.into());
        }
    }
}

extern "C" fn trampoline_unload<T: WindowDelegate>(window_ptr: *mut types::Window) {
    unsafe {
        let user_data: *mut c_void = interface::window_get_user_data(window_ptr);
        if !user_data.is_null() {
            let delegate = &*(user_data as *const T);
            delegate.unload(window_ptr.into());
        }
    }
}

extern "C" fn trampoline_appear<T: WindowDelegate>(window_ptr: *mut types::Window) {
    unsafe {
        let user_data: *mut c_void = interface::window_get_user_data(window_ptr);
        if !user_data.is_null() {
            let delegate = &*(user_data as *const T);
            delegate.appear(window_ptr.into());
        }
    }
}

extern "C" fn trampoline_disappear<T: WindowDelegate>(window_ptr: *mut types::Window) {
    unsafe {
        let user_data: *mut c_void = interface::window_get_user_data(window_ptr);
        if !user_data.is_null() {
            let delegate = &*(user_data as *const T);
            delegate.disappear(window_ptr.into());
        }
    }
}
