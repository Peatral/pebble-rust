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
use crate::pebble::event::Event;
use pebble_sys::BatteryChargeState;

pub struct BatteryStateEvent;

impl Event<BatteryChargeState> for BatteryStateEvent {
    fn subscribe(handler: extern "C" fn(state: BatteryChargeState)) {
        unsafe {
            pebble_sys::battery_state_service_subscribe(Some(handler));
        }
    }

    fn unsubscribe() {
        unsafe {
            pebble_sys::battery_state_service_unsubscribe();
        }
    }

    fn peek() -> Result<BatteryChargeState, i32> {
        unsafe { Ok(pebble_sys::battery_state_service_peek()) }
    }
}
