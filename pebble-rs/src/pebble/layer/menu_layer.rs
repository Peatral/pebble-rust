pub mod menu_cell_layer;
pub mod menu_index;

use crate::graphics::context::Context;
use crate::layer::{ILayer, ILayerMut, LayerRef};
use crate::pebble::window::WindowRef;
use alloc::boxed::Box;
use core::ffi::c_void;
use core::ops::{Deref, DerefMut};

use crate::graphics::types::Rect;
pub use menu_cell_layer::MenuCellLayer;
pub use menu_index::MenuIndexRef;
use pebble_sys::Layer;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct MenuLayerRef {
    internal: *mut pebble_sys::MenuLayer,
}

impl MenuLayerRef {
    pub fn set_click_config_onto_window(&self, window: &WindowRef) {
        unsafe {
            pebble_sys::menu_layer_set_click_config_onto_window(self.internal, window.as_ptr());
        }
    }

    pub fn reload_data(&self) {
        unsafe {
            pebble_sys::menu_layer_reload_data(self.internal);
        }
    }

    pub fn set_selected_next(
        &self,
        up: bool,
        scroll_align: pebble_sys::MenuRowAlign,
        animated: bool,
    ) {
        unsafe {
            pebble_sys::menu_layer_set_selected_next(self.internal, up, scroll_align, animated);
        }
    }

    pub fn set_selected_index(
        &self,
        index: pebble_sys::MenuIndex,
        scroll_align: pebble_sys::MenuRowAlign,
        animated: bool,
    ) {
        unsafe {
            pebble_sys::menu_layer_set_selected_index(self.internal, index, scroll_align, animated);
        }
    }

    pub fn get_selected_index(&self) -> pebble_sys::MenuIndex {
        unsafe { pebble_sys::menu_layer_get_selected_index(self.internal) }
    }

    pub fn set_normal_colors(
        &self,
        background: pebble_sys::GColor,
        foreground: pebble_sys::GColor,
    ) {
        unsafe {
            pebble_sys::menu_layer_set_normal_colors(self.internal, background, foreground);
        }
    }

    pub fn set_highlight_colors(
        &self,
        background: pebble_sys::GColor,
        foreground: pebble_sys::GColor,
    ) {
        unsafe {
            pebble_sys::menu_layer_set_highlight_colors(self.internal, background, foreground);
        }
    }

    pub fn pad_bottom_enable(&self, enable: bool) {
        unsafe {
            pebble_sys::menu_layer_pad_bottom_enable(self.internal, enable);
        }
    }

    pub fn set_center_focused(&self, center_focused: bool) {
        unsafe {
            pebble_sys::menu_layer_set_center_focused(self.internal, center_focused);
        }
    }

    pub fn get_center_focused(&self) -> bool {
        unsafe { pebble_sys::menu_layer_get_center_focused(self.internal) }
    }

    pub fn is_index_selected(&self, index: MenuIndexRef) -> bool {
        unsafe { pebble_sys::menu_layer_is_index_selected(self.internal, index.as_ptr()) }
    }
}

impl ILayer for MenuLayerRef {
    fn as_ptr(&self) -> *const Layer {
        unsafe { pebble_sys::menu_layer_get_layer(self.internal) }
    }
}

impl ILayerMut for MenuLayerRef {
    fn as_mut_ptr(&self) -> *mut Layer {
        unsafe { pebble_sys::menu_layer_get_layer(self.internal) }
    }
}

impl From<*mut pebble_sys::MenuLayer> for MenuLayerRef {
    fn from(internal: *mut pebble_sys::MenuLayer) -> Self {
        Self { internal }
    }
}

pub struct MenuLayer<T: MenuLayerDelegate> {
    layer_ref: MenuLayerRef,
    delegate: Box<T>,
}

pub trait MenuLayerDelegate {
    fn get_num_sections(&self, _menu_layer: MenuLayerRef) -> u16 {
        1
    }
    fn get_num_rows(&self, menu_layer: MenuLayerRef, section_index: u16) -> u16;
    fn get_cell_height(&self, _menu_layer: MenuLayerRef, _cell_index: MenuIndexRef) -> i16 {
        44
    }
    fn get_header_height(&self, _menu_layer: MenuLayerRef, _section_index: u16) -> i16 {
        0
    }
    fn get_separator_height(&self, _menu_layer: MenuLayerRef, _cell_index: MenuIndexRef) -> i16 {
        0
    }
    fn draw_row(&self, ctx: Context, cell_layer: MenuCellLayer, cell_index: MenuIndexRef);
    fn draw_header(&self, _ctx: Context, _cell_layer: MenuCellLayer, _section_index: u16) {}
    fn draw_separator(&self, _ctx: Context, _cell_layer: MenuCellLayer, _cell_index: MenuIndexRef) {
    }
    fn draw_background(&self, _ctx: Context, _bg_layer: LayerRef, _highlight: bool) {}
    fn select_click(&self, _menu_layer: MenuLayerRef, _cell_index: MenuIndexRef) {}
    fn select_long_click(&self, _menu_layer: MenuLayerRef, _cell_index: MenuIndexRef) {}
    fn selection_changed(
        &self,
        _menu_layer: MenuLayerRef,
        _new_index: pebble_sys::MenuIndex,
        _old_index: pebble_sys::MenuIndex,
    ) {
    }
    fn selection_will_change(
        &self,
        _menu_layer: MenuLayerRef,
        _new_index: MenuIndexRef,
        _old_index: pebble_sys::MenuIndex,
    ) {
    }
}

extern "C" fn trampoline_get_num_sections<T: MenuLayerDelegate>(
    layer: *mut pebble_sys::MenuLayer,
    ctx: *mut c_void,
) -> u16 {
    unsafe {
        let delegate = &*(ctx as *const T);
        delegate.get_num_sections(layer.into())
    }
}

extern "C" fn trampoline_get_num_rows<T: MenuLayerDelegate>(
    layer: *mut pebble_sys::MenuLayer,
    section_index: u16,
    ctx: *mut c_void,
) -> u16 {
    unsafe {
        let delegate = &*(ctx as *const T);
        delegate.get_num_rows(layer.into(), section_index)
    }
}

extern "C" fn trampoline_get_cell_height<T: MenuLayerDelegate>(
    layer: *mut pebble_sys::MenuLayer,
    cell_index: *mut pebble_sys::MenuIndex,
    ctx: *mut c_void,
) -> i16 {
    unsafe {
        let delegate = &*(ctx as *const T);
        delegate.get_cell_height(layer.into(), cell_index.into())
    }
}

extern "C" fn trampoline_get_header_height<T: MenuLayerDelegate>(
    layer: *mut pebble_sys::MenuLayer,
    section_index: u16,
    ctx: *mut c_void,
) -> i16 {
    unsafe {
        let delegate = &*(ctx as *const T);
        delegate.get_header_height(layer.into(), section_index)
    }
}

extern "C" fn trampoline_get_separator_height<T: MenuLayerDelegate>(
    layer: *mut pebble_sys::MenuLayer,
    cell_index: *mut pebble_sys::MenuIndex,
    ctx: *mut c_void,
) -> i16 {
    unsafe {
        let delegate = &*(ctx as *const T);
        delegate.get_separator_height(layer.into(), cell_index.into())
    }
}

extern "C" fn trampoline_draw_row<T: MenuLayerDelegate>(
    ctx: *mut pebble_sys::GContext,
    cell_layer: *const Layer,
    cell_index: *mut pebble_sys::MenuIndex,
    callback_context: *mut c_void,
) {
    unsafe {
        let delegate = &*(callback_context as *const T);
        delegate.draw_row(ctx.into(), cell_layer.into(), cell_index.into())
    }
}

extern "C" fn trampoline_draw_header<T: MenuLayerDelegate>(
    ctx: *mut pebble_sys::GContext,
    cell_layer: *const pebble_sys::Layer,
    section_index: u16,
    callback_context: *mut c_void,
) {
    unsafe {
        let delegate = &*(callback_context as *const T);
        delegate.draw_header(ctx.into(), cell_layer.into(), section_index)
    }
}

extern "C" fn trampoline_draw_separator<T: MenuLayerDelegate>(
    ctx: *mut pebble_sys::GContext,
    cell_layer: *const Layer,
    cell_index: *mut pebble_sys::MenuIndex,
    callback_context: *mut c_void,
) {
    unsafe {
        let delegate = &*(callback_context as *const T);
        delegate.draw_separator(ctx.into(), cell_layer.into(), cell_index.into())
    }
}

extern "C" fn trampoline_draw_background<T: MenuLayerDelegate>(
    ctx: *mut pebble_sys::GContext,
    bg_layer: *const Layer,
    highlight: bool,
    callback_context: *mut c_void,
) {
    unsafe {
        let delegate = &*(callback_context as *const T);
        delegate.draw_background(ctx.into(), bg_layer.into(), highlight)
    }
}

extern "C" fn trampoline_select_click<T: MenuLayerDelegate>(
    layer: *mut pebble_sys::MenuLayer,
    cell_index: *mut pebble_sys::MenuIndex,
    ctx: *mut c_void,
) {
    unsafe {
        let delegate = &*(ctx as *const T);
        delegate.select_click(layer.into(), cell_index.into())
    }
}

extern "C" fn trampoline_select_long_click<T: MenuLayerDelegate>(
    layer: *mut pebble_sys::MenuLayer,
    cell_index: *mut pebble_sys::MenuIndex,
    ctx: *mut c_void,
) {
    unsafe {
        let delegate = &*(ctx as *const T);
        delegate.select_long_click(layer.into(), cell_index.into())
    }
}

extern "C" fn trampoline_selection_changed<T: MenuLayerDelegate>(
    layer: *mut pebble_sys::MenuLayer,
    new_index: pebble_sys::MenuIndex,
    old_index: pebble_sys::MenuIndex,
    ctx: *mut c_void,
) {
    unsafe {
        let delegate = &*(ctx as *const T);
        delegate.selection_changed(layer.into(), new_index, old_index)
    }
}

extern "C" fn trampoline_selection_will_change<T: MenuLayerDelegate>(
    layer: *mut pebble_sys::MenuLayer,
    new_index: *mut pebble_sys::MenuIndex,
    old_index: pebble_sys::MenuIndex,
    ctx: *mut c_void,
) {
    unsafe {
        let delegate = &*(ctx as *const T);
        delegate.selection_will_change(layer.into(), new_index.into(), old_index)
    }
}

impl<T: MenuLayerDelegate> MenuLayer<T> {
    pub fn new(bounds: Rect, delegate: T) -> Self {
        unsafe {
            let internal = pebble_sys::menu_layer_create(bounds.0);

            let layer = MenuLayer {
                layer_ref: internal.into(),
                delegate: Box::new(delegate),
            };

            let context_ptr = &*layer.delegate as *const T as *mut c_void;

            let callbacks = pebble_sys::MenuLayerCallbacks {
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

            pebble_sys::menu_layer_set_callbacks(layer.internal, context_ptr, callbacks);

            layer
        }
    }
}

impl<T: MenuLayerDelegate> ILayer for MenuLayer<T> {
    fn as_ptr(&self) -> *const Layer {
        self.layer_ref.as_ptr()
    }
}

impl<T: MenuLayerDelegate> ILayerMut for MenuLayer<T> {
    fn as_mut_ptr(&self) -> *mut Layer {
        self.layer_ref.as_mut_ptr()
    }
}

impl<T: MenuLayerDelegate> Deref for MenuLayer<T> {
    type Target = MenuLayerRef;

    fn deref(&self) -> &Self::Target {
        &self.layer_ref
    }
}

impl<T: MenuLayerDelegate> DerefMut for MenuLayer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.layer_ref
    }
}

impl<T: MenuLayerDelegate> Drop for MenuLayer<T> {
    fn drop(&mut self) {
        unsafe {
            pebble_sys::menu_layer_destroy(self.internal);
        }
    }
}
