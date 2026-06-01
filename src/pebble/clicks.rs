use crate::pebble::internal::functions::interface;
use crate::pebble::internal::types;
use core::ffi::c_void;
use core::marker::PhantomData;

/// A safe wrapper around the Pebble ClickRecognizerRef.
pub struct ClickRecognizer {
    internal: types::ClickRecognizerRef,
}

impl ClickRecognizer {
    pub(crate) fn from_ptr(internal: types::ClickRecognizerRef) -> Self {
        Self { internal }
    }

    /// Gets the number of consecutive clicks, or the number of repetitions for auto-repeating.
    pub fn clicks_counted(&self) -> u8 {
        interface::click_number_of_clicks_counted(self.internal)
    }

    /// Gets the button identifier that caused the click event.
    pub fn get_button_id(&self) -> types::ButtonId {
        interface::click_recognizer_get_button_id(self.internal)
    }

    /// Returns true if this is a repeating click.
    pub fn is_repeating(&self) -> bool {
        interface::click_recognizer_is_repeating(self.internal)
    }
}

/// A comprehensive delegate for handling all Pebble window click events safely.
pub trait ClickDelegate: Sized {
    /// Called every time the window becomes visible.
    /// Use the provided `config` to subscribe to the clicks you want.
    fn click_config(&self, config: &ClickConfigurator<Self>);

    // Event Handlers (Defaults do nothing)
    fn on_single_click(&self, _recognizer: ClickRecognizer) {}
    fn on_repeating_click(&self, _recognizer: ClickRecognizer) {}
    fn on_multi_click(&self, _recognizer: ClickRecognizer) {}
    fn on_long_click_start(&self, _recognizer: ClickRecognizer) {}
    fn on_long_click_release(&self, _recognizer: ClickRecognizer) {}
    fn on_raw_down(&self, _recognizer: ClickRecognizer) {}
    fn on_raw_up(&self, _recognizer: ClickRecognizer) {}
}

/// Builder to safely configure button subscriptions.
pub struct ClickConfigurator<'a, T: ClickDelegate> {
    _marker: PhantomData<&'a T>,
}

impl<'a, T: ClickDelegate> ClickConfigurator<'a, T> {
    pub fn subscribe_single_click(&self, button: types::ButtonId) {
        interface::window_single_click_subscribe(button, Some(trampoline_single_click::<T>));
    }

    pub fn subscribe_repeating_click(&self, button: types::ButtonId, repeat_interval_ms: u16) {
        interface::window_single_repeating_click_subscribe(
            button,
            repeat_interval_ms,
            Some(trampoline_repeating_click::<T>),
        );
    }

    pub fn subscribe_multi_click(
        &self,
        button: types::ButtonId,
        min: u8,
        max: u8,
        timeout: u16,
        last_click_only: bool,
    ) {
        interface::window_multi_click_subscribe(
            button,
            min,
            max,
            timeout,
            last_click_only,
            Some(trampoline_multi_click::<T>),
        );
    }

    pub fn subscribe_long_click(&self, button: types::ButtonId, delay_ms: u16) {
        interface::window_long_click_subscribe(
            button,
            delay_ms,
            Some(trampoline_long_click_start::<T>),
            Some(trampoline_long_click_release::<T>),
        );
    }

    pub fn subscribe_raw_click(&self, button: types::ButtonId) {
        interface::window_raw_click_subscribe(
            button,
            Some(trampoline_raw_down::<T>),
            Some(trampoline_raw_up::<T>),
            core::ptr::null_mut(), // Context is already handled globally by the provider
        );
    }
}

pub(crate) extern "C" fn trampoline_click_config_provider<T: ClickDelegate>(ctx: *mut c_void) {
    let delegate = unsafe { &*(ctx as *const T) };
    let config = ClickConfigurator {
        _marker: PhantomData,
    };
    delegate.click_config(&config);
}

extern "C" fn trampoline_single_click<T: ClickDelegate>(
    rec: types::ClickRecognizerRef,
    ctx: *mut c_void,
) {
    let delegate = unsafe { &*(ctx as *const T) };
    delegate.on_single_click(ClickRecognizer::from_ptr(rec));
}

extern "C" fn trampoline_repeating_click<T: ClickDelegate>(
    rec: types::ClickRecognizerRef,
    ctx: *mut c_void,
) {
    let delegate = unsafe { &*(ctx as *const T) };
    delegate.on_repeating_click(ClickRecognizer::from_ptr(rec));
}

extern "C" fn trampoline_multi_click<T: ClickDelegate>(
    rec: types::ClickRecognizerRef,
    ctx: *mut c_void,
) {
    let delegate = unsafe { &*(ctx as *const T) };
    delegate.on_multi_click(ClickRecognizer::from_ptr(rec));
}

extern "C" fn trampoline_long_click_start<T: ClickDelegate>(
    rec: types::ClickRecognizerRef,
    ctx: *mut c_void,
) {
    let delegate = unsafe { &*(ctx as *const T) };
    delegate.on_long_click_start(ClickRecognizer::from_ptr(rec));
}

extern "C" fn trampoline_long_click_release<T: ClickDelegate>(
    rec: types::ClickRecognizerRef,
    ctx: *mut c_void,
) {
    let delegate = unsafe { &*(ctx as *const T) };
    delegate.on_long_click_release(ClickRecognizer::from_ptr(rec));
}

extern "C" fn trampoline_raw_down<T: ClickDelegate>(
    rec: types::ClickRecognizerRef,
    ctx: *mut c_void,
) {
    let delegate = unsafe { &*(ctx as *const T) };
    delegate.on_raw_down(ClickRecognizer::from_ptr(rec));
}

extern "C" fn trampoline_raw_up<T: ClickDelegate>(
    rec: types::ClickRecognizerRef,
    ctx: *mut c_void,
) {
    let delegate = unsafe { &*(ctx as *const T) };
    delegate.on_raw_up(ClickRecognizer::from_ptr(rec));
}
