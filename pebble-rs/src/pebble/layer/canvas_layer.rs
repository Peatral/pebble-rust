/*
 * This file is part of pebble-rs.
 * Copyright (c) 2026 Peatral
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */
use crate::graphics::context::Context;
use crate::graphics::types::Rect;
use crate::layer::{ILayer, ILayerMut, Layer, LayerMut};
use alloc::boxed::Box;
use core::ffi::c_void;
use core::ops::{Deref, DerefMut};

type DrawCallback = dyn Fn(LayerMut, Context);

pub struct CanvasLayer {
    layer_ref: Layer,
    _callback: Box<Box<DrawCallback>>,
}

extern "C" fn trampoline_update_proc(
    layer: *mut pebble_sys::Layer,
    ctx: *mut pebble_sys::GContext,
) {
    unsafe {
        let data_ptr = pebble_sys::layer_get_data(layer as *const _) as *mut *const c_void;
        let closure = &*(*data_ptr as *const Box<DrawCallback>);

        closure(layer.into(), ctx.into());
    }
}

impl CanvasLayer {
    /// Creates a new CanvasLayer with a custom drawing closure.
    pub fn new<F>(bounds: Rect, draw_logic: F) -> Self
    where
        F: Fn(LayerMut, Context) + 'static,
    {
        let callback: Box<Box<DrawCallback>> = Box::new(Box::new(draw_logic));

        unsafe {
            let internal = pebble_sys::layer_create_with_data(bounds.0, size_of::<*const c_void>());

            let data_ptr = pebble_sys::layer_get_data(internal as *const _) as *mut *const c_void;
            *data_ptr = &*callback as *const Box<DrawCallback> as *const c_void;

            pebble_sys::layer_set_update_proc(internal, Some(trampoline_update_proc));

            CanvasLayer {
                layer_ref: Layer::from_raw_owned(internal),
                _callback: callback,
            }
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
    fn as_ptr(&self) -> *const pebble_sys::Layer {
        self.layer_ref.as_ptr()
    }
}

impl ILayerMut for CanvasLayer {
    fn as_mut_ptr(&self) -> *mut pebble_sys::Layer {
        self.layer_ref.as_mut_ptr()
    }
}
