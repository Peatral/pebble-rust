use core::ffi::c_void;
use core::marker::PhantomData;

/// A safe wrapper around the Pebble ClickRecognizerRef.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ClickRecognizer {
    internal: pebble_sys::ClickRecognizerRef,
}

impl From<pebble_sys::ClickRecognizerRef> for ClickRecognizer {
    fn from(internal: pebble_sys::ClickRecognizerRef) -> Self {
        Self { internal }
    }
}

impl ClickRecognizer {
    /// Gets the number of consecutive clicks, or the number of repetitions for auto-repeating.
    pub fn clicks_counted(&self) -> u8 {
        unsafe { pebble_sys::click_number_of_clicks_counted(self.internal) }
    }

    /// Gets the button identifier that caused the click event.
    pub fn get_button_id(&self) -> pebble_sys::ButtonId {
        unsafe { pebble_sys::click_recognizer_get_button_id(self.internal) }
    }

    /// Returns true if this is a repeating click.
    pub fn is_repeating(&self) -> bool {
        unsafe { pebble_sys::click_recognizer_is_repeating(self.internal) }
    }
}

/// A comprehensive delegate for handling all Pebble window click events safely.
pub trait ClickDelegate: Sized {
    /// Called every time the window becomes visible.
    /// Use the provided `config` to subscribe to the clicks you want.
    fn click_config(&self, config: &ClickConfigurator<Self>);

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
    pub fn subscribe_single_click(&self, button: pebble_sys::ButtonId) {
        unsafe {
            pebble_sys::window_single_click_subscribe(button, Some(trampoline_single_click::<T>));
        }
    }

    pub fn subscribe_repeating_click(&self, button: pebble_sys::ButtonId, repeat_interval_ms: u16) {
        unsafe {
            pebble_sys::window_single_repeating_click_subscribe(
                button,
                repeat_interval_ms,
                Some(trampoline_repeating_click::<T>),
            );
        }
    }

    pub fn subscribe_multi_click(
        &self,
        button: pebble_sys::ButtonId,
        min: u8,
        max: u8,
        timeout: u16,
        last_click_only: bool,
    ) {
        unsafe {
            pebble_sys::window_multi_click_subscribe(
                button,
                min,
                max,
                timeout,
                last_click_only,
                Some(trampoline_multi_click::<T>),
            );
        }
    }

    pub fn subscribe_long_click(&self, button: pebble_sys::ButtonId, delay_ms: u16) {
        unsafe {
            pebble_sys::window_long_click_subscribe(
                button,
                delay_ms,
                Some(trampoline_long_click_start::<T>),
                Some(trampoline_long_click_release::<T>),
            );
        }
    }

    pub fn subscribe_raw_click(&self, button: pebble_sys::ButtonId) {
        unsafe {
            pebble_sys::window_raw_click_subscribe(
                button,
                Some(trampoline_raw_down::<T>),
                Some(trampoline_raw_up::<T>),
                core::ptr::null_mut(),
            );
        }
    }
}

pub(crate) extern "C" fn trampoline_click_config_provider<T: ClickDelegate>(ctx: *mut c_void) {
    unsafe {
        let delegate = &*(ctx as *const T);
        let config = ClickConfigurator {
            _marker: PhantomData,
        };
        delegate.click_config(&config);
    }
}

extern "C" fn trampoline_single_click<T: ClickDelegate>(
    rec: pebble_sys::ClickRecognizerRef,
    ctx: *mut c_void,
) {
    unsafe {
        let delegate = &*(ctx as *const T);
        delegate.on_single_click(rec.into());
    }
}

extern "C" fn trampoline_repeating_click<T: ClickDelegate>(
    rec: pebble_sys::ClickRecognizerRef,
    ctx: *mut c_void,
) {
    unsafe {
        let delegate = &*(ctx as *const T);
        delegate.on_repeating_click(rec.into());
    }
}

extern "C" fn trampoline_multi_click<T: ClickDelegate>(
    rec: pebble_sys::ClickRecognizerRef,
    ctx: *mut c_void,
) {
    unsafe {
        let delegate = &*(ctx as *const T);
        delegate.on_multi_click(rec.into());
    }
}

extern "C" fn trampoline_long_click_start<T: ClickDelegate>(
    rec: pebble_sys::ClickRecognizerRef,
    ctx: *mut c_void,
) {
    unsafe {
        let delegate = &*(ctx as *const T);
        delegate.on_long_click_start(rec.into());
    }
}

extern "C" fn trampoline_long_click_release<T: ClickDelegate>(
    rec: pebble_sys::ClickRecognizerRef,
    ctx: *mut c_void,
) {
    unsafe {
        let delegate = &*(ctx as *const T);
        delegate.on_long_click_release(rec.into());
    }
}

extern "C" fn trampoline_raw_down<T: ClickDelegate>(
    rec: pebble_sys::ClickRecognizerRef,
    ctx: *mut c_void,
) {
    unsafe {
        let delegate = &*(ctx as *const T);
        delegate.on_raw_down(rec.into());
    }
}

extern "C" fn trampoline_raw_up<T: ClickDelegate>(
    rec: pebble_sys::ClickRecognizerRef,
    ctx: *mut c_void,
) {
    unsafe {
        let delegate = &*(ctx as *const T);
        delegate.on_raw_up(rec.into());
    }
}
