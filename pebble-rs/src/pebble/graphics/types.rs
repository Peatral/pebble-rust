use core::cmp::Eq;
use core::ops::{Deref, DerefMut};
use pebble_sys::{GOvalScaleMode};

#[derive(Clone, Copy)]
pub struct GColor8(pub(crate) pebble_sys::GColor8); // Added pub to .0 for easier access

impl GColor8 {
    pub const fn new(argb: u8) -> Self {
        Self(pebble_sys::GColor8 { argb })
    }

    pub fn legible_over(background_color: GColor8) -> Self {
        unsafe {
            Self(pebble_sys::gcolor_legible_over(background_color.0))
        }
    }

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let a_bits = 0b1100_0000;
        let r_bits = (r >> 6) << 4;
        let g_bits = (g >> 6) << 2;
        let b_bits = b >> 6;
        Self(pebble_sys::GColor8 { argb: a_bits | r_bits | g_bits | b_bits })
    }

    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        let a_bits = (a >> 6) << 6;
        let r_bits = (r >> 6) << 4;
        let g_bits = (g >> 6) << 2;
        let b_bits = b >> 6;
        Self(pebble_sys::GColor8 { argb: a_bits | r_bits | g_bits | b_bits })
    }

    pub const fn from_hex(hex: u32) -> Self {
        Self::from_rgb(((hex >> 16) & 0xFF) as u8, ((hex >> 8) & 0xFF) as u8, (hex & 0xFF) as u8)
    }
}

impl PartialEq for GColor8 {
    fn eq(&self, other: &Self) -> bool {
        let self_argb = unsafe { self.argb };
        let other_argb = unsafe { other.argb };
        if self_argb == other_argb {
            return true;
        }
        let alpha_mask = 0b1100_0000;
        (self_argb | other_argb) & alpha_mask == 0
    }
}

impl Eq for GColor8 {}

impl From<pebble_sys::GColor8> for GColor8 {
    fn from(raw: pebble_sys::GColor8) -> Self { Self(raw) }
}
impl Deref for GColor8 {
    type Target = pebble_sys::GColor8;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for GColor8 {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

#[derive(Clone, Copy)]
pub struct GPoint(pub(crate) pebble_sys::GPoint);

impl GPoint {
    pub const fn new(x: i16, y: i16) -> Self {
        Self(pebble_sys::GPoint { x, y })
    }

    pub fn from_polar(rect: GRect, scale_mode: GOvalScaleMode, angle: i32) -> GPoint {
        unsafe { GPoint(pebble_sys::gpoint_from_polar(rect.0, scale_mode, angle)) }
    }
}

impl From<pebble_sys::GPoint> for GPoint {
    fn from(raw: pebble_sys::GPoint) -> Self { Self(raw) }
}
impl Deref for GPoint {
    type Target = pebble_sys::GPoint;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for GPoint {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

#[derive(Clone, Copy)]
pub struct GSize(pub(crate) pebble_sys::GSize);

impl GSize {
    pub const fn new(w: i16, h: i16) -> Self {
        Self(pebble_sys::GSize { w, h })
    }
}

impl From<pebble_sys::GSize> for GSize {
    fn from(raw: pebble_sys::GSize) -> Self { Self(raw) }
}
impl Deref for GSize {
    type Target = pebble_sys::GSize;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for GSize {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

#[derive(Clone, Copy)]
pub struct GRect(pub(crate) pebble_sys::GRect);

impl GRect {
    pub const fn new(origin: GPoint, size: GSize) -> Self {
        Self(pebble_sys::GRect { origin: origin.0, size: size.0 })
    }

    pub fn centered_from_polar(rect: GRect, scale_mode: GOvalScaleMode, angle: i32, size: GSize) -> GRect {
        unsafe { GRect(pebble_sys::grect_centered_from_polar(rect.0, scale_mode, angle, size.0)) }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { pebble_sys::grect_is_empty(&self.0) }
    }

    pub fn standardize(&mut self) {
        unsafe { pebble_sys::grect_standardize(&mut self.0) }
    }

    pub fn clip(&mut self, clipper: &GRect) {
        unsafe { pebble_sys::grect_clip(&mut self.0, &clipper.0) }
    }

    pub fn contains_point(&self, point: &GPoint) -> bool {
        unsafe { pebble_sys::grect_contains_point(&self.0, &point.0) }
    }

    pub fn center_point(&self) -> GPoint {
        unsafe { GPoint(pebble_sys::grect_center_point(&self.0)) }
    }

    pub fn crop(&self, crop_size_px: i32) -> GRect {
        unsafe { GRect(pebble_sys::grect_crop(self.0, crop_size_px)) }
    }
}

impl From<pebble_sys::GRect> for GRect {
    fn from(raw: pebble_sys::GRect) -> Self { Self(raw) }
}
impl Deref for GRect {
    type Target = pebble_sys::GRect;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for GRect {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}