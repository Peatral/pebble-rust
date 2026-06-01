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

use crate::pebble::internal::functions::declarations::*;
use core::ffi::{CStr, c_char};

pub fn compare_strings(str1: &CStr, str2: &CStr) -> i32 {
    unsafe { strcmp(str1.as_ptr(), str2.as_ptr()) }
}

pub fn compare_strings_bytes(str1: &CStr, str2: &CStr, max_bytes: usize) -> i32 {
    unsafe { strncmp(str1.as_ptr(), str2.as_ptr(), max_bytes) }
}

/// # Safety
/// `dest` must be a valid, mutable pointer to a buffer large enough to hold `source`.
pub unsafe fn copy_strings<'a>(source: &CStr, dest: *mut c_char) -> &'a str {
    unsafe {
        let ptr = strcpy(dest, source.as_ptr());
        ptr_to_str(ptr)
    }
}

/// # Safety
/// `dest` must be a valid, mutable pointer.
pub unsafe fn copy_strings_bytes<'a>(
    source: &CStr,
    dest: *mut c_char,
    max_bytes: usize,
) -> &'a str {
    unsafe {
        let ptr = strncpy(dest, source.as_ptr(), max_bytes);
        ptr_to_str(ptr)
    }
}

/// # Safety
/// `dest` must be a valid, mutable pointer to a null-terminated string,
/// with enough space to append `source`.
pub unsafe fn concat_strings<'a>(source: &CStr, dest: *mut c_char) -> &'a str {
    unsafe {
        let ptr = strcat(dest, source.as_ptr());
        ptr_to_str(ptr)
    }
}

/// # Safety
/// `dest` must be a valid, mutable pointer.
pub unsafe fn concat_strings_bytes<'a>(
    source: &CStr,
    dest: *mut c_char,
    max_bytes: usize,
) -> &'a str {
    unsafe {
        let ptr = strncat(dest, source.as_ptr(), max_bytes);
        ptr_to_str(ptr)
    }
}

pub fn string_length(string: &CStr) -> usize {
    unsafe { strlen(string.as_ptr()) }
}

unsafe fn ptr_to_str<'a>(ptr: *const c_char) -> &'a str {
    unsafe {
        if ptr.is_null() {
            return "";
        }
        // Safely determine length by finding the null terminator
        CStr::from_ptr(ptr).to_str().unwrap_or("")
    }
}
