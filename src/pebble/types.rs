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
use crate::pebble::internal::functions::{declarations, interface};
use crate::pebble::internal::types::GBitmap;
pub use crate::pebble::internal::types::{
    AppLaunchReason, AppMessageResult, ButtonId, GColor, GColor8, GCompOp, GContext,
    GOvalScaleMode, GPoint, GRect, GSize, GTextAlignment, Layer, MenuIndex, MenuLayer, Status,
    StatusCode, TimeUnits, Tuple, TupleValue, WakeupId, time_t, tm,
};
use core::cell::{Ref, RefCell, RefMut};
use core::ffi::c_void;

pub type VoidPtr = *const c_void;
pub type DictPtr = *mut crate::pebble::internal::types::DictionaryIterator;

pub struct Bitmap {
    pub internal: *mut GBitmap,
}

impl Bitmap {
    pub fn new(resource_id: u32) -> Bitmap {
        let internal = interface::gbitmap_create_with_resource(resource_id);
        Bitmap { internal }
    }
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        unsafe {
            if !self.internal.is_null() {
                declarations::gbitmap_destroy(self.internal);
            }
        }
    }
}

/// A wrapper for global state in a single-threaded environment (like Pebble).
pub struct GlobalCell<T>(RefCell<T>);

impl<T> GlobalCell<T> {
    pub const fn new(value: T) -> Self {
        Self(RefCell::new(value))
    }

    /// Immutably borrows the wrapped value.
    /// Panics if the value is currently mutably borrowed.
    pub fn borrow(&self) -> Ref<'_, T> {
        self.0.borrow()
    }

    /// Mutably borrows the wrapped value.
    /// Panics if the value is currently borrowed.
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.0.borrow_mut()
    }
}

// We promise the compiler this is safe to share globally
// ONLY because Pebble is single-threaded.
unsafe impl<T> Sync for GlobalCell<T> {}
