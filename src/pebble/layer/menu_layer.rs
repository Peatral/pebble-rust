use crate::graphics::context::GContext;
use crate::layer::ILayer;
use crate::pebble::internal::functions::interface;
use crate::pebble::internal::types;
use crate::pebble::window::WindowRef;
use alloc::boxed::Box;
use core::ffi::{CStr, c_void};

/// Helper to draw a basic section cell with a title, subtitle, and optional icon.
pub fn cell_basic_draw(
    ctx: GContext,
    cell_layer: *const types::Layer,
    title: &CStr,
    subtitle: &CStr,
    icon: *mut types::GBitmap,
) {
    interface::menu_cell_basic_draw(ctx.as_ptr(), cell_layer, title, subtitle, icon);
}

/// Helper to draw a cell layout with only one big title.
pub fn cell_title_draw(ctx: GContext, cell_layer: *const types::Layer, title: &CStr) {
    interface::menu_cell_title_draw(ctx.as_ptr(), cell_layer, title);
}

/// Helper to draw a basic section header cell layout with the title.
pub fn cell_basic_header_draw(ctx: GContext, cell_layer: *const types::Layer, title: &CStr) {
    interface::menu_cell_basic_header_draw(ctx.as_ptr(), cell_layer, title);
}

/// Returns whether or not the given cell layer is highlighted.
pub fn cell_layer_is_highlighted(cell_layer: *const types::Layer) -> bool {
    interface::menu_cell_layer_is_highlighted(cell_layer)
}

/// Comparator function to determine the order of two MenuIndex values.
pub fn index_compare(a: &types::MenuIndex, b: &types::MenuIndex) -> i16 {
    interface::menu_index_compare(a as *const types::MenuIndex, b as *const types::MenuIndex)
}

pub struct MenuLayer<T: MenuLayerDelegate> {
    internal: *mut types::MenuLayer,
    inner: *mut types::Layer,
    delegate: Box<T>,
}

pub struct MenuLayerRef {
    internal: *mut types::MenuLayer,
}

impl MenuLayerRef {
    /// Create a new ref from a raw pointer (used internally by trampolines)
    pub(crate) fn from_ptr(internal: *mut types::MenuLayer) -> Self {
        Self { internal }
    }

    /// Safely applies the click config onto the target Window by extracting the raw pointer from the WindowRef.
    pub fn set_click_config_onto_window(&self, window: &WindowRef) {
        interface::menu_layer_set_click_config_onto_window(self.internal, window.as_ptr());
    }

    pub fn reload_data(&self) {
        interface::menu_layer_reload_data(self.internal);
    }

    pub fn set_selected_next(&self, up: bool, scroll_align: types::MenuRowAlign, animated: bool) {
        interface::menu_layer_set_selected_next(self.internal, up, scroll_align, animated);
    }

    pub fn set_selected_index(
        &self,
        index: types::MenuIndex,
        scroll_align: types::MenuRowAlign,
        animated: bool,
    ) {
        interface::menu_layer_set_selected_index(self.internal, index, scroll_align, animated);
    }

    pub fn get_selected_index(&self) -> types::MenuIndex {
        interface::menu_layer_get_selected_index(self.internal)
    }

    pub fn set_normal_colors(&self, background: types::GColor, foreground: types::GColor) {
        interface::menu_layer_set_normal_colors(self.internal, background, foreground);
    }

    pub fn set_highlight_colors(&self, background: types::GColor, foreground: types::GColor) {
        interface::menu_layer_set_highlight_colors(self.internal, background, foreground);
    }

    pub fn pad_bottom_enable(&self, enable: bool) {
        interface::menu_layer_pad_bottom_enable(self.internal, enable);
    }

    pub fn set_center_focused(&self, center_focused: bool) {
        interface::menu_layer_set_center_focused(self.internal, center_focused);
    }

    pub fn get_center_focused(&self) -> bool {
        interface::menu_layer_get_center_focused(self.internal)
    }

    pub fn is_index_selected(&self, index: &types::MenuIndex) -> bool {
        interface::menu_layer_is_index_selected(self.internal, index as *const types::MenuIndex)
    }
}

pub trait MenuLayerDelegate {
    fn get_num_sections(&self, _menu_layer: MenuLayerRef) -> u16 {
        1
    }
    fn get_num_rows(&self, menu_layer: MenuLayerRef, section_index: u16) -> u16;
    fn get_cell_height(
        &self,
        _menu_layer: MenuLayerRef,
        _cell_index: *mut types::MenuIndex,
    ) -> i16 {
        44
    }
    fn get_header_height(&self, _menu_layer: MenuLayerRef, _section_index: u16) -> i16 {
        0
    }
    fn get_separator_height(
        &self,
        _menu_layer: MenuLayerRef,
        _cell_index: *mut types::MenuIndex,
    ) -> i16 {
        0
    }
    fn draw_row(
        &self,
        ctx: GContext,
        cell_layer: *const types::Layer,
        cell_index: *mut types::MenuIndex,
    );
    fn draw_header(&self, _ctx: GContext, _cell_layer: *const types::Layer, _section_index: u16) {}
    fn draw_separator(
        &self,
        _ctx: GContext,
        _cell_layer: *const types::Layer,
        _cell_index: *mut types::MenuIndex,
    ) {
    }
    fn draw_background(&self, _ctx: GContext, _bg_layer: *const types::Layer, _highlight: bool) {}
    fn select_click(&self, _menu_layer: MenuLayerRef, _cell_index: *mut types::MenuIndex) {}
    fn select_long_click(&self, _menu_layer: MenuLayerRef, _cell_index: *mut types::MenuIndex) {}
    fn selection_changed(
        &self,
        _menu_layer: MenuLayerRef,
        _new_index: types::MenuIndex,
        _old_index: types::MenuIndex,
    ) {
    }
    fn selection_will_change(
        &self,
        _menu_layer: MenuLayerRef,
        _new_index: *mut types::MenuIndex,
        _old_index: types::MenuIndex,
    ) {
    }
}

extern "C" fn trampoline_get_num_sections<T: MenuLayerDelegate>(
    layer: *mut types::MenuLayer,
    ctx: *mut c_void,
) -> u16 {
    let delegate = unsafe { &*(ctx as *const T) };
    delegate.get_num_sections(MenuLayerRef::from_ptr(layer))
}
extern "C" fn trampoline_get_num_rows<T: MenuLayerDelegate>(
    layer: *mut types::MenuLayer,
    section_index: u16,
    ctx: *mut c_void,
) -> u16 {
    let delegate = unsafe { &*(ctx as *const T) };
    delegate.get_num_rows(MenuLayerRef::from_ptr(layer), section_index)
}
extern "C" fn trampoline_get_cell_height<T: MenuLayerDelegate>(
    layer: *mut types::MenuLayer,
    cell_index: *mut types::MenuIndex,
    ctx: *mut c_void,
) -> i16 {
    let delegate = unsafe { &*(ctx as *const T) };
    delegate.get_cell_height(MenuLayerRef::from_ptr(layer), cell_index)
}
extern "C" fn trampoline_get_header_height<T: MenuLayerDelegate>(
    layer: *mut types::MenuLayer,
    section_index: u16,
    ctx: *mut c_void,
) -> i16 {
    let delegate = unsafe { &*(ctx as *const T) };
    delegate.get_header_height(MenuLayerRef::from_ptr(layer), section_index)
}
extern "C" fn trampoline_get_separator_height<T: MenuLayerDelegate>(
    layer: *mut types::MenuLayer,
    cell_index: *mut types::MenuIndex,
    ctx: *mut c_void,
) -> i16 {
    let delegate = unsafe { &*(ctx as *const T) };
    delegate.get_separator_height(MenuLayerRef::from_ptr(layer), cell_index)
}
extern "C" fn trampoline_draw_row<T: MenuLayerDelegate>(
    ctx: *mut types::GContext,
    cell_layer: *const types::Layer,
    cell_index: *mut types::MenuIndex,
    callback_context: *mut c_void,
) {
    let delegate = unsafe { &*(callback_context as *const T) };
    delegate.draw_row(GContext::from_ptr(ctx), cell_layer, cell_index)
}
extern "C" fn trampoline_draw_header<T: MenuLayerDelegate>(
    ctx: *mut types::GContext,
    cell_layer: *const types::Layer,
    section_index: u16,
    callback_context: *mut c_void,
) {
    let delegate = unsafe { &*(callback_context as *const T) };
    delegate.draw_header(GContext::from_ptr(ctx), cell_layer, section_index)
}
extern "C" fn trampoline_draw_separator<T: MenuLayerDelegate>(
    ctx: *mut types::GContext,
    cell_layer: *const types::Layer,
    cell_index: *mut types::MenuIndex,
    callback_context: *mut c_void,
) {
    let delegate = unsafe { &*(callback_context as *const T) };
    delegate.draw_separator(GContext::from_ptr(ctx), cell_layer, cell_index)
}
extern "C" fn trampoline_draw_background<T: MenuLayerDelegate>(
    ctx: *mut types::GContext,
    bg_layer: *const types::Layer,
    highlight: bool,
    callback_context: *mut c_void,
) {
    let delegate = unsafe { &*(callback_context as *const T) };
    delegate.draw_background(GContext::from_ptr(ctx), bg_layer, highlight)
}
extern "C" fn trampoline_select_click<T: MenuLayerDelegate>(
    layer: *mut types::MenuLayer,
    cell_index: *mut types::MenuIndex,
    ctx: *mut c_void,
) {
    let delegate = unsafe { &*(ctx as *const T) };
    delegate.select_click(MenuLayerRef::from_ptr(layer), cell_index)
}
extern "C" fn trampoline_select_long_click<T: MenuLayerDelegate>(
    layer: *mut types::MenuLayer,
    cell_index: *mut types::MenuIndex,
    ctx: *mut c_void,
) {
    let delegate = unsafe { &*(ctx as *const T) };
    delegate.select_long_click(MenuLayerRef::from_ptr(layer), cell_index)
}
extern "C" fn trampoline_selection_changed<T: MenuLayerDelegate>(
    layer: *mut types::MenuLayer,
    new_index: types::MenuIndex,
    old_index: types::MenuIndex,
    ctx: *mut c_void,
) {
    let delegate = unsafe { &*(ctx as *const T) };
    delegate.selection_changed(MenuLayerRef::from_ptr(layer), new_index, old_index)
}
extern "C" fn trampoline_selection_will_change<T: MenuLayerDelegate>(
    layer: *mut types::MenuLayer,
    new_index: *mut types::MenuIndex,
    old_index: types::MenuIndex,
    ctx: *mut c_void,
) {
    let delegate = unsafe { &*(ctx as *const T) };
    delegate.selection_will_change(MenuLayerRef::from_ptr(layer), new_index, old_index)
}

impl<T: MenuLayerDelegate> MenuLayer<T> {
    pub fn new(bounds: types::GRect, delegate: T) -> Self {
        let internal = interface::menu_layer_create(bounds);
        let inner = interface::menu_layer_get_layer(internal);

        let layer = MenuLayer {
            internal,
            inner,
            delegate: Box::new(delegate),
        };

        let context_ptr = &*layer.delegate as *const T as *mut c_void;

        let callbacks = types::MenuLayerCallbacks {
            get_num_sections: Some(trampoline_get_num_sections::<T>),
            get_num_rows: Some(trampoline_get_num_rows::<T>),
            get_cell_height: Some(trampoline_get_cell_height::<T>),
            get_header_height: Some(trampoline_get_header_height::<T>),
            draw_row: Some(trampoline_draw_row::<T>),
            draw_header: Some(trampoline_draw_header::<T>),
            select_click: Some(trampoline_select_click::<T>),
            select_long_click: Some(trampoline_select_long_click::<T>),
            selection_changed: Some(trampoline_selection_changed::<T>),
            get_separator_height: Some(trampoline_get_separator_height::<T>),
            draw_separator: Some(trampoline_draw_separator::<T>),
            selection_will_change: Some(trampoline_selection_will_change::<T>),
            draw_background: Some(trampoline_draw_background::<T>),
        };

        interface::menu_layer_set_callbacks(layer.internal, context_ptr, callbacks);

        layer
    }

    pub fn as_ref(&self) -> MenuLayerRef {
        MenuLayerRef::from_ptr(self.internal)
    }

    /// Safely applies the click config onto the target Window by extracting the raw pointer from the WindowRef.
    pub fn set_click_config_onto_window(&self, window: &WindowRef) {
        self.as_ref().set_click_config_onto_window(window);
    }

    pub fn reload_data(&self) {
        self.as_ref().reload_data();
    }

    pub fn set_selected_next(&self, up: bool, scroll_align: types::MenuRowAlign, animated: bool) {
        self.as_ref().set_selected_next(up, scroll_align, animated);
    }

    pub fn set_selected_index(
        &self,
        index: types::MenuIndex,
        scroll_align: types::MenuRowAlign,
        animated: bool,
    ) {
        self.as_ref()
            .set_selected_index(index, scroll_align, animated);
    }

    pub fn get_selected_index(&self) -> types::MenuIndex {
        self.as_ref().get_selected_index()
    }

    pub fn set_normal_colors(&self, background: types::GColor, foreground: types::GColor) {
        self.as_ref().set_normal_colors(background, foreground);
    }

    pub fn set_highlight_colors(&self, background: types::GColor, foreground: types::GColor) {
        self.as_ref().set_highlight_colors(background, foreground);
    }

    pub fn pad_bottom_enable(&self, enable: bool) {
        self.as_ref().pad_bottom_enable(enable);
    }

    pub fn set_center_focused(&self, center_focused: bool) {
        self.as_ref().set_center_focused(center_focused);
    }

    pub fn get_center_focused(&self) -> bool {
        self.as_ref().get_center_focused()
    }

    pub fn is_index_selected(&self, index: &types::MenuIndex) -> bool {
        self.as_ref().is_index_selected(index)
    }
}

impl<T: MenuLayerDelegate> Drop for MenuLayer<T> {
    fn drop(&mut self) {
        interface::menu_layer_destroy(self.internal);
    }
}

impl<T: MenuLayerDelegate> ILayer for MenuLayer<T> {
    fn get_bounds(&self) -> types::GRect {
        interface::layer_get_bounds(self.inner)
    }

    fn get_frame(&self) -> types::GRect {
        interface::layer_get_frame(self.inner)
    }

    fn add_child(&self, layer: &dyn ILayer) {
        interface::layer_add_child(self.inner, layer.get_internal())
    }

    fn mark_dirty(&self) {
        interface::layer_mark_dirty(self.inner)
    }

    fn get_internal(&self) -> *mut types::Layer {
        self.inner
    }
}
