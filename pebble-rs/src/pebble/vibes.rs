use pebble_sys::VibePattern;

pub fn cancel() {
    unsafe { pebble_sys::vibes_cancel() }
}
pub fn short_pulse() {
    unsafe { pebble_sys::vibes_short_pulse() }
}

pub fn long_pulse() {
    unsafe { pebble_sys::vibes_long_pulse() }
}

pub fn double_pulse() {
    unsafe { pebble_sys::vibes_double_pulse() }
}

pub fn enqueue_custom_pattern(durations: &'static [u32]) {
    let pattern = VibePattern {
        durations: durations.as_ptr(),
        num_segments: durations.len() as u32,
    };
    unsafe { pebble_sys::vibes_enqueue_custom_pattern(pattern) }
}
