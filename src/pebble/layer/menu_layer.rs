use crate::layer::ILayer;
use crate::pebble::internal::functions::interface;
use crate::pebble::internal::types;
use crate::pebble::internal::types::{c_void, WindowPtr};
use alloc::boxed::Box;

pub struct MenuLayer<T: MenuLayerDelegate> {
    internal: *mut types::MenuLayer,
    inner: *mut types::Layer,
    delegate: Box<T>,
}

pub trait MenuLayerDelegate {
    fn get_num_sections(&self, _menu_layer: *mut types::MenuLayer) -> u16 {
        1
    }

    fn get_num_rows(&self, menu_layer: *mut types::MenuLayer, section_index: u16) -> u16;

    fn get_cell_height(
        &self,
        _menu_layer: *mut types::MenuLayer,
        _cell_index: *mut types::MenuIndex,
    ) -> i16 {
        44
    }

    fn get_header_height(&self, _menu_layer: *mut types::MenuLayer, _section_index: u16) -> i16 {
        0
    }

    fn get_separator_height(
        &self,
        _menu_layer: *mut types::MenuLayer,
        _cell_index: *mut types::MenuIndex,
    ) -> i16 {
        0
    }

    fn draw_row(
        &self,
        ctx: *mut types::GContext,
        cell_layer: *const types::Layer,
        cell_index: *mut types::MenuIndex,
    );

    fn draw_header(
        &self,
        _ctx: *mut types::GContext,
        _cell_layer: *const types::Layer,
        _section_index: u16,
    ) {
    }

    fn draw_separator(
        &self,
        _ctx: *mut types::GContext,
        _cell_layer: *const types::Layer,
        _cell_index: *mut types::MenuIndex,
    ) {
    }

    fn draw_background(
        &self,
        _ctx: *mut types::GContext,
        _bg_layer: *const types::Layer,
        _highlight: bool,
    ) {
    }

    fn select_click(&self, _menu_layer: *mut types::MenuLayer, _cell_index: *mut types::MenuIndex) {
    }

    fn select_long_click(
        &self,
        _menu_layer: *mut types::MenuLayer,
        _cell_index: *mut types::MenuIndex,
    ) {
    }

    fn selection_changed(
        &self,
        _menu_layer: *mut types::MenuLayer,
        _new_index: types::MenuIndex,
        _old_index: types::MenuIndex,
    ) {
    }

    fn selection_will_change(
        &self,
        _menu_layer: *mut types::MenuLayer,
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
    delegate.get_num_sections(layer)
}

extern "C" fn trampoline_get_num_rows<T: MenuLayerDelegate>(
    layer: *mut types::MenuLayer,
    section_index: u16,
    ctx: *mut c_void,
) -> u16 {
    let delegate = unsafe { &*(ctx as *const T) };
    delegate.get_num_rows(layer, section_index)
}

extern "C" fn trampoline_get_cell_height<T: MenuLayerDelegate>(
    layer: *mut types::MenuLayer,
    cell_index: *mut types::MenuIndex,
    ctx: *mut c_void,
) -> i16 {
    let delegate = unsafe { &*(ctx as *const T) };
    delegate.get_cell_height(layer, cell_index)
}

extern "C" fn trampoline_get_header_height<T: MenuLayerDelegate>(
    layer: *mut types::MenuLayer,
    section_index: u16,
    ctx: *mut c_void,
) -> i16 {
    let delegate = unsafe { &*(ctx as *const T) };
    delegate.get_header_height(layer, section_index)
}

extern "C" fn trampoline_get_separator_height<T: MenuLayerDelegate>(
    layer: *mut types::MenuLayer,
    cell_index: *mut types::MenuIndex,
    ctx: *mut c_void,
) -> i16 {
    let delegate = unsafe { &*(ctx as *const T) };
    delegate.get_separator_height(layer, cell_index)
}

extern "C" fn trampoline_draw_row<T: MenuLayerDelegate>(
    ctx: *mut types::GContext,
    cell_layer: *const types::Layer,
    cell_index: *mut types::MenuIndex,
    callback_context: *mut c_void,
) {
    let delegate = unsafe { &*(callback_context as *const T) };
    delegate.draw_row(ctx, cell_layer, cell_index)
}

extern "C" fn trampoline_draw_header<T: MenuLayerDelegate>(
    ctx: *mut types::GContext,
    cell_layer: *const types::Layer,
    section_index: u16,
    callback_context: *mut c_void,
) {
    let delegate = unsafe { &*(callback_context as *const T) };
    delegate.draw_header(ctx, cell_layer, section_index)
}

extern "C" fn trampoline_draw_separator<T: MenuLayerDelegate>(
    ctx: *mut types::GContext,
    cell_layer: *const types::Layer,
    cell_index: *mut types::MenuIndex,
    callback_context: *mut c_void,
) {
    let delegate = unsafe { &*(callback_context as *const T) };
    delegate.draw_separator(ctx, cell_layer, cell_index)
}

extern "C" fn trampoline_draw_background<T: MenuLayerDelegate>(
    ctx: *mut types::GContext,
    bg_layer: *const types::Layer,
    highlight: bool,
    callback_context: *mut c_void,
) {
    let delegate = unsafe { &*(callback_context as *const T) };
    delegate.draw_background(ctx, bg_layer, highlight)
}

extern "C" fn trampoline_select_click<T: MenuLayerDelegate>(
    layer: *mut types::MenuLayer,
    cell_index: *mut types::MenuIndex,
    ctx: *mut c_void,
) {
    let delegate = unsafe { &*(ctx as *const T) };
    delegate.select_click(layer, cell_index)
}

extern "C" fn trampoline_select_long_click<T: MenuLayerDelegate>(
    layer: *mut types::MenuLayer,
    cell_index: *mut types::MenuIndex,
    ctx: *mut c_void,
) {
    let delegate = unsafe { &*(ctx as *const T) };
    delegate.select_long_click(layer, cell_index)
}

extern "C" fn trampoline_selection_changed<T: MenuLayerDelegate>(
    layer: *mut types::MenuLayer,
    new_index: types::MenuIndex,
    old_index: types::MenuIndex,
    ctx: *mut c_void,
) {
    let delegate = unsafe { &*(ctx as *const T) };
    delegate.selection_changed(layer, new_index, old_index)
}

extern "C" fn trampoline_selection_will_change<T: MenuLayerDelegate>(
    layer: *mut types::MenuLayer,
    new_index: *mut types::MenuIndex,
    old_index: types::MenuIndex,
    ctx: *mut c_void,
) {
    let delegate = unsafe { &*(ctx as *const T) };
    delegate.selection_will_change(layer, new_index, old_index)
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

    pub fn set_click_config_onto_window(&self, window: WindowPtr) {
        interface::menu_layer_set_click_config_onto_window(self.internal, window);
    }

    pub fn reload_data(&self) {
        interface::menu_layer_reload_data(self.internal);
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
