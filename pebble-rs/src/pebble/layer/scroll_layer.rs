use alloc::boxed::Box;
use core::ffi::c_void;
use core::ops::{Deref, DerefMut};

use crate::graphics::types::{Point, Rect, Size};
use crate::layer::{ILayer, ILayerMut};
use crate::pebble::clicks::{ClickDelegate, ClickRecognizer, trampoline_click_config_provider};
use crate::pebble::window::WindowRef;
use pebble_sys::{ContentIndicatorConfig, ContentIndicatorDirection, Layer};

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ContentIndicatorRef {
    internal: *mut pebble_sys::ContentIndicator,
}

impl ContentIndicatorRef {
    pub fn configure_direction(
        &self,
        direction: ContentIndicatorDirection,
        config: Option<&ContentIndicatorConfig>,
    ) -> bool {
        unsafe {
            let config_ptr = config.map_or(core::ptr::null(), |c| c as *const _);
            pebble_sys::content_indicator_configure_direction(self.internal, direction, config_ptr)
        }
    }

    pub fn get_content_available(&self, direction: ContentIndicatorDirection) -> bool {
        unsafe { pebble_sys::content_indicator_get_content_available(self.internal, direction) }
    }

    pub fn set_content_available(&self, direction: ContentIndicatorDirection, available: bool) {
        unsafe {
            pebble_sys::content_indicator_set_content_available(
                self.internal,
                direction,
                available,
            );
        }
    }
}

#[repr(transparent)]
pub struct ContentIndicator {
    internal: *mut pebble_sys::ContentIndicator,
}

impl ContentIndicator {
    pub fn new() -> Option<Self> {
        let ptr = unsafe { pebble_sys::content_indicator_create() };
        if ptr.is_null() {
            None
        } else {
            Some(Self { internal: ptr })
        }
    }

    pub fn as_ref(&self) -> ContentIndicatorRef {
        ContentIndicatorRef {
            internal: self.internal,
        }
    }
}

impl Drop for ContentIndicator {
    fn drop(&mut self) {
        unsafe {
            pebble_sys::content_indicator_destroy(self.internal);
        }
    }
}

/// A delegate specifically for ScrollLayers.
pub trait ScrollDelegate: Sized {
    fn content_offset_changed(&self, _scroll_layer: ScrollLayerRef) {}
}

extern "C" fn trampoline_content_offset_changed<T: ScrollDelegate>(
    scroll_layer: *mut pebble_sys::ScrollLayer,
    ctx: *mut c_void,
) {
    unsafe {
        let delegate = &*(ctx as *const T);
        delegate.content_offset_changed(ScrollLayerRef {
            internal: scroll_layer,
        });
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ScrollLayerRef {
    internal: *mut pebble_sys::ScrollLayer,
}

impl ScrollLayerRef {
    /// Adds a child layer to the scrollable content sub-layer.
    pub fn add_scroll_child(&self, child: &dyn ILayerMut) {
        unsafe {
            pebble_sys::scroll_layer_add_child(self.internal, child.as_mut_ptr());
        }
    }

    /// Associates the UP and DOWN buttons on the window with scrolling up and down.
    pub fn set_click_config_onto_window(&self, window: &WindowRef) {
        unsafe {
            pebble_sys::scroll_layer_set_click_config_onto_window(self.internal, window.as_ptr());
        }
    }

    pub fn set_content_offset(&self, offset: Point, animated: bool) {
        unsafe {
            pebble_sys::scroll_layer_set_content_offset(self.internal, offset.0, animated);
        }
    }

    pub fn get_content_offset(&self) -> Point {
        unsafe { Point(pebble_sys::scroll_layer_get_content_offset(self.internal)) }
    }

    pub fn set_content_size(&self, size: Size) {
        unsafe {
            pebble_sys::scroll_layer_set_content_size(self.internal, size.0);
        }
    }

    pub fn get_content_size(&self) -> Size {
        unsafe { Size(pebble_sys::scroll_layer_get_content_size(self.internal)) }
    }

    pub fn set_frame(&self, frame: Rect) {
        unsafe {
            pebble_sys::scroll_layer_set_frame(self.internal, frame.0);
        }
    }

    pub fn set_shadow_hidden(&self, hidden: bool) {
        unsafe {
            pebble_sys::scroll_layer_set_shadow_hidden(self.internal, hidden);
        }
    }

    pub fn get_shadow_hidden(&self) -> bool {
        unsafe { pebble_sys::scroll_layer_get_shadow_hidden(self.internal) }
    }

    pub fn set_paging(&self, paging_enabled: bool) {
        unsafe {
            pebble_sys::scroll_layer_set_paging(self.internal, paging_enabled);
        }
    }

    pub fn get_paging(&self) -> bool {
        unsafe { pebble_sys::scroll_layer_get_paging(self.internal) }
    }

    pub fn get_content_indicator(&self) -> Option<ContentIndicatorRef> {
        unsafe {
            let ptr = pebble_sys::scroll_layer_get_content_indicator(self.internal);
            if ptr.is_null() {
                None
            } else {
                Some(ContentIndicatorRef { internal: ptr })
            }
        }
    }

    /// Exposes the default scroll up behavior if you are manually handling button events.
    pub fn scroll_up_click_handler(&self, recognizer: ClickRecognizer) {
        unsafe {
            pebble_sys::scroll_layer_scroll_up_click_handler(
                recognizer.as_ptr(),
                self.internal as *mut c_void,
            );
        }
    }

    /// Exposes the default scroll down behavior if you are manually handling button events.
    pub fn scroll_down_click_handler(&self, recognizer: ClickRecognizer) {
        unsafe {
            pebble_sys::scroll_layer_scroll_down_click_handler(
                recognizer.as_ptr(),
                self.internal as *mut c_void,
            );
        }
    }
}

impl ILayer for ScrollLayerRef {
    fn as_ptr(&self) -> *const Layer {
        unsafe { pebble_sys::scroll_layer_get_layer(self.internal) }
    }
}

impl ILayerMut for ScrollLayerRef {
    fn as_mut_ptr(&self) -> *mut Layer {
        unsafe { pebble_sys::scroll_layer_get_layer(self.internal) }
    }
}

impl From<*mut pebble_sys::ScrollLayer> for ScrollLayerRef {
    fn from(internal: *mut pebble_sys::ScrollLayer) -> Self {
        Self { internal }
    }
}

/// A ScrollLayer that natively manages its own memory and delegate lifecycle.
pub struct ScrollLayer<T: ScrollDelegate> {
    layer_ref: ScrollLayerRef,
    delegate: Box<T>,
}

impl<T: ScrollDelegate> ScrollLayer<T> {
    pub fn new(frame: Rect, delegate: T) -> Self {
        unsafe {
            let internal = pebble_sys::scroll_layer_create(frame.0);

            let layer = Self {
                layer_ref: internal.into(),
                delegate: Box::new(delegate),
            };

            let context_ptr = &*layer.delegate as *const T as *mut c_void;
            pebble_sys::scroll_layer_set_context(internal, context_ptr);

            let callbacks = pebble_sys::ScrollLayerCallbacks {
                click_config_provider: None,
                content_offset_changed_handler: Some(trampoline_content_offset_changed::<T>),
            };

            pebble_sys::scroll_layer_set_callbacks(internal, callbacks);

            layer
        }
    }
}

impl<T: ScrollDelegate + ClickDelegate> ScrollLayer<T> {
    /// Overrides the ScrollLayer's internal callbacks to include your ClickDelegate.
    /// This allows you to safely capture the SELECT button or override UP/DOWN.
    pub fn enable_clicks_override(&self) {
        unsafe {
            let callbacks = pebble_sys::ScrollLayerCallbacks {
                click_config_provider: Some(trampoline_click_config_provider::<T>),
                content_offset_changed_handler: Some(trampoline_content_offset_changed::<T>),
            };

            pebble_sys::scroll_layer_set_callbacks(self.layer_ref.internal, callbacks);
        }
    }

    /// Removes your custom ClickDelegate from the ScrollLayer,
    /// reverting it entirely to the default Up/Down scrolling behavior.
    pub fn disable_clicks_override(&self) {
        unsafe {
            let callbacks = pebble_sys::ScrollLayerCallbacks {
                click_config_provider: None,
                content_offset_changed_handler: Some(trampoline_content_offset_changed::<T>),
            };

            pebble_sys::scroll_layer_set_callbacks(self.layer_ref.internal, callbacks);
        }
    }
}

impl<T: ScrollDelegate> ILayer for ScrollLayer<T> {
    fn as_ptr(&self) -> *const Layer {
        self.layer_ref.as_ptr()
    }
}

impl<T: ScrollDelegate> ILayerMut for ScrollLayer<T> {
    fn as_mut_ptr(&self) -> *mut Layer {
        self.layer_ref.as_mut_ptr()
    }
}

impl<T: ScrollDelegate> Deref for ScrollLayer<T> {
    type Target = ScrollLayerRef;

    fn deref(&self) -> &Self::Target {
        &self.layer_ref
    }
}

impl<T: ScrollDelegate> DerefMut for ScrollLayer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.layer_ref
    }
}

impl<T: ScrollDelegate> Drop for ScrollLayer<T> {
    fn drop(&mut self) {
        unsafe {
            pebble_sys::scroll_layer_destroy(self.internal);
        }
    }
}
