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

use crate::pebble::internal::functions::declarations::text_layer_set_text;
use crate::pebble::internal::types::GTextAlignment;
use crate::pebble::internal::{functions::interface, types};
use crate::pebble::types::{Bitmap, GCompOp, GRect};
use crate::system::fonts::Font;
use core::ffi::{CStr, c_char};

pub mod action_bar_layer;
pub mod menu_layer;
pub mod status_bar_layer;

pub use action_bar_layer::ActionBarLayer;
pub use menu_layer::{MenuIndexRef, MenuLayer, MenuLayerDelegate, MenuLayerRef};
pub use status_bar_layer::StatusBarLayer;
use crate::types::GPoint;

/// A safe, immutable reference to a standard UI Layer.
/// Used primarily for reading properties of layers you don't own (like the menu background).
pub struct LayerRef {
    internal: *const types::Layer,
}

pub struct Layer {
    internal: *mut types::Layer,
    is_owned_by_rust: bool,
}

pub struct TextLayer {
    internal: *mut types::TextLayer,
    inner: *mut types::Layer,
}

pub struct BitmapLayer {
    internal: *mut types::BitmapLayer,
    inner: *mut types::Layer,
}

pub trait ILayer {
    fn as_ptr(&self) -> *const types::Layer;

    fn get_bounds(&self) -> GRect {
        interface::layer_get_bounds(self.as_ptr())
    }

    fn get_unobstructed_bounds(&self) -> GRect {
        interface::layer_get_unobstructed_bounds(self.as_ptr())
    }

    fn convert_point_to_screen(&self, point: GPoint) -> GPoint {
        interface::layer_convert_point_to_screen(self.as_ptr(), point)
    }

    fn convert_rect_to_screen(&self, rect: GRect) -> GRect {
        interface::layer_convert_rect_to_screen(self.as_ptr(), rect)
    }

    fn get_frame(&self) -> GRect {
        interface::layer_get_frame(self.as_ptr())
    }

    fn get_hidden(&self) -> bool {
        interface::layer_get_hidden(self.as_ptr())
    }
    fn get_clips(&self) -> bool {
        interface::layer_get_clips(self.as_ptr())
    }
}

pub trait ILayerMut: ILayer {
    fn as_mut_ptr(&self) -> *mut types::Layer;

    fn set_bounds(&self, bounds: GRect) {
        interface::layer_set_bounds(self.as_mut_ptr(), bounds);
    }
    fn get_unobstructed_bounds(&self) -> GRect {
        interface::layer_get_unobstructed_bounds(self.as_mut_ptr())
    }

    fn set_frame(&self, frame: GRect) {
        interface::layer_set_frame(self.as_mut_ptr(), frame);
    }

    fn add_child(&self, child: &dyn ILayerMut) {
        interface::layer_add_child(self.as_mut_ptr(), child.as_mut_ptr());
    }
    fn remove_from_parent(&self) {
        interface::layer_remove_from_parent(self.as_mut_ptr());
    }
    fn remove_child_layers(&self) {
        interface::layer_remove_child_layers(self.as_mut_ptr());
    }
    fn insert_below_sibling(&self, sibling: &dyn ILayerMut) {
        interface::layer_insert_below_sibling(self.as_mut_ptr(), sibling.as_mut_ptr());
    }
    fn insert_above_sibling(&self, sibling: &dyn ILayerMut) {
        interface::layer_insert_above_sibling(self.as_mut_ptr(), sibling.as_mut_ptr());
    }

    fn mark_dirty(&self) {
        interface::layer_mark_dirty(self.as_mut_ptr());
    }

    fn set_hidden(&self, hidden: bool) {
        interface::layer_set_hidden(self.as_mut_ptr(), hidden);
    }

    fn set_clips(&self, clips: bool) {
        interface::layer_set_clips(self.as_mut_ptr(), clips);
    }
}

impl ILayer for LayerRef {
    fn as_ptr(&self) -> *const types::Layer {
        self.internal
    }
}

impl LayerRef {
    pub(crate) fn from_ptr(ptr: *const types::Layer) -> Self {
        Self { internal: ptr }
    }
}

impl ILayer for Layer {
    fn as_ptr(&self) -> *const types::Layer {
        self.internal
    }
}

impl ILayerMut for Layer {
    fn as_mut_ptr(&self) -> *mut types::Layer {
        self.internal
    }
}

impl Layer {
    pub fn new(bounds: GRect) -> Layer {
        Layer {
            internal: interface::layer_create(bounds),
            is_owned_by_rust: false,
        }
    }

    pub(crate) fn from_ptr(ptr: *mut types::Layer, is_owned_by_rust: bool) -> Layer {
        Layer {
            internal: ptr,
            is_owned_by_rust,
        }
    }
}

impl ILayer for TextLayer {
    fn as_ptr(&self) -> *const types::Layer {
        self.inner
    }
}

impl ILayerMut for TextLayer {
    fn as_mut_ptr(&self) -> *mut types::Layer {
        self.inner
    }
}

impl Drop for Layer {
    fn drop(&mut self) {
        if self.is_owned_by_rust {
            interface::layer_destroy(self.internal);
        }
    }
}

impl TextLayer {
    pub fn new(bounds: GRect) -> TextLayer {
        let internal = interface::text_layer_create(bounds);
        let inner = interface::text_layer_get_layer(internal);

        TextLayer { internal, inner }
    }

    pub fn set_text(&self, text: &CStr) {
        interface::text_layer_set_text(self.internal, text);
    }
    pub unsafe fn set_text_raw(&self, text: *const c_char) {
        unsafe {
            text_layer_set_text(self.internal, text);
        }
    }

    pub fn set_font(&self, font: Font) {
        interface::text_layer_set_font(self.internal, font.internal)
    }
    pub fn set_text_alignment(&self, alignment: GTextAlignment) {
        interface::text_layer_set_text_alignment(self.internal, alignment);
    }
}

impl Drop for TextLayer {
    fn drop(&mut self) {
        interface::text_layer_destroy(self.internal);
    }
}

impl ILayer for BitmapLayer {
    fn as_ptr(&self) -> *const types::Layer {
        self.inner
    }
}

impl ILayerMut for BitmapLayer {
    fn as_mut_ptr(&self) -> *mut types::Layer {
        self.inner
    }
}

impl BitmapLayer {
    pub fn new(bounds: GRect) -> BitmapLayer {
        let internal = interface::bitmap_layer_create(bounds);
        let inner = interface::bitmap_layer_get_layer(internal);

        BitmapLayer { internal, inner }
    }

    pub fn set_bitmap(&self, bitmap: &Bitmap) {
        interface::bitmap_layer_set_bitmap(self.internal, bitmap.internal);
    }

    pub fn set_compositing_mode(&self, mode: GCompOp) {
        interface::bitmap_layer_set_compositing_mode(self.internal, mode);
    }
}

impl Drop for BitmapLayer {
    fn drop(&mut self) {
        interface::bitmap_layer_destroy(self.internal);
    }
}
