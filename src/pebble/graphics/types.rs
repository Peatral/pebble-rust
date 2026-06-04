use crate::pebble::internal::functions::interface;
use crate::types::{GColor8, GOvalScaleMode, GPoint, GRect, GSize};
use core::cmp::Eq;

impl GColor8 {
    pub fn new(argb: u8) -> Self {
        GColor8 { argb }
    }

    pub fn legible_over(background_color: GColor8) -> Self {
        interface::gcolor_legible_over(background_color)
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let a_bits = 0b1100_0000;

        let r_bits = (r >> 6) << 4;
        let g_bits = (g >> 6) << 2;
        let b_bits = b >> 6;

        Self {
            argb: a_bits | r_bits | g_bits | b_bits,
        }
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        let a_bits = (a >> 6) << 6;
        let r_bits = (r >> 6) << 4;
        let g_bits = (g >> 6) << 2;
        let b_bits = b >> 6;

        Self {
            argb: a_bits | r_bits | g_bits | b_bits,
        }
    }

    pub fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;

        Self::from_rgb(r, g, b)
    }
}
impl PartialEq<Self> for GColor8 {
    fn eq(&self, other: &Self) -> bool {
        if self.argb == other.argb {
            return true;
        }

        let alpha_mask = 0b1100_0000;

        (self.argb | other.argb) & alpha_mask == 0
    }
}
impl Eq for GColor8 {}

impl GPoint {
    pub fn new(x: i16, y: i16) -> Self {
        GPoint { x, y }
    }

    pub fn from_polar(rect: GRect, scale_mode: GOvalScaleMode, angle: i32) -> GPoint {
        interface::gpoint_from_polar(rect, scale_mode, angle)
    }
}

impl GSize {
    pub fn new(w: i16, h: i16) -> Self {
        GSize { w, h }
    }
}

impl GRect {
    pub fn new(origin: GPoint, size: GSize) -> Self {
        GRect { origin, size }
    }

    pub fn centered_from_polar(
        rect: GRect,
        scale_mode: GOvalScaleMode,
        angle: i32,
        size: GSize,
    ) -> GRect {
        interface::grect_centered_from_polar(rect, scale_mode, angle, size)
    }

    pub fn is_empty(&self) -> bool {
        interface::grect_is_empty(self)
    }

    pub fn standardize(&mut self) {
        interface::grect_standardize(self)
    }

    pub fn clip(&mut self, clipper: &GRect) {
        interface::grect_clip(self, clipper)
    }

    pub fn contains_point(&self, point: &GPoint) -> bool {
        interface::grect_contains_point(self, point)
    }

    pub fn center_point(&self) -> GPoint {
        interface::grect_center_point(self)
    }

    pub fn crop(&self, crop_size_px: i32) -> GRect {
        interface::grect_crop(*self, crop_size_px)
    }
}
