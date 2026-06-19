/*
 * This file is part of pebble-rs.
 * Copyright (c) 2019 RoccoDev
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
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct MenuIndexRef {
    internal: *mut pebble_sys::MenuIndex,
}

impl MenuIndexRef {
    #[inline(always)]
    pub fn as_ptr(&self) -> *mut pebble_sys::MenuIndex {
        self.internal
    }

    pub fn section(&self) -> u16 {
        debug_assert!(!self.internal.is_null(), "MenuIndex pointer was null!");
        unsafe { (*self.internal).section }
    }

    pub fn set_section(&mut self, section: u16) {
        debug_assert!(!self.internal.is_null(), "MenuIndex pointer was null!");
        unsafe {
            (*self.internal).section = section;
        }
    }

    pub fn row(&self) -> u16 {
        debug_assert!(!self.internal.is_null(), "MenuIndex pointer was null!");
        unsafe { (*self.internal).row }
    }

    pub fn set_row(&mut self, row: u16) {
        debug_assert!(!self.internal.is_null(), "MenuIndex pointer was null!");
        unsafe {
            (*self.internal).row = row;
        }
    }
}

impl From<*mut pebble_sys::MenuIndex> for MenuIndexRef {
    fn from(internal: *mut pebble_sys::MenuIndex) -> Self {
        Self { internal }
    }
}
