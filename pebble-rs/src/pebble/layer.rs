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

pub mod action_bar_layer;
pub mod bitmap_layer;
pub mod canvas_layer;
pub mod menu_layer;
pub mod status_bar_layer;
pub mod text_layer;

pub use action_bar_layer::ActionBarLayer;
pub use bitmap_layer::BitmapLayer;
pub use canvas_layer::CanvasLayer;
pub use menu_layer::{MenuCellLayer, MenuIndexRef, MenuLayer, MenuLayerDelegate, MenuLayerRef};
pub use status_bar_layer::StatusBarLayer;
pub use text_layer::TextLayer;
use crate::graphics::types::{GPoint, GRect};

pub trait ILayer {
    fn as_ptr(&self) -> *const pebble_sys::Layer;

    fn get_bounds(&self) -> GRect {
        unsafe { GRect(pebble_sys::layer_get_bounds(self.as_ptr())) }
    }

    fn get_unobstructed_bounds(&self) -> GRect {
        unsafe { GRect(pebble_sys::layer_get_unobstructed_bounds(self.as_ptr())) }
    }

    fn convert_point_to_screen(&self, point: GPoint) -> GPoint {
        unsafe { GPoint(pebble_sys::layer_convert_point_to_screen(self.as_ptr(), point.0)) }
    }

    fn convert_rect_to_screen(&self, rect: GRect) -> GRect {
        unsafe { GRect(pebble_sys::layer_convert_rect_to_screen(self.as_ptr(), rect.0)) }
    }

    fn get_frame(&self) -> GRect {
        unsafe { GRect(pebble_sys::layer_get_frame(self.as_ptr())) }
    }

    fn get_hidden(&self) -> bool {
        unsafe { pebble_sys::layer_get_hidden(self.as_ptr()) }
    }
    fn get_clips(&self) -> bool {
        unsafe { pebble_sys::layer_get_clips(self.as_ptr()) }
    }
}

pub trait ILayerMut: ILayer {
    fn as_mut_ptr(&self) -> *mut pebble_sys::Layer;

    fn set_bounds(&self, bounds: GRect) {
        unsafe {
            pebble_sys::layer_set_bounds(self.as_mut_ptr(), bounds.0);
        }
    }
    fn get_unobstructed_bounds(&self) -> GRect {
        unsafe { GRect(pebble_sys::layer_get_unobstructed_bounds(self.as_mut_ptr())) }
    }

    fn set_frame(&self, frame: GRect) {
        unsafe {
            pebble_sys::layer_set_frame(self.as_mut_ptr(), frame.0);
        }
    }

    fn add_child(&self, child: &dyn ILayerMut) {
        unsafe {
            pebble_sys::layer_add_child(self.as_mut_ptr(), child.as_mut_ptr());
        }
    }
    fn remove_from_parent(&self) {
        unsafe {
            pebble_sys::layer_remove_from_parent(self.as_mut_ptr());
        }
    }
    fn remove_child_layers(&self) {
        unsafe {
            pebble_sys::layer_remove_child_layers(self.as_mut_ptr());
        }
    }
    fn insert_below_sibling(&self, sibling: &dyn ILayerMut) {
        unsafe {
            pebble_sys::layer_insert_below_sibling(self.as_mut_ptr(), sibling.as_mut_ptr());
        }
    }
    fn insert_above_sibling(&self, sibling: &dyn ILayerMut) {
        unsafe {
            pebble_sys::layer_insert_above_sibling(self.as_mut_ptr(), sibling.as_mut_ptr());
        }
    }

    fn mark_dirty(&self) {
        unsafe {
            pebble_sys::layer_mark_dirty(self.as_mut_ptr());
        }
    }

    fn set_hidden(&self, hidden: bool) {
        unsafe {
            pebble_sys::layer_set_hidden(self.as_mut_ptr(), hidden);
        }
    }

    fn set_clips(&self, clips: bool) {
        unsafe {
            pebble_sys::layer_set_clips(self.as_mut_ptr(), clips);
        }
    }

    fn set_update_proc(
        &self,
        func: extern "C" fn(*mut pebble_sys::Layer, *mut pebble_sys::GContext),
    ) {
        unsafe {
            pebble_sys::layer_set_update_proc(self.as_mut_ptr(), Some(func));
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct LayerRef {
    internal: *const pebble_sys::Layer,
}

impl ILayer for LayerRef {
    fn as_ptr(&self) -> *const pebble_sys::Layer {
        self.internal
    }
}

impl From<*const pebble_sys::Layer> for LayerRef {
    fn from(internal: *const pebble_sys::Layer) -> Self {
        Self { internal }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct LayerMut {
    internal: *mut pebble_sys::Layer,
}

impl ILayer for LayerMut {
    fn as_ptr(&self) -> *const pebble_sys::Layer {
        self.internal
    }
}

impl ILayerMut for LayerMut {
    fn as_mut_ptr(&self) -> *mut pebble_sys::Layer {
        self.internal
    }
}

impl From<*mut pebble_sys::Layer> for LayerMut {
    fn from(internal: *mut pebble_sys::Layer) -> Self {
        Self { internal }
    }
}

#[repr(transparent)]
pub struct Layer {
    internal: *mut pebble_sys::Layer,
}

impl Layer {
    pub fn new(bounds: GRect) -> Layer {
        unsafe {
            Layer {
                internal: pebble_sys::layer_create(bounds.0),
            }
        }
    }

    /// Internal helper to construct an owned Layer from a raw pointer.
    /// By calling this, you guarantee Rust is responsible for freeing this memory.
    pub(crate) fn from_raw_owned(internal: *mut pebble_sys::Layer) -> Self {
        Self { internal }
    }
}

impl Layer {
    pub fn as_ref(&self) -> LayerRef {
        LayerRef {
            internal: self.internal,
        }
    }

    pub fn as_mut(&self) -> LayerMut {
        LayerMut {
            internal: self.internal,
        }
    }
}

impl ILayer for Layer {
    fn as_ptr(&self) -> *const pebble_sys::Layer {
        self.internal
    }
}

impl ILayerMut for Layer {
    fn as_mut_ptr(&self) -> *mut pebble_sys::Layer {
        self.internal
    }
}

impl Drop for Layer {
    fn drop(&mut self) {
        unsafe {
            pebble_sys::layer_destroy(self.internal);
        }
    }
}
