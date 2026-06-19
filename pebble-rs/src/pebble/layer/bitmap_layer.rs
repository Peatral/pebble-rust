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
use crate::graphics::bitmap::{BitmapMut, IBitmapMut};
use crate::graphics::types::Rect;
use crate::layer::{ILayer, ILayerMut};
use pebble_sys::GCompOp;

#[repr(transparent)]
pub struct BitmapLayer {
    internal: *mut pebble_sys::BitmapLayer,
}

impl BitmapLayer {
    pub fn new(bounds: Rect) -> BitmapLayer {
        unsafe {
            let internal = pebble_sys::bitmap_layer_create(bounds.0);

            BitmapLayer { internal }
        }
    }

    pub fn set_bitmap(&self, bitmap: BitmapMut) {
        unsafe {
            pebble_sys::bitmap_layer_set_bitmap(self.internal, bitmap.as_mut_ptr());
        }
    }

    pub fn set_compositing_mode(&self, mode: GCompOp) {
        unsafe {
            pebble_sys::bitmap_layer_set_compositing_mode(self.internal, mode);
        }
    }
}

impl ILayer for BitmapLayer {
    fn as_ptr(&self) -> *const pebble_sys::Layer {
        unsafe { pebble_sys::bitmap_layer_get_layer(self.internal) }
    }
}

impl ILayerMut for BitmapLayer {
    fn as_mut_ptr(&self) -> *mut pebble_sys::Layer {
        unsafe { pebble_sys::bitmap_layer_get_layer(self.internal) }
    }
}

impl Drop for BitmapLayer {
    fn drop(&mut self) {
        unsafe {
            pebble_sys::bitmap_layer_destroy(self.internal);
        }
    }
}
