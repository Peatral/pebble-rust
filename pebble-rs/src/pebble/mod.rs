/*
 * This file is part of pebble-rs.
 * Copyright (c) 2019 RoccoDev
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

mod internal;

pub mod app;
pub mod app_message;
pub mod clicks;
pub mod clock;
pub mod event;
pub mod graphics;
pub mod launch;
pub mod layer;
pub mod std;
pub mod storage;
pub mod system;
pub mod timer;
pub mod types;
pub mod vibes;
pub mod wakeup;
pub mod window;
pub mod window_stack;

pub use internal::alloc;

pub type Result<T> = core::result::Result<T, &'static str>;

pub use pebble_sys::app_log as println;
pub use pebble_sys::snprintf;

#[cfg(not(test))]
#[inline(never)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
        let file = location.file();
        let line = location.line();

        crate::pbl_err!(c"FATAL PANIC at {}:{}! Forcing App Fault...", file, line);
    } else {
        crate::pbl_err!(c"FATAL PANIC! (Unknown location). Forcing App Fault...");
    }

    unsafe {
        let crash: *mut u32 = core::ptr::null_mut();
        core::ptr::write_volatile(crash, 0xDEADBEEF);
    }

    loop {}
}

#[allow(clippy::empty_loop)]
#[unsafe(no_mangle)]
pub extern "C" fn _exit(_status: i32) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _kill(_pid: i32, _sig: i32) -> i32 {
    -1
}

#[unsafe(no_mangle)]
pub extern "C" fn _getpid() -> i32 {
    1
}

#[allow(non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static __exidx_start: u32 = 0;

#[allow(non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static __exidx_end: u32 = 0;

#[macro_export]
macro_rules! pbl_print {
    ($lvl: expr, $name: expr, $fmt: expr $(, $arg:expr)*) => {
        unsafe {
            pebble_sys::app_log($lvl, $name.as_ptr(), 0, $fmt.as_ptr() $(, $arg)*);
        }
    };
}

#[macro_export]
macro_rules! pbl_log {
    ($fmt: expr $(, $arg: expr)*) => {
        $crate::pbl_print!(100, c"pebble-rs (Info)", $fmt $(, $arg)*);
    };
}

#[macro_export]
macro_rules! pbl_warn {
    ($fmt: expr $(, $arg: expr)*) => {
        $crate::pbl_print!(50, c"pebble-rs (Warning)", $fmt $(, $arg)*);
    };
}

#[macro_export]
macro_rules! pbl_err {
    ($fmt: expr $(, $arg: expr)*) => {
        $crate::pbl_print!(1, c"pebble-rs (Error)", $fmt $(, $arg)*);
    };
}

#[macro_export]
macro_rules! null_term {
    ($content: tt) => {
        concat!($content, "\0");
    };
}

#[macro_export]
macro_rules! nt {
    ($content: tt) => {
        null_term!($content);
    };
}

#[macro_export]
macro_rules! include_generated {
    ($mod_name:ident, $file_name:expr) => {
        pub mod $mod_name {
            include!(concat!(env!("OUT_DIR"), "/", $file_name));
        }
    };
}

#[macro_export]
macro_rules! include_message_keys {
    () => {
        $crate::include_generated!(message_keys, "message_keys.rs");
    };
}
