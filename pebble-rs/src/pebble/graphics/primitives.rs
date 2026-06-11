use crate::graphics::context::GContext;
use crate::pebble::internal::functions::interface;
use crate::pebble::internal::types::{GBitmap, GBitmapFormat, GCornerMask};
use crate::types::{GOvalScaleMode, GPoint, GRect};
use core::ffi::c_int;

impl GContext {
    pub fn draw_pixel(&self, center: GPoint) {
        interface::graphics_draw_pixel(self.as_ptr(), center);
    }
    pub fn draw_line(&self, p0: GPoint, p1: GPoint) {
        interface::graphics_draw_line(self.as_ptr(), p0, p1);
    }
    pub fn draw_rect(&self, rect: GRect) {
        interface::graphics_draw_rect(self.as_ptr(), rect);
    }
    pub fn fill_rect(&self, rect: GRect, corner_radius: u16, corner_mask: GCornerMask) {
        interface::graphics_fill_rect(self.as_ptr(), rect, corner_radius, corner_mask);
    }
    pub fn draw_circle(&self, center: GPoint, radius: u16) {
        interface::graphics_draw_circle(self.as_ptr(), center, radius);
    }
    pub fn fill_circle(&self, center: GPoint, radius: u16) {
        interface::graphics_fill_circle(self.as_ptr(), center, radius);
    }
    pub fn draw_round_rect(&self, rect: GRect, radius: u16) {
        interface::graphics_draw_round_rect(self.as_ptr(), rect, radius);
    }
    pub fn draw_bitmap_in_rect(&self, bitmap: *const GBitmap, rect: GRect) {
        interface::graphics_draw_bitmap_in_rect(self.as_ptr(), bitmap, rect);
    }
    pub fn capture_frame_buffer(&self) -> *mut GBitmap {
        interface::graphics_capture_frame_buffer(self.as_ptr())
    }
    pub fn capture_frame_buffer_format(&self, format: GBitmapFormat) -> *mut GBitmap {
        interface::graphics_capture_frame_buffer_format(self.as_ptr(), format)
    }
    pub fn release_frame_buffer(&self, buffer: *mut GBitmap) -> bool {
        interface::graphics_release_frame_buffer(self.as_ptr(), buffer)
    }
    pub fn frame_buffer_is_captured(&self) -> bool {
        interface::graphics_frame_buffer_is_captured(self.as_ptr())
    }
    pub fn draw_rotated_bitmap(
        &self,
        src: *mut GBitmap,
        src_ic: GPoint,
        rotation: c_int,
        dest_ic: GPoint,
    ) {
        interface::graphics_draw_rotated_bitmap(self.as_ptr(), src, src_ic, rotation, dest_ic);
    }
    pub fn draw_arc(
        &self,
        rect: GRect,
        scale_mode: GOvalScaleMode,
        angle_start: i32,
        angle_end: i32,
    ) {
        interface::graphics_draw_arc(self.as_ptr(), rect, scale_mode, angle_start, angle_end);
    }
    pub fn fill_radial(
        &self,
        rect: GRect,
        scale_mode: GOvalScaleMode,
        inset_thickness: u16,
        angle_start: i32,
        angle_end: i32,
    ) {
        interface::graphics_fill_radial(
            self.as_ptr(),
            rect,
            scale_mode,
            inset_thickness,
            angle_start,
            angle_end,
        );
    }
}
