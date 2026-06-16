use crate::graphics::context::Context;
use crate::graphics::types::{Point, Rect, Size};
use crate::system::fonts::Font;
use core::ffi::CStr;
use core::ptr;
use pebble_sys::{GTextAlignment, GTextOverflowMode};

#[repr(transparent)]
pub struct TextAttributes {
    internal: *mut pebble_sys::GTextAttributes,
}

impl TextAttributes {
    pub fn new() -> Self {
        let ptr = unsafe { pebble_sys::graphics_text_attributes_create() };
        Self { internal: ptr }
    }

    /// Restores text flow to the rectangular default.
    pub fn restore_default_text_flow(&self) {
        unsafe {
            pebble_sys::graphics_text_attributes_restore_default_text_flow(self.internal);
        }
    }

    /// Enables text flow that follows the boundaries of the screen.
    pub fn enable_screen_text_flow(&self, inset: u8) {
        unsafe {
            pebble_sys::graphics_text_attributes_enable_screen_text_flow(self.internal, inset);
        }
    }

    /// Restores paging and locked content origin to the defaults.
    pub fn restore_default_paging(&self) {
        unsafe {
            pebble_sys::graphics_text_attributes_restore_default_paging(self.internal);
        }
    }

    /// Enables paging and locks the text flow calculation to a fixed point on the screen.
    pub fn enable_paging(&self, content_origin_on_screen: Point, paging_on_screen: Rect) {
        unsafe {
            pebble_sys::graphics_text_attributes_enable_paging(
                self.internal,
                content_origin_on_screen.0,
                paging_on_screen.0,
            );
        }
    }

    #[inline(always)]
    pub(crate) fn as_ptr(&self) -> *mut pebble_sys::GTextAttributes {
        self.internal
    }
}

impl Drop for TextAttributes {
    fn drop(&mut self) {
        if !self.internal.is_null() {
            unsafe {
                pebble_sys::graphics_text_attributes_destroy(self.internal);
            }
        }
    }
}

impl Context {
    /// Safely draws text into the current graphics context.
    pub fn draw_text(
        &self,
        text: &CStr,
        font: &Font,
        box_rect: Rect,
        overflow_mode: GTextOverflowMode,
        alignment: GTextAlignment,
        text_attributes: Option<&TextAttributes>,
    ) {
        unsafe {
            pebble_sys::graphics_draw_text(
                self.as_ptr(),
                text.as_ptr(),
                font.internal,
                box_rect.0,
                overflow_mode,
                alignment,
                text_attributes.map_or(ptr::null_mut(), |attr| attr.as_ptr()),
            );
        }
    }

    /// Calculates the maximum size that a text occupies within a rectangular constraint.
    pub fn text_layout_get_content_size(
        text: &CStr,
        font: &Font,
        box_rect: Rect,
        overflow_mode: GTextOverflowMode,
        alignment: GTextAlignment,
        text_attributes: Option<&TextAttributes>,
    ) -> Size {
        if let Some(attributes) = text_attributes {
            unsafe {
                Size(
                    pebble_sys::graphics_text_layout_get_content_size_with_attributes(
                        text.as_ptr(),
                        font.internal,
                        box_rect.0,
                        overflow_mode,
                        alignment,
                        attributes.as_ptr(),
                    ),
                )
            }
        } else {
            unsafe {
                Size(pebble_sys::graphics_text_layout_get_content_size(
                    text.as_ptr(),
                    font.internal,
                    box_rect.0,
                    overflow_mode,
                    alignment,
                ))
            }
        }
    }
}
