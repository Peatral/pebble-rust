# pebble-rs

A safe, idiomatic, high-level wrapper around the Pebble SDK for Rust. 

This crate wraps the low-level FFI bindings provided by [`pebble-sys`](../pebble-sys), bringing Rust's advantages (type safety, memory safety, ownership, safe closures, and clean abstractions) to Pebble smartwatch development while keeping the API close to the original SDK names and patterns.

## Features

- **Memory Management**: Custom allocator linking Rust's `alloc` subsystem directly to Pebble's heap allocation APIs.
- **Window & Window Stack**: Delegate-based event handling with safe, type-safe lifecycles.
- **UI Layers**: Safe wrappers for standard layer types, including [`TextLayer`](src/pebble/layer/text_layer.rs) and [`BitmapLayer`](src/pebble/layer/bitmap_layer.rs).
- **Communication (AppMessage)**: Simplified event subscriptions for exchanging data dictionary tuples with host phones.
- **Persistent Storage**: Easy-to-use API for read/write data storage.
- **Graphics**: Safe creation and rendering of bitmaps, shapes, and text.
- **Logging & Formatting**: Macros (`pbl_log!`, `pbl_warn!`, `pbl_err!`) for printf-like logging and `pbl_fmt!` for secure string formatting.
- **Code Generation Integration**: Macros to easily pull in generated assets and message keys (see [`pebble-build`](../pebble-build)).

## Quick Start Example

Add `pebble-rs` to your application `Cargo.toml`. Write your app entrypoint in `src/lib.rs`:

```rust
#![no_std]
#![no_builtins]

extern crate pebble_rs as pebble;

use pebble::window::{Window, WindowDelegate, WindowRef};
use pebble::layer::{TextLayer, ILayer, ILayerMut};
use pebble::graphics::types::{Rect, Point, Size};
use pebble::{app, window_stack};
use core::cell::RefCell;

struct AppDelegate {
    text_layer: RefCell<Option<TextLayer>>,
}

impl WindowDelegate for AppDelegate {
    fn load(&self, window: WindowRef) {
        let root = window.get_root_layer();
        let bounds = root.get_bounds();
        
        let mut text = TextLayer::new(Rect::new(Point::new(0, 50), Size::new(bounds.size.w, 40)));
        text.set_text_static(c"Hello Pebble!");
        root.add_child(&text);
        
        *self.text_layer.borrow_mut() = Some(text);
    }
    
    fn unload(&self, _window: WindowRef) {}
}

#[unsafe(no_mangle)]
pub fn main() -> isize {
    let app = app::App::new();
    let delegate = AppDelegate { text_layer: RefCell::new(None) };
    let window = Window::new(delegate);
    
    window_stack::push(*window, false);
    app.run_event_loop();
    0
}
```

## Including Assets and Keys

If your project utilizes resources or communication keys defined in `package.json`, use the helper macros:
```rust
// Generate and include resource ID constants
include_resource_ids!();

// Generate and include message key constants
include_message_keys!();
```

---

## Feature Roadmap

Below is the current wrapper coverage of the Pebble C SDK APIs in `pebble-rs`:

### Foundation APIs

| Feature / Module | Status | Details / Wrapper Modules |
| ---------------- | ------ | ------------------------- |
| **App & Event Loop** | Yes | [`pebble::app`](src/pebble/app.rs) |
| **Launch Reason** | Yes | [`pebble::launch`](src/pebble/launch.rs) |
| **AppMessage / Dictionary** | Yes | [`pebble::app_message`](src/pebble/app_message.rs) |
| **Timer** | Yes | [`pebble::timer`](src/pebble/timer.rs) |
| **Storage (Persistent)** | Yes | [`pebble::storage`](src/pebble/storage.rs) |
| **Wakeup** | Yes | [`pebble::wakeup`](src/pebble/wakeup.rs) |
| **Event Services** | Partial | Battery, Connection, TickTimer, and Touch are supported via [`pebble::event`](src/pebble/event/). Accelerometer, Compass, AppFocus, and Health services are not yet wrapped. |
| **AppSync / AppWorker** | No | Not yet wrapped |
| **DataLogging / Dictation** | No | Not yet wrapped |
| **App Glance / Exit Reason** | No | Not yet wrapped |

### Graphics APIs

| Feature / Module | Status | Details / Wrapper Modules |
| ---------------- | ------ | ------------------------- |
| **Graphics Context** | Yes | [`pebble::graphics::context`](src/pebble/graphics/context.rs) |
| **Drawing Primitives** | Yes | Circles, rectangles, lines, etc. in [`pebble::graphics::primitives`](src/pebble/graphics/primitives.rs) |
| **Drawing Text** | Yes | [`pebble::graphics::text`](src/pebble/graphics/text.rs) |
| **Drawing Paths (GPath)** | Partial | Basic paths in [`pebble::graphics::primitives`](src/pebble/graphics/primitives.rs) |
| **Fonts** | Yes | [`pebble::system::fonts`](src/pebble/system/fonts.rs) |
| **Draw Commands (gdraw)** | No | Not yet wrapped |

### User Interface APIs

| Feature / Module | Status | Details / Wrapper Modules |
| ---------------- | ------ | ------------------------- |
| **Window & Window Stack** | Yes | [`pebble::window`](src/pebble/window.rs) & [`pebble::window_stack`](src/pebble/window_stack.rs) |
| **Clicks (Click Recognition)**| Yes | [`pebble::clicks`](src/pebble/clicks.rs) (ClickRecognizer, ClickDelegate) |
| **Layers** | Partial | TextLayer, BitmapLayer, CanvasLayer, MenuLayer, ScrollLayer, ActionBarLayer, and StatusBarLayer are supported via [`pebble::layer`](src/pebble/layer/). RotBitmapLayer and SimpleMenuLayer are not yet wrapped. |
| **Vibes (Vibration)** | Yes | [`pebble::vibes`](src/pebble/vibes.rs) |
| **Animation / PropAnim** | No | Not yet wrapped |
| **Light & Speaker** | No | Not yet wrapped |
| **ActionMenu / NumberWindow** | No | Not yet wrapped |
| **UnobstructedArea** | No | Not yet wrapped |

### Standard C Wrappers

| Feature / Module | Status | Details / Wrapper Modules |
| ---------------- | ------ | ------------------------- |
| **Math, Locale, String, Time**| Yes | Standard C library wraps in [`pebble::std`](src/pebble/std/) (locale, math, string, time) |
| **Formatting** | Yes | Stack-allocated secure formatting macro `pbl_fmt!` using raw `snprintf` |

For the complete list of C APIs, refer to the [Pebble C SDK Documentation](https://developer.repebble.com/docs/c/).

---

## Troubleshooting

### Failed to parse ELF sections while calculating the virtual size
If you receive this error, it is because your compiled ELF binary has more than 99 sections. This typically happens when using formatting modules like `core::fmt`, `alloc::string::String`, or certain UTF-8 validation methods such as `core::str::from_utf8` (consider using `from_utf8_unchecked` where appropriate).

The standard Pebble SDK metadata injection script breaks if the ELF section indices occupy more than 2 digits in the `readelf` command columns.

To fix this:
1. Open the file `~/.local/share/pebble-sdk/SDKs/current/sdk-core/pebble/common/tools/inject_metadata.py` (or `~/.pebble-sdk/SDKs/current/sdk-core/pebble/common/tools/inject_metadata.py` for legacy installations)
2. Locate line `136` (or similar) which contains:
   ```python
   line = line[6:]
   ```
3. Change it to:
   ```python
   if not ']' in line:
       continue
   line = line[line.index(']')+1:]
   ```

---

## License

This project is licensed under **both** the [GPLv3](../LICENSE-GPLv3) and [BSD-3-Clause](../LICENSE-BSD-3.0) licenses.

---

## Credits & Acknowledgements

* **[RoccoDev](https://github.com/roccodev)**: The original maintainer and creator of the `pebble-rust` project, who wrote the initial wrappers and API definitions.
* **[Eva van Houten](https://github.com/evavh)**: For extensive cleanup and improvements to the build setup to ensure compatibility with newer Rust toolchains and emulator deployments.
* **[andars](https://github.com/andars)**: This project uses some files from their [pebble.rs](https://github.com/andars/pebble.rs) project.