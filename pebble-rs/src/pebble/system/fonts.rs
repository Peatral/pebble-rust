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

use crate::pebble::internal::functions::declarations::*;
use crate::pebble::internal::functions::interface;
use crate::pebble::internal::types::{GFont, ResHandle};
use core::ffi::CStr;

pub const FONT_KEY_GOTHIC_18_BOLD: &CStr = c"RESOURCE_ID_GOTHIC_18_BOLD";
pub const FONT_KEY_GOTHIC_24: &CStr = c"RESOURCE_ID_GOTHIC_24";
pub const FONT_KEY_GOTHIC_09: &CStr = c"RESOURCE_ID_GOTHIC_09";
pub const FONT_KEY_GOTHIC_14: &CStr = c"RESOURCE_ID_GOTHIC_14";
pub const FONT_KEY_GOTHIC_14_BOLD: &CStr = c"RESOURCE_ID_GOTHIC_14_BOLD";
pub const FONT_KEY_GOTHIC_18: &CStr = c"RESOURCE_ID_GOTHIC_18";
pub const FONT_KEY_GOTHIC_24_BOLD: &CStr = c"RESOURCE_ID_GOTHIC_24_BOLD";
pub const FONT_KEY_GOTHIC_28: &CStr = c"RESOURCE_ID_GOTHIC_28";
pub const FONT_KEY_GOTHIC_28_BOLD: &CStr = c"RESOURCE_ID_GOTHIC_28_BOLD";
pub const FONT_KEY_BITHAM_30_BLACK: &CStr = c"RESOURCE_ID_BITHAM_30_BLACK";
pub const FONT_KEY_BITHAM_42_BOLD: &CStr = c"RESOURCE_ID_BITHAM_42_BOLD";
pub const FONT_KEY_BITHAM_42_LIGHT: &CStr = c"RESOURCE_ID_BITHAM_42_LIGHT";
pub const FONT_KEY_BITHAM_42_MEDIUM_NUMBERS: &CStr = c"RESOURCE_ID_BITHAM_42_MEDIUM_NUMBERS";
pub const FONT_KEY_BITHAM_34_MEDIUM_NUMBERS: &CStr = c"RESOURCE_ID_BITHAM_34_MEDIUM_NUMBERS";
pub const FONT_KEY_BITHAM_34_LIGHT_SUBSET: &CStr = c"RESOURCE_ID_BITHAM_34_LIGHT_SUBSET";
pub const FONT_KEY_BITHAM_18_LIGHT_SUBSET: &CStr = c"RESOURCE_ID_BITHAM_18_LIGHT_SUBSET";
pub const FONT_KEY_DROID_SERIF_28_BOLD: &CStr = c"RESOURCE_ID_DROID_SERIF_28_BOLD";
pub const FONT_KEY_LECO_20_BOLD_NUMBERS: &CStr = c"RESOURCE_ID_LECO_20_BOLD_NUMBERS";
pub const FONT_KEY_LECO_26_BOLD_NUMBERS_AM_PM: &CStr = c"RESOURCE_ID_LECO_26_BOLD_NUMBERS_AM_PM";
pub const FONT_KEY_LECO_32_BOLD_NUMBERS: &CStr = c"RESOURCE_ID_LECO_32_BOLD_NUMBERS";
pub const FONT_KEY_LECO_36_BOLD_NUMBERS: &CStr = c"RESOURCE_ID_LECO_36_BOLD_NUMBERS";
pub const FONT_KEY_LECO_38_BOLD_NUMBERS: &CStr = c"RESOURCE_ID_LECO_38_BOLD_NUMBERS";
pub const FONT_KEY_LECO_42_BOLD_NUMBERS: &CStr = c"RESOURCE_ID_LECO_42_BOLD_NUMBERS";
pub const FONT_KEY_LECO_28_LIGHT_NUMBERS: &CStr = c"RESOURCE_ID_LECO_28_LIGHT_NUMBERS";

pub struct Font {
    pub internal: GFont,
    /// Tracks whether this font needs to be manually freed
    is_custom: bool,
}

impl Font {
    pub fn get_system(font_key: &CStr) -> Self {
        let internal = interface::fonts_get_system_font(font_key.as_ptr());
        Self {
            internal,
            is_custom: false,
        }
    }

    pub fn get_custom_from_handle(res_handle: ResHandle) -> Self {
        let internal = interface::fonts_load_custom_font(res_handle);
        Self {
            internal,
            is_custom: true,
        }
    }

    pub fn get_custom(resource_id: u32) -> Self {
        let res_handle = interface::resource_get_handle(resource_id);
        Self::get_custom_from_handle(res_handle)
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        if self.is_custom && !self.internal.is_null() {
            interface::fonts_unload_custom_font(self.internal);
        }
    }
}
