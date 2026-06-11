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

use crate::pebble::internal::functions::declarations::setlocale;
use core::ffi::{CStr, c_char};

pub fn set_locale(category: i32, locale: &CStr) {
    unsafe {
        setlocale(category, locale.as_ptr());
    }
}

pub fn get_locale<'a>(category: i32) -> Option<&'a str> {
    unsafe {
        let ptr = setlocale(category, core::ptr::null());

        if ptr.is_null() {
            return None;
        }

        let c_str = CStr::from_ptr(ptr as *const c_char);
        c_str.to_str().ok()
    }
}
