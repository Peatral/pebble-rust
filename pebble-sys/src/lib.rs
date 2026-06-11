#![no_std]
#![no_builtins]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unnecessary_transmutes)]

cfg_if::cfg_if! {
    if #[cfg(feature = "emery")] {
        include!(concat!(env!("OUT_DIR"), "/bindings_emery.rs"));
    } else if #[cfg(feature = "diorite")] {
        include!(concat!(env!("OUT_DIR"), "/bindings_diorite.rs"));
    } else if #[cfg(feature = "chalk")] {
        include!(concat!(env!("OUT_DIR"), "/bindings_chalk.rs"));
    } else if #[cfg(feature = "basalt")] {
        include!(concat!(env!("OUT_DIR"), "/bindings_basalt.rs"));
    } else if #[cfg(feature = "flint")] {
        include!(concat!(env!("OUT_DIR"), "/bindings_flint.rs"));
    } else if #[cfg(feature = "gabbro")] {
        include!(concat!(env!("OUT_DIR"), "/bindings_gabbro.rs"));
    } else if #[cfg(feature = "aplite")] {
        include!(concat!(env!("OUT_DIR"), "/bindings_aplite.rs"));
    } else {
        compile_error!("You must enable at least one Pebble platform feature (e.g., 'emery', 'basalt').");
    }
}