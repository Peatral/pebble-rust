/*
 * This file is part of pebble-rs.
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

use core::ffi::{CStr, c_void};
use pebble_sys::{AppMessageResult, DictionaryIterator, DictionaryResult, Tuple};

/// Represents a `DictionaryIterator`, essentially a list of `Tuple`s.
pub struct Dictionary {
    internal: *mut DictionaryIterator,
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}

impl Dictionary {
    #[allow(clippy::cast_ptr_alignment)]
    pub fn new() -> Self {
        let null_ptr = core::ptr::null_mut();
        let mut iter = DictionaryIterator {
            end: null_ptr,
            cursor: null_ptr as *mut _,
            dictionary: null_ptr as *mut pebble_sys::Dictionary,
        };

        Self {
            internal: &mut iter as *mut DictionaryIterator,
        }
    }

    /// Fetches the underlying dictionary from a raw pointer.
    pub fn from_raw(raw: *mut DictionaryIterator) -> Self {
        Self { internal: raw }
    }

    /// Prepares the dictionary for reading.
    /// Calling this is **required** after writing, before reading.
    pub fn init_read(&self, buffer: &mut [u8]) -> Option<&Tuple> {
        unsafe {
            let ptr = pebble_sys::dict_read_begin_from_buffer(
                self.internal,
                buffer.as_mut_ptr(),
                buffer.len() as u16,
            );
            if ptr.is_null() { None } else { Some(&*ptr) }
        }
    }

    /// Prepares the dictionary for writing.
    /// You don't need to call this if you use `AppMessage`.
    pub fn init_write(&self, buffer: &mut [u8]) {
        unsafe {
            pebble_sys::dict_write_begin(self.internal, buffer.as_mut_ptr(), buffer.len() as u16);
        }
    }

    /// Attempts to read the next `Tuple` in the dictionary.
    pub fn read_next(&self) -> Option<&Tuple> {
        unsafe {
            let ptr = pebble_sys::dict_read_next(self.internal);
            if ptr.is_null() { None } else { Some(&*ptr) }
        }
    }

    /// Resets the dictionary, and returns the first `Tuple`, if present.
    pub fn reset(&self) -> Option<&Tuple> {
        unsafe {
            let ptr = pebble_sys::dict_read_first(self.internal);
            if ptr.is_null() { None } else { Some(&*ptr) }
        }
    }

    /// Attempts to find a `Tuple` by its key.
    pub fn find(&self, key: u32) -> Option<&Tuple> {
        unsafe {
            let ptr = pebble_sys::dict_find(self.internal, key);
            if ptr.is_null() { None } else { Some(&*ptr) }
        }
    }

    pub fn write_string(&self, key: u32, string: &CStr) -> Result<(), DictionaryResult> {
        unsafe {
            let status = pebble_sys::dict_write_cstring(self.internal, key, string.as_ptr());
            if status == DictionaryResult::DICT_OK {
                Ok(())
            } else {
                Err(status)
            }
        }
    }

    pub fn prepare_for_read(&self) {
        unsafe { pebble_sys::dict_write_end(self.internal) };
    }

    pub fn write_int<T: Integer>(&self, key: u32, int: T) {
        unsafe {
            let ptr = &int as *const T as *const c_void;
            pebble_sys::dict_write_int(
                self.internal,
                key,
                ptr,
                size_of_val(&int) as u8,
                int.signed(),
            );
        }
    }

    /// Adds a key with a byte array value pair to the dictionary.
    pub fn write_data(&self, key: u32, data: &[u8]) -> Result<(), DictionaryResult> {
        unsafe {
            let status =
                pebble_sys::dict_write_data(self.internal, key, data.as_ptr(), data.len() as u16);
            if status == DictionaryResult::DICT_OK {
                Ok(())
            } else {
                Err(status)
            }
        }
    }
}

pub trait Integer {
    fn signed(&self) -> bool;
}

macro_rules! impl_signed {
    (for $($t:ty),+) => {
        $(impl Integer for $t {
            fn signed(&self) -> bool { true }
        })*
    }
}

macro_rules! impl_unsigned {
    (for $($t:ty),+) => {
        $(impl Integer for $t {
            fn signed(&self) -> bool { false }
        })*
    }
}

impl_signed!(for i32, i64, i8, i16, isize);
impl_unsigned!(for u32, u64, u8, u16, usize);

static mut INBOX_RECEIVED: Option<fn(Dictionary)> = None;
static mut INBOX_DROPPED: Option<fn(AppMessageResult)> = None;
static mut OUTBOX_SENT: Option<fn(Dictionary)> = None;
static mut OUTBOX_FAILED: Option<fn(Dictionary, AppMessageResult)> = None;

extern "C" fn trampoline_inbox_received(dict_ptr: *mut DictionaryIterator, _ctx: *mut cty::c_void) {
    unsafe {
        if let Some(handler) = INBOX_RECEIVED {
            handler(Dictionary::from_raw(dict_ptr));
        }
    }
}

extern "C" fn trampoline_inbox_dropped(reason: AppMessageResult, _ctx: *mut cty::c_void) {
    unsafe {
        if let Some(handler) = INBOX_DROPPED {
            handler(AppMessageResult::from(reason));
        }
    }
}

extern "C" fn trampoline_outbox_sent(dict_ptr: *mut DictionaryIterator, _ctx: *mut cty::c_void) {
    unsafe {
        if let Some(handler) = OUTBOX_SENT {
            handler(Dictionary::from_raw(dict_ptr));
        }
    }
}

extern "C" fn trampoline_outbox_failed(
    dict_ptr: *mut DictionaryIterator,
    reason: AppMessageResult,
    _ctx: *mut cty::c_void,
) {
    unsafe {
        if let Some(handler) = OUTBOX_FAILED {
            handler(
                Dictionary::from_raw(dict_ptr),
                AppMessageResult::from(reason),
            );
        }
    }
}

pub struct AppMessage;

impl AppMessage {
    /// Opens the AppMessage subsystem.
    /// Note: Callbacks should be registered BEFORE calling open.
    pub fn open(size_inbound: u32, size_outbound: u32) -> Result<(), AppMessageResult> {
        let result = unsafe { pebble_sys::app_message_open(size_inbound, size_outbound) };
        let status = AppMessageResult::from(result);
        if status == AppMessageResult::APP_MSG_OK {
            Ok(())
        } else {
            Err(status)
        }
    }

    pub fn inbox_size_maximum() -> u32 {
        unsafe { pebble_sys::app_message_inbox_size_maximum() }
    }

    pub fn outbox_size_maximum() -> u32 {
        unsafe { pebble_sys::app_message_outbox_size_maximum() }
    }

    pub fn register_inbox_received(handler: fn(Dictionary)) {
        unsafe {
            INBOX_RECEIVED = Some(handler);
            pebble_sys::app_message_register_inbox_received(Some(trampoline_inbox_received));
        }
    }

    pub fn register_inbox_dropped(handler: fn(AppMessageResult)) {
        unsafe {
            INBOX_DROPPED = Some(handler);
            pebble_sys::app_message_register_inbox_dropped(Some(trampoline_inbox_dropped));
        }
    }

    pub fn register_outbox_sent(handler: fn(Dictionary)) {
        unsafe {
            OUTBOX_SENT = Some(handler);
            pebble_sys::app_message_register_outbox_sent(Some(trampoline_outbox_sent));
        }
    }

    pub fn register_outbox_failed(handler: fn(Dictionary, AppMessageResult)) {
        unsafe {
            OUTBOX_FAILED = Some(handler);
            pebble_sys::app_message_register_outbox_failed(Some(trampoline_outbox_failed));
        }
    }
}

pub struct Outbox;

impl Outbox {
    /// Prepares a new dictionary for outgoing transmission.
    pub fn begin() -> Result<Dictionary, AppMessageResult> {
        unsafe {
            let mut iter: *mut DictionaryIterator = core::ptr::null_mut();
            let result = pebble_sys::app_message_outbox_begin(&mut iter);
            let status = AppMessageResult::from(result);

            if status == AppMessageResult::APP_MSG_OK && !iter.is_null() {
                Ok(Dictionary::from_raw(iter))
            } else {
                Err(status)
            }
        }
    }

    pub fn send() -> Result<(), AppMessageResult> {
        let result = unsafe { pebble_sys::app_message_outbox_send() };
        let status = AppMessageResult::from(result);
        if status == AppMessageResult::APP_MSG_OK {
            Ok(())
        } else {
            Err(status)
        }
    }
}
