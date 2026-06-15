use crate::types::GlobalCell;
use pebble_sys::{TimeUnits, tm};

static USER_HANDLER: GlobalCell<Option<fn(&tm, TimeUnits)>> = GlobalCell::new(None);

extern "C" fn trampoline(tick_time: *mut tm, units_changed: TimeUnits) {
    if let Some(cb) = USER_HANDLER.get() {
        unsafe {
            if let Some(time_ref) = tick_time.as_ref() {
                cb(time_ref, units_changed);
            }
        }
    }
}

pub fn subscribe(tick_units: TimeUnits, handler: fn(&tm, TimeUnits)) {
    USER_HANDLER.set(Some(handler));
    unsafe {
        pebble_sys::tick_timer_service_subscribe(tick_units, Some(trampoline));
    }
}

pub fn unsubscribe() {
    unsafe {
        pebble_sys::tick_timer_service_unsubscribe();
    }
    USER_HANDLER.set(None);
}
