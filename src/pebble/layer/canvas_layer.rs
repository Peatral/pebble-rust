use crate::graphics::context::GContext;
use crate::layer::{ILayerMut, ILayer, LayerRef};
use crate::pebble::internal::functions::interface;
use crate::pebble::internal::types;
use crate::types::GRect;
use alloc::boxed::Box;
use core::ffi::c_void;

type DrawCallback = dyn Fn(LayerRef, GContext);

pub struct CanvasLayer {
    internal: *mut types::Layer,
    _callback: Box<Box<DrawCallback>>,
}

extern "C" fn trampoline_update_proc(layer: *mut types::Layer, ctx: *mut types::GContext) {
    unsafe {
        let data_ptr = interface::layer_get_data(layer as *const _) as *mut *const c_void;

        let closure = &*(*data_ptr as *const Box<DrawCallback>);

        closure(LayerRef::from_ptr(layer), GContext::from_ptr(ctx));
    }
}

impl CanvasLayer {
    /// Creates a new CanvasLayer with a custom drawing closure.
    pub fn new<F>(bounds: GRect, draw_logic: F) -> Self
    where
        F: Fn(LayerRef, GContext) + 'static,
    {
        let callback: Box<Box<DrawCallback>> = Box::new(Box::new(draw_logic));

        let internal = interface::layer_create_with_data(bounds, size_of::<*const c_void>());

        unsafe {
            let data_ptr = interface::layer_get_data(internal as *const _) as *mut *const c_void;
            *data_ptr = &*callback as *const Box<DrawCallback> as *const c_void;
        }

        interface::layer_set_update_proc(internal, trampoline_update_proc);

        CanvasLayer {
            internal,
            _callback: callback,
        }
    }
}

impl ILayer for CanvasLayer {
    fn as_ptr(&self) -> *const types::Layer {
        self.internal
    }
}
impl ILayerMut for CanvasLayer {
    fn as_mut_ptr(&self) -> *mut types::Layer {
        self.internal
    }
}

impl Drop for CanvasLayer {
    fn drop(&mut self) {
        interface::layer_destroy(self.internal);
    }
}
