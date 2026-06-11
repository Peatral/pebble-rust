use crate::pebble::internal::functions::interface;

pub fn cancel() {
    interface::vibes_cancel()
}
pub fn short_pulse() {
    interface::vibes_short_pulse()
}

pub fn long_pulse() {
    interface::vibes_long_pulse()
}

pub fn double_pulse() {
    interface::vibes_double_pulse()
}

pub fn enqueue_custom_pattern(durations: &'static [u32]) {
    interface::vibes_enqueue_custom_pattern(durations)
}
