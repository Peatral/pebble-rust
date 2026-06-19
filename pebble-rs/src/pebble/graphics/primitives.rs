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
use crate::graphics::bitmap::{BitmapMut, BitmapRef, IBitmap, IBitmapMut};
use crate::graphics::context::Context;
use crate::graphics::types::{Point, Rect};
use core::ffi::c_int;
use core::ops::{Deref, DerefMut};
use pebble_sys::{GBitmapFormat, GCornerMask, GOvalScaleMode};

/// An RAII guard that safely manages a captured frame buffer.
/// When this goes out of scope, the frame buffer is automatically released.
pub struct FrameBufferGuard<'a> {
    context: &'a mut Context,
    bitmap: BitmapMut,
}

impl<'a> Deref for FrameBufferGuard<'a> {
    type Target = BitmapMut;

    fn deref(&self) -> &Self::Target {
        &self.bitmap
    }
}

impl<'a> DerefMut for FrameBufferGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bitmap
    }
}

impl<'a> Drop for FrameBufferGuard<'a> {
    fn drop(&mut self) {
        unsafe {
            pebble_sys::graphics_release_frame_buffer(
                self.context.as_ptr(),
                self.bitmap.as_mut_ptr(),
            );
        }
    }
}

impl Context {
    pub fn draw_pixel(&self, center: Point) {
        unsafe {
            pebble_sys::graphics_draw_pixel(self.as_ptr(), center.0);
        }
    }
    pub fn draw_line(&self, p0: Point, p1: Point) {
        unsafe {
            pebble_sys::graphics_draw_line(self.as_ptr(), p0.0, p1.0);
        }
    }
    pub fn draw_rect(&self, rect: Rect) {
        unsafe {
            pebble_sys::graphics_draw_rect(self.as_ptr(), rect.0);
        }
    }
    pub fn fill_rect(&self, rect: Rect, corner_radius: u16, corner_mask: GCornerMask) {
        unsafe {
            pebble_sys::graphics_fill_rect(self.as_ptr(), rect.0, corner_radius, corner_mask);
        }
    }
    pub fn draw_circle(&self, center: Point, radius: u16) {
        unsafe {
            pebble_sys::graphics_draw_circle(self.as_ptr(), center.0, radius);
        }
    }
    pub fn fill_circle(&self, center: Point, radius: u16) {
        unsafe {
            pebble_sys::graphics_fill_circle(self.as_ptr(), center.0, radius);
        }
    }
    pub fn draw_round_rect(&self, rect: Rect, radius: u16) {
        unsafe {
            pebble_sys::graphics_draw_round_rect(self.as_ptr(), rect.0, radius);
        }
    }
    pub fn draw_bitmap_in_rect(&self, bitmap: BitmapRef, rect: Rect) {
        unsafe {
            pebble_sys::graphics_draw_bitmap_in_rect(self.as_ptr(), bitmap.as_ptr(), rect.0);
        }
    }

    /// Captures the frame buffer safely.
    /// Taking `&mut self` ensures no other graphics functions can be called
    /// on this Context until the returned Guard is dropped.
    pub fn capture_frame_buffer(&mut self) -> Option<FrameBufferGuard<'_>> {
        unsafe {
            let ptr = pebble_sys::graphics_capture_frame_buffer(self.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(FrameBufferGuard {
                    context: self,
                    bitmap: ptr.into(),
                })
            }
        }
    }

    /// Captures the frame buffer with a specific format safely.
    pub fn capture_frame_buffer_format(
        &mut self,
        format: GBitmapFormat,
    ) -> Option<FrameBufferGuard<'_>> {
        unsafe {
            let ptr = pebble_sys::graphics_capture_frame_buffer_format(self.as_ptr(), format);
            if ptr.is_null() {
                None
            } else {
                Some(FrameBufferGuard {
                    context: self,
                    bitmap: ptr.into(),
                })
            }
        }
    }

    pub fn frame_buffer_is_captured(&self) -> bool {
        unsafe { pebble_sys::graphics_frame_buffer_is_captured(self.as_ptr()) }
    }

    pub fn draw_rotated_bitmap(
        &self,
        src: BitmapMut,
        src_ic: Point,
        rotation: c_int,
        dest_ic: Point,
    ) {
        unsafe {
            pebble_sys::graphics_draw_rotated_bitmap(
                self.as_ptr(),
                src.as_mut_ptr(),
                src_ic.0,
                rotation,
                dest_ic.0,
            );
        }
    }
    pub fn draw_arc(
        &self,
        rect: Rect,
        scale_mode: GOvalScaleMode,
        angle_start: i32,
        angle_end: i32,
    ) {
        unsafe {
            pebble_sys::graphics_draw_arc(
                self.as_ptr(),
                rect.0,
                scale_mode,
                angle_start,
                angle_end,
            );
        }
    }
    pub fn fill_radial(
        &self,
        rect: Rect,
        scale_mode: GOvalScaleMode,
        inset_thickness: u16,
        angle_start: i32,
        angle_end: i32,
    ) {
        unsafe {
            pebble_sys::graphics_fill_radial(
                self.as_ptr(),
                rect.0,
                scale_mode,
                inset_thickness,
                angle_start,
                angle_end,
            );
        }
    }
}
