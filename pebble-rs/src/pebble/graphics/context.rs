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
use crate::graphics::types::Color;
use pebble_sys::GCompOp;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Context {
    internal: *mut pebble_sys::GContext,
}

impl Context {
    #[inline(always)]
    pub fn as_ptr(&self) -> *mut pebble_sys::GContext {
        self.internal
    }

    pub fn set_stroke_color(&self, color: Color) {
        unsafe {
            pebble_sys::graphics_context_set_stroke_color(self.internal, color.0);
        }
    }
    pub fn set_fill_color(&self, color: Color) {
        unsafe {
            pebble_sys::graphics_context_set_fill_color(self.internal, color.0);
        }
    }
    pub fn set_text_color(&self, color: Color) {
        unsafe {
            pebble_sys::graphics_context_set_text_color(self.internal, color.0);
        }
    }
    pub fn set_compositing_mode(&self, mode: GCompOp) {
        unsafe {
            pebble_sys::graphics_context_set_compositing_mode(self.internal, mode);
        }
    }
    pub fn set_antialiased(&self, enable: bool) {
        unsafe {
            pebble_sys::graphics_context_set_antialiased(self.internal, enable);
        }
    }
    pub fn set_stroke_width(&self, stroke_width: u8) {
        unsafe { pebble_sys::graphics_context_set_stroke_width(self.internal, stroke_width) }
    }
}

impl From<*mut pebble_sys::GContext> for Context {
    fn from(internal: *mut pebble_sys::GContext) -> Self {
        Self { internal }
    }
}
