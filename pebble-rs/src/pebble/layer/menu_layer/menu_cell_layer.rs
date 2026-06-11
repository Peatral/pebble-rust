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
        subtitle: &CStr,
        icon: *mut pebble_sys::GBitmap,
    ) {
        unsafe {
            pebble_sys::menu_cell_basic_draw(
                ctx.as_ptr(),
                self.internal,
                title.as_ptr(),
                subtitle.as_ptr(),
                icon,
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
