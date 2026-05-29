/*
 * This file is part of pebble-rust.
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

use crate::pebble::internal::functions::declarations::{
    battery_state_service_peek, battery_state_service_subscribe, battery_state_service_unsubscribe,
};
pub use crate::pebble::internal::types::BatteryChargeState;

pub struct BatteryStateEvent;

impl Event<BatteryChargeState> for BatteryStateEvent {
    fn subscribe(handler: extern "C" fn(state: BatteryChargeState)) {
        unsafe {
            battery_state_service_subscribe(handler);
        }
    }

    fn unsubscribe() {
        unsafe {
            battery_state_service_unsubscribe();
        }
    }

    fn peek() -> Result<BatteryChargeState, i32> {
        unsafe { Ok(battery_state_service_peek()) }
    }
}
