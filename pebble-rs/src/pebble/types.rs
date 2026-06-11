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
use core::cell::{Cell, Ref, RefCell, RefMut};

/// A wrapper for global state in a single-threaded environment.
pub struct GlobalRefCell<T>(RefCell<T>);

impl<T> GlobalRefCell<T> {
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
unsafe impl<T> Sync for GlobalRefCell<T> {}

/// A wrapper for global state in a single-threaded environment.
pub struct GlobalCell<T>(Cell<T>);

impl<T> GlobalCell<T> {
    pub const fn new(value: T) -> Self {
        Self(Cell::new(value))
    }

    /// Gets the wrapped value (only works if T is Copy).
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        self.0.get()
    }

    /// Sets the wrapped value.
    pub fn set(&self, value: T) {
        self.0.set(value)
    }
}

// We promise the compiler this is safe to share globally
// ONLY because Pebble is single-threaded.
unsafe impl<T> Sync for GlobalCell<T> {}
