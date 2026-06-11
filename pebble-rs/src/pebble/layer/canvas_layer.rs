use crate::graphics::context::GContext;
use crate::layer::{ILayer, ILayerMut, Layer, LayerMut, LayerRef};
use crate::pebble::internal::functions::interface;
use crate::pebble::internal::types;
use crate::types::GRect;
use alloc::boxed::Box;
use core::ffi::c_void;
use core::ops::{Deref, DerefMut};

type DrawCallback = dyn Fn(LayerMut, GContext);

pub struct CanvasLayer {
    layer_ref: Layer,
    _callback: Box<Box<DrawCallback>>,
}

extern "C" fn trampoline_update_proc(layer: *mut types::Layer, ctx: *mut types::GContext) {
    unsafe {
        let data_ptr = interface::layer_get_data(layer as *const _) as *mut *const c_void;
        let closure = &*(*data_ptr as *const Box<DrawCallback>);

        closure(layer.into(), ctx.into());
    }
}

impl CanvasLayer {
    /// Creates a new CanvasLayer with a custom drawing closure.
    pub fn new<F>(bounds: GRect, draw_logic: F) -> Self
    where
        F: Fn(LayerMut, GContext) + 'static,
    {
        let callback: Box<Box<DrawCallback>> = Box::new(Box::new(draw_logic));

        let internal = interface::layer_create_with_data(bounds, size_of::<*const c_void>());

        unsafe {
            let data_ptr = interface::layer_get_data(internal as *const _) as *mut *const c_void;
            *data_ptr = &*callback as *const Box<DrawCallback> as *const c_void;
        }

        interface::layer_set_update_proc(internal, trampoline_update_proc);

        CanvasLayer {
            layer_ref: Layer::from_raw_owned(internal),
            _callback: callback,
        }
    }
}

impl Deref for CanvasLayer {
    type Target = Layer;

    fn deref(&self) -> &Self::Target {
        &self.layer_ref
    }
}

impl DerefMut for CanvasLayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.layer_ref
    }
}

impl ILayer for CanvasLayer {
    fn as_ptr(&self) -> *const types::Layer {
        self.layer_ref.as_ptr()
    }
}

impl ILayerMut for CanvasLayer {
    fn as_mut_ptr(&self) -> *mut types::Layer {
        self.layer_ref.as_mut_ptr()
    }
}
