use crate::graphics::context::Context;
use core::ffi::c_int;
use pebble_sys::{GBitmap, GBitmapFormat, GCornerMask, GOvalScaleMode};
use crate::graphics::types::{Point, Rect};

impl Context {
    pub fn draw_pixel(&self, center: Point) {
        unsafe { pebble_sys::graphics_draw_pixel(self.as_ptr(), center.0); }
    }
    pub fn draw_line(&self, p0: Point, p1: Point) {
        unsafe { pebble_sys::graphics_draw_line(self.as_ptr(), p0.0, p1.0); }
    }
    pub fn draw_rect(&self, rect: Rect) {
        unsafe { pebble_sys::graphics_draw_rect(self.as_ptr(), rect.0); }
    }
    pub fn fill_rect(&self, rect: Rect, corner_radius: u16, corner_mask: GCornerMask) {
        unsafe { pebble_sys::graphics_fill_rect(self.as_ptr(), rect.0, corner_radius, corner_mask); }
    }
    pub fn draw_circle(&self, center: Point, radius: u16) {
        unsafe { pebble_sys::graphics_draw_circle(self.as_ptr(), center.0, radius); }
    }
    pub fn fill_circle(&self, center: Point, radius: u16) {
        unsafe { pebble_sys::graphics_fill_circle(self.as_ptr(), center.0, radius); }
    }
    pub fn draw_round_rect(&self, rect: Rect, radius: u16) {
        unsafe { pebble_sys::graphics_draw_round_rect(self.as_ptr(), rect.0, radius); }
    }
    pub fn draw_bitmap_in_rect(&self, bitmap: *const GBitmap, rect: Rect) {
        unsafe { pebble_sys::graphics_draw_bitmap_in_rect(self.as_ptr(), bitmap, rect.0); }
    }
    pub fn capture_frame_buffer(&self) -> *mut GBitmap {
        unsafe {
            pebble_sys::graphics_capture_frame_buffer(self.as_ptr())
        }
    }
    pub fn capture_frame_buffer_format(&self, format: GBitmapFormat) -> *mut GBitmap {
        unsafe {
            pebble_sys::graphics_capture_frame_buffer_format(self.as_ptr(), format)
        }
    }
    pub fn release_frame_buffer(&self, buffer: *mut GBitmap) -> bool {
        unsafe {
            pebble_sys::graphics_release_frame_buffer(self.as_ptr(), buffer)
        }
    }
    pub fn frame_buffer_is_captured(&self) -> bool {
        unsafe {
            pebble_sys::graphics_frame_buffer_is_captured(self.as_ptr())
        }
    }
    pub fn draw_rotated_bitmap(
        &self,
        src: *mut GBitmap,
        src_ic: Point,
        rotation: c_int,
        dest_ic: Point,
    ) {
        unsafe {
            pebble_sys::graphics_draw_rotated_bitmap(self.as_ptr(), src, src_ic.0, rotation, dest_ic.0);
        }
    }
    pub fn draw_arc(
        &self,
        rect: Rect,
        scale_mode: GOvalScaleMode,
        angle_start: i32,
        angle_end: i32,
    ) {
        unsafe { pebble_sys::graphics_draw_arc(self.as_ptr(), rect.0, scale_mode, angle_start, angle_end); }
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
