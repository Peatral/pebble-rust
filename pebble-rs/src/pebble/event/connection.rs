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
use crate::types::GlobalCell;
use pebble_sys::ConnectionHandlers;

static APP_HANDLER: GlobalCell<Option<fn(bool)>> = GlobalCell::new(None);
static KIT_HANDLER: GlobalCell<Option<fn(bool)>> = GlobalCell::new(None);

extern "C" fn app_trampoline(connected: bool) {
    if let Some(cb) = APP_HANDLER.get() {
        cb(connected);
    }
}

extern "C" fn kit_trampoline(connected: bool) {
    if let Some(cb) = KIT_HANDLER.get() {
        cb(connected);
    }
}

pub fn subscribe(app_handler: Option<fn(bool)>, kit_handler: Option<fn(bool)>) {
    APP_HANDLER.set(app_handler);
    KIT_HANDLER.set(kit_handler);

    let handlers = ConnectionHandlers {
        pebble_app_connection_handler: if app_handler.is_some() {
            Some(app_trampoline)
        } else {
            None
        },
        pebblekit_connection_handler: if kit_handler.is_some() {
            Some(kit_trampoline)
        } else {
            None
        },
    };

    unsafe {
        pebble_sys::connection_service_subscribe(handlers);
    }
}

pub fn unsubscribe() {
    unsafe {
        pebble_sys::connection_service_unsubscribe();
    }
    APP_HANDLER.set(None);
    KIT_HANDLER.set(None);
}

pub fn peek_app() -> bool {
    unsafe { pebble_sys::connection_service_peek_pebble_app_connection() }
}

pub fn peek_pebblekit() -> bool {
    unsafe { pebble_sys::connection_service_peek_pebblekit_connection() }
}
