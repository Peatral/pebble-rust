/*
 * Copyright (c) 2019, Andrew Foote. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
    * Redistributions of source code must retain the above copyright
      notice, this list of conditions and the following disclaimer.
    * Redistributions in binary form must reproduce the above copyright
      notice, this list of conditions and the following disclaimer in the
      documentation and/or other materials provided with the distribution.
    * Neither the name of the copyright holder nor the
      names of its contributors may be used to endorse or promote products
      derived from this software without specific prior written permission.

 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
 * BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE
 * OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF
 * ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

#![allow(non_camel_case_types)]
#![allow(clippy::from_over_into)]

use core::ffi::{CStr, c_uint, c_void};
use core::ptr;

pub enum Window {}
pub enum Layer {}
pub enum TextLayer {}
pub enum ClickRecognizer {}
pub enum GBitmap {}
pub enum GContext {}
pub enum BitmapLayer {}
pub enum MenuLayer {}
pub enum ActionBarLayer {}
pub enum StatusBarLayer {}
pub enum AppTimer {}

pub type WindowPtr = *mut Window;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tm {
    pub tm_sec: u32,
    pub tm_min: u32,
    pub tm_hour: u32,
    pub tm_mday: u32,
    pub tm_mon: u32,
    pub tm_year: u32,
    pub tm_wday: u32,
    pub tm_yday: u32,
    pub tm_isdst: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct GPoint {
    pub x: i16,
    pub y: i16,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct GSize {
    pub w: i16,
    pub h: i16,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct GRect {
    pub origin: GPoint,
    pub size: GSize,
}

pub type WindowHandler = extern "C" fn(WindowPtr);

#[repr(C)]
pub struct WindowHandlers {
    pub load: Option<WindowHandler>,
    pub appear: Option<WindowHandler>,
    pub disappear: Option<WindowHandler>,
    pub unload: Option<WindowHandler>,
}

#[repr(C)]
pub enum GCompOp {
    GCompOpAssign,
    GCompOpAssignInverted,
    GCompOpOr,
    GCompOpAnd,
    GCompOpClear,
    GCompOpSet,
}

#[repr(C)]
pub enum GColor {
    GColorClear = -1,
    GColorBlack = 0,
    GColorWhite = 1,
}

#[repr(C)]
pub enum TimeUnits {
    SECOND_UNIT = 1,
    MINUTE_UNIT,
    HOUR_UNIT,
    DAY_UNIT,
    MONTH_UNIT,
    YEAR_UNIT,
}

pub type ResHandle = c_void;

#[repr(C)]
pub struct FontInfo {
    _data: [u8; 0],
}

pub type GFont = *mut FontInfo;

#[repr(C, align(1))]
#[derive(Copy, Clone, BitfieldStruct)]
pub struct Tuple {
    pub key: u32,
    #[bitfield(name = "t_type", ty = "u8", bits = "32..=39")]
    #[bitfield(name = "length", ty = "u16", bits = "40..=55")]
    pub t_type: [u8; 2],
    value: TupleValue,
}

impl Tuple {
    unsafe fn read(&self) -> Option<TupleValue> {
        unsafe {
            let ptr = (&self.key as *const u32) as usize;
            let value_ptr = ptr + 7;
            let t = self.t_type[0];
            match t {
                0 => Some(TupleValue {
                    data: value_ptr as *const u8,
                }),
                1 => Some(TupleValue {
                    cstring: value_ptr as *const u8,
                }),
                2 => {
                    let val_ptr = value_ptr as *const u32;
                    Some(TupleValue {
                        uint32: ptr::read_unaligned(val_ptr),
                    })
                }
                3 => {
                    let val_ptr = value_ptr as *const i32;
                    Some(TupleValue {
                        int32: ptr::read_unaligned(val_ptr),
                    })
                }
                _ => None,
            }
        }
    }

    pub fn get_string(&self) -> Option<&CStr> {
        unsafe {
            let opt = self.get_value();
            if let Some(opt) = opt {
                let c_str = CStr::from_ptr(opt.cstring as *const core::ffi::c_char);
                Some(c_str)
            } else {
                None
            }
        }
    }

    pub fn get_i32(&self) -> Option<i32> {
        unsafe {
            if self.t_type[0] == 3 {
                self.get_value().map(|val| val.int32)
            } else {
                None
            }
        }
    }

    pub fn get_u32(&self) -> Option<u32> {
        unsafe {
            if self.t_type[0] == 2 {
                self.get_value().map(|val| val.uint32)
            } else {
                None
            }
        }
    }

    pub fn get_value(&self) -> Option<TupleValue> {
        unsafe { self.read() }
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union TupleValue {
    data: *const u8,
    cstring: *const u8,
    pub uint32: u32,
    pub int32: i32,

    placeholder: [u8; u8::MAX as usize + 325usize],
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum TupleType {
    ByteArray,
    Cstring,
    Uint,
    Int,
}

#[repr(C)]
pub struct Dictionary {
    _data: [u8; 0],
}

#[repr(C)]
pub struct DictionaryIterator {
    pub dict: *mut Dictionary,
    pub end: *const c_void,
    pub cursor: *mut Tuple,
}

#[repr(u8)]
pub enum DictionaryResult {
    DICT_OK,
    DICT_NOT_ENOUGH_STORAGE,
    DICT_INVALID_ARGS,
    DICT_INTERNAL_INCONSISTENCY,
    DICT_MALLOC_FAILED,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AppMessageResult {
    Ok = 0,
    SendTimeout = 2,
    SendRejected = 4,
    NotConnected = 8,
    AppNotRunning = 16,
    InvalidArgs = 32,
    Busy = 64,
    BufferOverflow = 128,
    AlreadyReleased = 512,
    CallbackAlreadyRegistered = 1024,
    CallbackNotRegistered = 2048,
    OutOfMemory = 4096,
    Closed = 8192,
    InternalError = 16384,
    InvalidState = 32768,
    Unknown = -1,
}

impl From<i32> for AppMessageResult {
    fn from(val: i32) -> Self {
        match val {
            0 => AppMessageResult::Ok,
            2 => AppMessageResult::SendTimeout,
            4 => AppMessageResult::SendRejected,
            8 => AppMessageResult::NotConnected,
            16 => AppMessageResult::AppNotRunning,
            32 => AppMessageResult::InvalidArgs,
            64 => AppMessageResult::Busy,
            128 => AppMessageResult::BufferOverflow,
            512 => AppMessageResult::AlreadyReleased,
            1024 => AppMessageResult::CallbackAlreadyRegistered,
            2048 => AppMessageResult::CallbackNotRegistered,
            4096 => AppMessageResult::OutOfMemory,
            8192 => AppMessageResult::Closed,
            16384 => AppMessageResult::InternalError,
            32768 => AppMessageResult::InvalidState,
            _ => AppMessageResult::Unknown,
        }
    }
}

#[repr(C)]
pub struct BatteryChargeState {
    pub charge_percent: u8,
    pub is_charging: bool,
    pub is_plugged: bool,
}

#[repr(C)]
pub struct ConnectionHandlers {
    pub app: extern "C" fn(bool),
    pub pebblekit: extern "C" fn(bool),
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct MenuIndex {
    pub section: u16,
    pub row: u16,
}

pub type MenuLayerGetNumberOfSectionsCallback = extern "C" fn(*mut MenuLayer, *mut c_void) -> u16;
pub type MenuLayerGetNumberOfRowsInSectionsCallback =
    extern "C" fn(*mut MenuLayer, u16, *mut c_void) -> u16;
pub type MenuLayerGetCellHeightCallback =
    extern "C" fn(*mut MenuLayer, *mut MenuIndex, *mut c_void) -> i16;
pub type MenuLayerGetHeaderHeightCallback = extern "C" fn(*mut MenuLayer, u16, *mut c_void) -> i16;
pub type MenuLayerGetSeparatorHeightCallback =
    extern "C" fn(*mut MenuLayer, *mut MenuIndex, *mut c_void) -> i16;
pub type MenuLayerDrawRowCallback =
    extern "C" fn(*mut GContext, *const Layer, *mut MenuIndex, *mut c_void);
pub type MenuLayerDrawHeaderCallback = extern "C" fn(*mut GContext, *const Layer, u16, *mut c_void);
pub type MenuLayerDrawSeparatorCallback =
    extern "C" fn(*mut GContext, *const Layer, *mut MenuIndex, *mut c_void);
pub type MenuLayerSelectCallback = extern "C" fn(*mut MenuLayer, *mut MenuIndex, *mut c_void);
pub type MenuLayerSelectionChangedCallback =
    extern "C" fn(*mut MenuLayer, MenuIndex, MenuIndex, *mut c_void);

pub type MenuLayerSelectionWillChangeCallback =
    extern "C" fn(*mut MenuLayer, *mut MenuIndex, MenuIndex, *mut c_void);
pub type MenuLayerDrawBackgroundCallback =
    extern "C" fn(*mut GContext, *const Layer, bool, *mut c_void);

#[repr(C)]
pub struct MenuLayerCallbacks {
    pub get_num_sections: Option<MenuLayerGetNumberOfSectionsCallback>,
    pub get_num_rows: Option<MenuLayerGetNumberOfRowsInSectionsCallback>,
    pub get_cell_height: Option<MenuLayerGetCellHeightCallback>,
    pub get_header_height: Option<MenuLayerGetHeaderHeightCallback>,
    pub draw_row: Option<MenuLayerDrawRowCallback>,
    pub draw_header: Option<MenuLayerDrawHeaderCallback>,
    pub select_click: Option<MenuLayerSelectCallback>,
    pub select_long_click: Option<MenuLayerSelectCallback>,
    pub selection_changed: Option<MenuLayerSelectionChangedCallback>,
    pub get_separator_height: Option<MenuLayerGetSeparatorHeightCallback>,
    pub draw_separator: Option<MenuLayerDrawSeparatorCallback>,
    pub selection_will_change: Option<MenuLayerSelectionWillChangeCallback>,
    pub draw_background: Option<MenuLayerDrawBackgroundCallback>,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MenuRowAlign {
    None = 0,
    Center = 1,
    Top = 2,
    Bottom = 3,
}

#[repr(u32)]
pub enum GTextAlignment {
    Left = 0,
    Center = 1,
    Right = 2,
}

#[repr(C)]
pub struct VibePattern {
    pub durations: *const u32,
    pub num_segments: u32,
}

pub type AppTimerCallback = extern "C" fn(data: *mut c_void);

pub type Status = i32;

pub const PERSIST_DATA_MAX_LENGTH: usize = 256;
pub const PERSIST_STRING_MAX_LENGTH: usize = 256;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StatusCode {
    Success = 0,
    Error = -1,
    Unknown = -2,
    Internal = -3,
    InvalidArgument = -4,
    OutOfMemory = -5,
    OutOfStorage = -6,
    OutOfResources = -7,
    Range = -8,
    DoesNotExist = -9,
    InvalidOperation = -10,
    Busy = -11,
    Again = -12,
    True = 1,
    False = 2,
    NoMoreItems = 3,
    NoActionRequired = 4,
}

impl From<i32> for StatusCode {
    fn from(value: i32) -> Self {
        match value {
            0 => StatusCode::Success,
            -1 => StatusCode::Error,
            -2 => StatusCode::Unknown,
            -3 => StatusCode::Internal,
            -4 => StatusCode::InvalidArgument,
            -5 => StatusCode::OutOfMemory,
            -6 => StatusCode::OutOfStorage,
            -7 => StatusCode::OutOfResources,
            -8 => StatusCode::Range,
            -9 => StatusCode::DoesNotExist,
            -10 => StatusCode::InvalidOperation,
            -11 => StatusCode::Busy,
            -12 => StatusCode::Again,
            1 => StatusCode::True,
            2 => StatusCode::False,
            3 => StatusCode::NoMoreItems,
            4 => StatusCode::NoActionRequired,
            _ => StatusCode::Unknown, // Fallback for undefined status
        }
    }
}

/// Identifier for a wakeup event
pub type WakeupId = i32;

/// The type of function which can be called when a wakeup event occurs.
pub type WakeupHandler = extern "C" fn(wakeup_id: WakeupId, cookie: i32);

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AppLaunchReason {
    System = 0,
    User = 1,
    Phone = 2,
    Wakeup = 3,
    Worker = 4,
    QuickLaunch = 5,
    TimelineAction = 6,
    Smartstrap = 7,
}

impl From<u32> for AppLaunchReason {
    fn from(value: u32) -> Self {
        match value {
            0 => AppLaunchReason::System,
            1 => AppLaunchReason::User,
            2 => AppLaunchReason::Phone,
            3 => AppLaunchReason::Wakeup,
            4 => AppLaunchReason::Worker,
            5 => AppLaunchReason::QuickLaunch,
            6 => AppLaunchReason::TimelineAction,
            7 => AppLaunchReason::Smartstrap,
            _ => AppLaunchReason::System,
        }
    }
}

pub type time_t = c_uint;

pub type ClickConfigProvider = extern "C" fn(*mut c_void);
pub type ClickRecognizerRef = *mut c_void;
pub type ClickHandler = extern "C" fn(recognizer: ClickRecognizerRef, context: *mut c_void);

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ButtonId {
    Back,
    Up,
    Select,
    Down,
    NumButtons,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ActionBarLayerIconPressAnimation {
    None,
    MoveLeft,
    MoveUp,
    MoveRight,
    MoveDown,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StatusBarLayerSeparatorMode {
    None,
    Dotted,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GBitmapFormat {
    Format1Bit,
    Format8Bit,
    Format1BitPalette,
    Format2BitPalette,
    Format4BitPalette,
    Format8BitCircular,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GCornerMask {
    CornerNone,
    CornerTopLeft,
    CornerTopRight,
    CornerBottomLeft,
    CornerBottomRight,
    CornersAll,
    CornersTop,
    CornersBottom,
    CornersLeft,
    CornersRight,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GOvalScaleMode {
    FitCircle,
    FillCircle,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GColor8 {
    pub argb: u8,
}
