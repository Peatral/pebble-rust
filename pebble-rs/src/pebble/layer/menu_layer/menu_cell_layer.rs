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
use crate::graphics::context::Context;
use crate::layer::ILayer;
use core::ffi::CStr;

/// A safe wrapper representing a single menu cell layer during a draw callback.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct MenuCellLayer {
    internal: *const pebble_sys::Layer,
}

impl MenuCellLayer {
    /// Draws a basic section cell with a title, subtitle, and optional icon.
    pub fn draw_basic(
        &self,
        ctx: Context,
        title: &CStr,
        subtitle: Option<&CStr>,
        icon: Option<BitmapMut>,
    ) {
        unsafe {
            pebble_sys::menu_cell_basic_draw(
                ctx.as_ptr(),
                self.internal,
                title.as_ptr(),
                subtitle
                    .map(|b| b.as_ptr())
                    .unwrap_or(core::ptr::null_mut()),
                icon.map(|b| b.as_mut_ptr())
                    .unwrap_or(core::ptr::null_mut()),
            );
        }
    }

    /// Draws a cell layout with only one big title.
    pub fn draw_title(&self, ctx: Context, title: &CStr) {
        unsafe {
            pebble_sys::menu_cell_title_draw(ctx.as_ptr(), self.internal, title.as_ptr());
        }
    }

    /// Draws a basic section header cell layout with the title.
    pub fn draw_basic_header(&self, ctx: Context, title: &CStr) {
        unsafe {
            pebble_sys::menu_cell_basic_header_draw(ctx.as_ptr(), self.internal, title.as_ptr());
        }
    }

    /// Returns whether or not this cell layer is currently highlighted.
    pub fn is_highlighted(&self) -> bool {
        unsafe { pebble_sys::menu_cell_layer_is_highlighted(self.internal) }
    }
}

impl ILayer for MenuCellLayer {
    fn as_ptr(&self) -> *const pebble_sys::Layer {
        self.internal
    }
}

impl From<*const pebble_sys::Layer> for MenuCellLayer {
    fn from(internal: *const pebble_sys::Layer) -> Self {
        Self { internal }
    }
}
