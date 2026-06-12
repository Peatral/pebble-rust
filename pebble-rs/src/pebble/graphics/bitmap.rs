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
use crate::graphics::types::{Color, Rect};
use alloc::vec::Vec;
use pebble_sys::{GBitmapDataRowInfo, GBitmapFormat};

#[inline(always)]
fn palette_len(format: GBitmapFormat) -> usize {
    match format {
        GBitmapFormat::GBitmapFormat1BitPalette => 2,
        GBitmapFormat::GBitmapFormat2BitPalette => 4,
        GBitmapFormat::GBitmapFormat4BitPalette => 16,
        // Standard 1-bit, 8-bit, and circular formats don't have palettes
        _ => 0,
    }
}

// TODO: Implement rest of bitmap https://developer.repebble.com/docs/c/Graphics/Graphics_Types/#GBitmap
// Missing: create_*, bitmap sequence
pub trait IBitmap {
    fn as_ptr(&self) -> *const pebble_sys::GBitmap;

    fn get_bytes_per_row(&self) -> u16 {
        unsafe { pebble_sys::gbitmap_get_bytes_per_row(self.as_ptr()) }
    }

    fn get_format(&self) -> GBitmapFormat {
        unsafe { pebble_sys::gbitmap_get_format(self.as_ptr()) }
    }

    fn get_data(&self) -> &[u8] {
        let bytes_per_row = self.get_bytes_per_row();
        let height = self.get_bounds().size.h;
        let len = (bytes_per_row as usize) * (height as usize);

        unsafe {
            let ptr = pebble_sys::gbitmap_get_data(self.as_ptr());

            if ptr.is_null() {
                return &[];
            }

            core::slice::from_raw_parts(ptr, len)
        }
    }

    fn get_bounds(&self) -> Rect {
        unsafe { Rect(pebble_sys::gbitmap_get_bounds(self.as_ptr())) }
    }

    fn get_palette(&self) -> &[Color] {
        let len = palette_len(self.get_format());
        if len == 0 {
            return &[];
        }

        unsafe {
            let ptr = pebble_sys::gbitmap_get_palette(self.as_ptr()) as *const Color;
            if ptr.is_null() {
                return &[];
            }
            core::slice::from_raw_parts(ptr, len)
        }
    }

    fn get_data_row_info(&self, y: u16) -> GBitmapDataRowInfo {
        unsafe { pebble_sys::gbitmap_get_data_row_info(self.as_ptr(), y) }
    }
}

pub trait IBitmapMut: IBitmap {
    fn as_mut_ptr(&self) -> *mut pebble_sys::GBitmap;

    fn set_bounds(&mut self, bounds: Rect) {
        unsafe {
            pebble_sys::gbitmap_set_bounds(self.as_mut_ptr(), bounds.0);
        }
    }

    fn get_data_mut(&mut self) -> &mut [u8] {
        let bytes_per_row = self.get_bytes_per_row();
        let height = self.get_bounds().size.h;
        let len = (bytes_per_row as usize) * (height as usize);

        unsafe {
            let ptr = pebble_sys::gbitmap_get_data(self.as_mut_ptr());

            if ptr.is_null() {
                return &mut [];
            }

            core::slice::from_raw_parts_mut(ptr, len)
        }
    }

    /// Safely sets the bitmap data. Takes ownership of the Rust Vector.
    /// The Pebble OS will automatically free this memory when the bitmap is destroyed.
    fn set_data(&mut self, data: Vec<u8>, format: GBitmapFormat, row_size_bytes: u16) {
        // Box::into_raw converts the Vec into a raw pointer and prevents Rust from dropping it.
        let ptr = alloc::boxed::Box::into_raw(data.into_boxed_slice()) as *mut u8;

        unsafe {
            pebble_sys::gbitmap_set_data(self.as_mut_ptr(), ptr, format, row_size_bytes, true);
        }
    }

    fn get_palette_mut(&mut self) -> &mut [Color] {
        let len = palette_len(self.get_format());
        if len == 0 {
            return &mut [];
        }

        unsafe {
            let ptr = pebble_sys::gbitmap_get_palette(self.as_mut_ptr()) as *mut Color;
            if ptr.is_null() {
                return &mut [];
            }
            core::slice::from_raw_parts_mut(ptr, len)
        }
    }

    /// Safely sets the palette. Takes ownership of the Rust Vector.
    /// The Pebble OS will automatically free this memory when the bitmap is destroyed.
    fn set_palette(&mut self, palette: Vec<Color>) {
        // Box::into_raw converts the Vec into a raw pointer and prevents Rust from dropping it.
        let ptr = alloc::boxed::Box::into_raw(palette.into_boxed_slice()) as *mut Color;

        unsafe {
            pebble_sys::gbitmap_set_palette(self.as_mut_ptr(), ptr as *mut _, true);
        }
    }

    /// Unsafe escape hatch for static arrays, borrowed memory, or custom C allocators.
    /// The caller is entirely responsible for ensuring the pointer outlives the Bitmap.
    unsafe fn set_data_raw(
        &mut self,
        data: *mut u8,
        format: GBitmapFormat,
        row_size_bytes: u16,
        free_on_destroy: bool,
    ) {
        unsafe {
            pebble_sys::gbitmap_set_data(
                self.as_mut_ptr(),
                data,
                format,
                row_size_bytes,
                free_on_destroy,
            );
        }
    }

    /// Unsafe escape hatch for raw palettes.
    unsafe fn set_palette_raw(&mut self, palette: *mut Color, free_on_destroy: bool) {
        unsafe {
            pebble_sys::gbitmap_set_palette(self.as_mut_ptr(), palette as *mut _, free_on_destroy);
        }
    }
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
