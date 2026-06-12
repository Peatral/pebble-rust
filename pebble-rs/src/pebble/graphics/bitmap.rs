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

// TODO: Implement bitmap https://developer.repebble.com/docs/c/Graphics/Graphics_Types/#GBitmap
pub trait IBitmap {
    fn as_ptr(&self) -> *const pebble_sys::GBitmap;
}

pub trait IBitmapMut: IBitmap {
    fn as_mut_ptr(&self) -> *mut pebble_sys::GBitmap;
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct BitmapRef {
    internal: *const pebble_sys::GBitmap,
}

impl IBitmap for BitmapRef {
    fn as_ptr(&self) -> *const pebble_sys::GBitmap {
        self.internal
    }
}

impl From<*const pebble_sys::GBitmap> for BitmapRef {
    fn from(raw: *const pebble_sys::GBitmap) -> Self {
        Self { internal: raw }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct BitmapMut {
    internal: *mut pebble_sys::GBitmap,
}

impl IBitmap for BitmapMut {
    fn as_ptr(&self) -> *const pebble_sys::GBitmap {
        self.internal
    }
}

impl IBitmapMut for BitmapMut {
    fn as_mut_ptr(&self) -> *mut pebble_sys::GBitmap {
        self.internal
    }
}

impl From<*mut pebble_sys::GBitmap> for BitmapMut {
    fn from(raw: *mut pebble_sys::GBitmap) -> Self {
        Self { internal: raw }
    }
}

#[repr(transparent)]
pub struct Bitmap {
    internal: *mut pebble_sys::GBitmap,
}

impl Bitmap {
    pub fn new(resource_id: u32) -> Bitmap {
        unsafe {
            let internal = pebble_sys::gbitmap_create_with_resource(resource_id);
            Bitmap { internal }
        }
    }
}

impl Bitmap {
    pub fn as_ref(&self) -> BitmapRef {
        BitmapRef {
            internal: self.internal,
        }
    }

    pub fn as_mut(&self) -> BitmapMut {
        BitmapMut {
            internal: self.internal,
        }
    }
}

impl IBitmap for Bitmap {
    fn as_ptr(&self) -> *const pebble_sys::GBitmap {
        self.internal
    }
}

impl IBitmapMut for Bitmap {
    fn as_mut_ptr(&self) -> *mut pebble_sys::GBitmap {
        self.internal
    }
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        unsafe {
            if !self.internal.is_null() {
                pebble_sys::gbitmap_destroy(self.internal);
            }
        }
    }
}
