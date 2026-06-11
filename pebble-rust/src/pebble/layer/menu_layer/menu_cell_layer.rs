use crate::graphics::context::GContext;
use crate::layer::ILayer;
use crate::pebble::internal::functions::interface;
use crate::pebble::internal::types;
use core::ffi::CStr;

/// A safe wrapper representing a single menu cell layer during a draw callback.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct MenuCellLayer {
    internal: *const types::Layer,
}

impl MenuCellLayer {
    /// Draws a basic section cell with a title, subtitle, and optional icon.
    pub fn draw_basic(
        &self,
        ctx: GContext,
        title: &CStr,
        subtitle: &CStr,
        icon: *mut types::GBitmap,
    ) {
        interface::menu_cell_basic_draw(ctx.as_ptr(), self.internal, title, subtitle, icon);
    }

    /// Draws a cell layout with only one big title.
    pub fn draw_title(&self, ctx: GContext, title: &CStr) {
        interface::menu_cell_title_draw(ctx.as_ptr(), self.internal, title);
    }

    /// Draws a basic section header cell layout with the title.
    pub fn draw_basic_header(&self, ctx: GContext, title: &CStr) {
        interface::menu_cell_basic_header_draw(ctx.as_ptr(), self.internal, title);
    }

    /// Returns whether or not this cell layer is currently highlighted.
    pub fn is_highlighted(&self) -> bool {
        interface::menu_cell_layer_is_highlighted(self.internal)
    }
}

impl ILayer for MenuCellLayer {
    fn as_ptr(&self) -> *const types::Layer {
        self.internal
    }
}

impl From<*const types::Layer> for MenuCellLayer {
    fn from(internal: *const types::Layer) -> Self {
        Self { internal }
    }
}
