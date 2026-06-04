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
use crate::pebble::internal::types::*;
use crate::types::{DictPtr, VoidPtr};
use core::ffi::{c_char, c_int, c_void};

unsafe extern "C" {
    // App
    pub fn app_event_loop();

    // Window
    pub fn window_create() -> WindowPtr;
    pub fn window_destroy(window: WindowPtr);
    pub fn window_set_click_config_provider(
        window: *mut Window,
        provider: Option<ClickConfigProvider>,
    );
    pub fn window_set_click_config_provider_with_context(
        window: *mut Window,
        provider: Option<ClickConfigProvider>,
        context: *mut c_void,
    );
    pub fn window_set_click_context(button_id: ButtonId, context: *mut c_void);
    pub fn window_set_window_handlers(window: WindowPtr, handlers: WindowHandlers);
    pub fn window_set_background_color(window: WindowPtr, color: GColor);
    pub fn window_set_user_data(window: WindowPtr, data: *mut c_void);
    pub fn window_get_user_data(window: WindowPtr) -> *mut c_void;
    pub fn window_stack_push(window: WindowPtr, animated: bool);
    pub fn window_stack_pop(animated: bool) -> WindowPtr;
    pub fn window_stack_pop_all(animated: bool);
    pub fn window_stack_remove(window: WindowPtr, animated: bool) -> bool;
    pub fn window_stack_get_top_window() -> WindowPtr;
    pub fn window_stack_contains_window(window: WindowPtr) -> bool;
    pub fn window_get_root_layer(window: WindowPtr) -> *mut Layer;
    pub fn window_single_click_subscribe(button_id: ButtonId, handler: Option<ClickHandler>);
    pub fn window_single_repeating_click_subscribe(
        button_id: ButtonId,
        repeat_interval_ms: u16,
        handler: Option<ClickHandler>,
    );
    pub fn window_multi_click_subscribe(
        button_id: ButtonId,
        min_clicks: u8,
        max_clicks: u8,
        timeout: u16,
        last_click_only: bool,
        handler: Option<ClickHandler>,
    );
    pub fn window_long_click_subscribe(
        button_id: ButtonId,
        delay_ms: u16,
        down_handler: Option<ClickHandler>,
        up_handler: Option<ClickHandler>,
    );
    pub fn window_raw_click_subscribe(
        button_id: ButtonId,
        down_handler: Option<ClickHandler>,
        up_handler: Option<ClickHandler>,
        context: *mut c_void,
    );

    // Layer
    pub fn layer_create(bounds: GRect) -> *mut Layer;
    pub fn layer_destroy(layer: *mut Layer);
    pub fn layer_get_frame(layer: *mut Layer) -> GRect;
    pub fn layer_get_bounds(layer: *mut Layer) -> GRect;
    pub fn layer_add_child(layer: *mut Layer, child: *mut Layer);
    pub fn layer_mark_dirty(layer: *mut Layer);
    pub fn layer_set_update_proc(layer: *mut Layer, func: extern "C" fn(*mut Layer, *mut GContext));

    // TextLayer
    pub fn text_layer_create(bounds: GRect) -> *mut TextLayer;
    pub fn text_layer_destroy(text_layer: *mut TextLayer);
    pub fn text_layer_set_text(layer: *mut TextLayer, text: *const c_char);
    pub fn text_layer_get_layer(layer: *mut TextLayer) -> *mut Layer;
    pub fn text_layer_set_font(layer: *mut TextLayer, font: GFont);
    pub fn text_layer_set_text_alignment(
        text_layer: *mut TextLayer,
        text_alignment: GTextAlignment,
    );

    // GBitmap
    pub fn gbitmap_create_with_resource(id: u32) -> *mut GBitmap;
    pub fn gbitmap_destroy(bitmap: *mut GBitmap);

    // BitmapLayer
    pub fn bitmap_layer_create(frame: GRect) -> *mut BitmapLayer;
    pub fn bitmap_layer_destroy(bitmap_layer: *mut BitmapLayer);
    pub fn bitmap_layer_set_bitmap(layer: *mut BitmapLayer, bitmap: *mut GBitmap);
    pub fn bitmap_layer_set_compositing_mode(layer: *mut BitmapLayer, mode: GCompOp);
    pub fn bitmap_layer_get_layer(layer: *mut BitmapLayer) -> *mut Layer;

    // Menu
    pub fn menu_layer_create(bounds: GRect) -> *mut MenuLayer;
    pub fn menu_layer_destroy(menu_layer: *mut MenuLayer);
    pub fn menu_layer_get_layer(menu_layer: *mut MenuLayer) -> *mut Layer;
    pub fn menu_layer_set_callbacks(
        menu_layer: *mut MenuLayer,
        context: *mut c_void,
        callbacks: MenuLayerCallbacks,
    );
    pub fn menu_layer_set_click_config_onto_window(menu_layer: *mut MenuLayer, window: WindowPtr);
    pub fn menu_layer_reload_data(menu_layer: *mut MenuLayer);

    pub fn menu_cell_basic_draw(
        ctx: *mut GContext,
        cell_layer: *const Layer,
        title: *const c_char,
        subtitle: *const c_char,
        icon: *mut GBitmap,
    );
    pub fn menu_cell_title_draw(ctx: *mut GContext, cell_layer: *const Layer, title: *const c_char);
    pub fn menu_cell_basic_header_draw(
        ctx: *mut GContext,
        cell_layer: *const Layer,
        title: *const c_char,
    );
    pub fn menu_index_compare(a: *const MenuIndex, b: *const MenuIndex) -> i16;
    pub fn menu_layer_get_scroll_layer(menu_layer: *const MenuLayer) -> *mut Layer; // TODO: Technically ScrollLayer*, mapped to Layer*
    pub fn menu_layer_set_selected_next(
        menu_layer: *mut MenuLayer,
        up: bool,
        scroll_align: MenuRowAlign,
        animated: bool,
    );
    pub fn menu_layer_set_selected_index(
        menu_layer: *mut MenuLayer,
        index: MenuIndex,
        scroll_align: MenuRowAlign,
        animated: bool,
    );
    pub fn menu_layer_get_selected_index(menu_layer: *const MenuLayer) -> MenuIndex;
    pub fn menu_cell_layer_is_highlighted(cell_layer: *const Layer) -> bool;
    pub fn menu_layer_set_normal_colors(
        menu_layer: *mut MenuLayer,
        background: GColor,
        foreground: GColor,
    );
    pub fn menu_layer_set_highlight_colors(
        menu_layer: *mut MenuLayer,
        background: GColor,
        foreground: GColor,
    );
    pub fn menu_layer_pad_bottom_enable(menu_layer: *mut MenuLayer, enable: bool);
    pub fn menu_layer_get_center_focused(menu_layer: *const MenuLayer) -> bool;
    pub fn menu_layer_set_center_focused(menu_layer: *mut MenuLayer, center_focused: bool);
    pub fn menu_layer_is_index_selected(
        menu_layer: *const MenuLayer,
        index: *const MenuIndex,
    ) -> bool;

    // Graphics
    pub fn graphics_context_set_stroke_color(ctx: *mut GContext, color: GColor);
    pub fn graphics_context_set_fill_color(ctx: *mut GContext, color: GColor);
    pub fn graphics_context_set_text_color(ctx: *mut GContext, color: GColor);
    pub fn graphics_context_set_compositing_mode(ctx: *mut GContext, mode: GCompOp);
    pub fn graphics_context_set_antialiased(ctx: *mut GContext, enable: bool);
    pub fn graphics_context_set_stroke_width(ctx: *mut GContext, stroke_width: u8);

    // Graphics - Primitives
    pub fn graphics_draw_pixel(ctx: *mut GContext, center: GPoint);
    pub fn graphics_draw_line(ctx: *mut GContext, p0: GPoint, p1: GPoint);
    pub fn graphics_draw_rect(ctx: *mut GContext, rect: GRect);
    pub fn graphics_fill_rect(
        ctx: *mut GContext,
        rect: GRect,
        corner_radius: u16,
        corner_mask: GCornerMask,
    );
    pub fn graphics_draw_circle(ctx: *mut GContext, center: GPoint, radius: u16);
    pub fn graphics_fill_circle(ctx: *mut GContext, center: GPoint, radius: u16);
    pub fn graphics_draw_round_rect(ctx: *mut GContext, rect: GRect, radius: u16);
    pub fn graphics_draw_bitmap_in_rect(ctx: *mut GContext, bitmap: *const GBitmap, rect: GRect);
    pub fn graphics_capture_frame_buffer(ctx: *mut GContext) -> *mut GBitmap;
    pub fn graphics_capture_frame_buffer_format(
        ctx: *mut GContext,
        format: GBitmapFormat,
    ) -> *mut GBitmap;
    pub fn graphics_release_frame_buffer(ctx: *mut GContext, buffer: *mut GBitmap) -> bool;
    pub fn graphics_frame_buffer_is_captured(ctx: *mut GContext) -> bool;
    pub fn graphics_draw_rotated_bitmap(
        ctx: *mut GContext,
        src: *mut GBitmap,
        src_ic: GPoint,
        rotation: c_int,
        dest_ic: GPoint,
    );
    pub fn graphics_draw_arc(
        ctx: *mut GContext,
        rect: GRect,
        scale_mode: GOvalScaleMode,
        angle_start: i32,
        angle_end: i32,
    );
    pub fn graphics_fill_radial(
        ctx: *mut GContext,
        rect: GRect,
        scale_mode: GOvalScaleMode,
        inset_thickness: u16,
        angle_start: i32,
        angle_end: i32,
    );
    pub fn gpoint_from_polar(rect: GRect, scale_mode: GOvalScaleMode, angle: i32) -> GPoint;
    pub fn grect_centered_from_polar(
        rect: GRect,
        scale_mode: GOvalScaleMode,
        angle: i32,
        size: GSize,
    ) -> GRect;

    // Wall Time
    pub fn clock_copy_time_string(buffer: *mut c_char, size: u8);
    pub fn clock_is_24h_style() -> u8;
    pub fn clock_get_timezone(buffer: *mut c_char, size: usize);

    pub fn tick_timer_service_subscribe(unit: TimeUnits, func: extern "C" fn(*mut tm, TimeUnits));

    // Standard C - Time
    pub fn time(t: *mut time_t) -> time_t;
    pub fn localtime(now: *const time_t) -> *mut tm;
    pub fn gmtime(now: *const time_t) -> *mut tm;

    // Standard C - Locale
    pub fn setlocale(category: i32, locale: *const c_char) -> *const c_char;

    // Standard C - Math
    pub fn rand() -> i32;
    pub fn srand(seed: u32) -> i32;

    // Standard C - Strings
    pub fn strcmp(str1: *const c_char, str2: *const c_char) -> i32;
    pub fn strncmp(str1: *const c_char, str2: *const c_char, num_bytes: usize) -> i32;
    pub fn strcpy(destination: *const c_char, source: *const c_char) -> *const c_char;
    pub fn strncpy(
        destination: *const c_char,
        source: *const c_char,
        num_bytes: usize,
    ) -> *const c_char;
    pub fn strcat(destination: *const c_char, source: *const c_char) -> *const c_char;
    pub fn strncat(
        destination: *const c_char,
        source: *const c_char,
        num_bytes: usize,
    ) -> *const c_char;
    pub fn strlen(str: *const c_char) -> usize;

    // Standard C - Format
    pub fn snprintf(buf: *const c_char, max: usize, fmt: *const c_char, ...) -> usize;

    // Fonts
    pub fn fonts_get_system_font(key: *const c_char) -> GFont;
    pub fn fonts_load_custom_font(res: ResHandle) -> GFont;
    pub fn fonts_unload_custom_font(font: GFont);

    // Resources
    pub fn resource_get_handle(id: u32) -> ResHandle;

    // Dictionary
    pub fn dict_calc_buffer_size(tuple_count: u8) -> u32;
    pub fn dict_size(iter: *mut DictionaryIterator) -> u32;
    pub fn dict_write_begin(
        iter: *mut DictionaryIterator,
        buffer: *mut u8,
        size: u16,
    ) -> DictionaryResult;
    pub fn dict_write_data(
        iter: *mut DictionaryIterator,
        key: u32,
        data: *mut u8,
        size: u16,
    ) -> DictionaryResult;
    pub fn dict_write_cstring(
        iter: *mut DictionaryIterator,
        key: u32,
        cstring: *const c_char,
    ) -> DictionaryResult;
    pub fn dict_write_int(
        iter: *mut DictionaryIterator,
        key: u32,
        int: *const c_void,
        len_bytes: u8,
        signed: bool,
    ) -> DictionaryResult;
    pub fn dict_write_end(iter: *mut DictionaryIterator) -> u32;
    pub fn dict_read_begin_from_buffer(
        iter: *mut DictionaryIterator,
        buffer: *mut u8,
        size: u16,
    ) -> *mut Tuple;
    pub fn dict_read_next(iter: *mut DictionaryIterator) -> *mut Tuple;
    pub fn dict_read_first(iter: *mut DictionaryIterator) -> *mut Tuple;
    pub fn dict_find(iter: *mut DictionaryIterator, key: u32) -> *mut Tuple;

    // AppMessage
    pub fn app_message_open(size_inbound: u32, size_outbound: u32) -> i32;
    pub fn app_message_inbox_size_maximum() -> u32;
    pub fn app_message_outbox_size_maximum() -> u32;

    // Callbacks
    pub fn app_message_register_inbox_received(
        callback: extern "C" fn(iter: DictPtr, ctx: VoidPtr),
    );
    pub fn app_message_register_inbox_dropped(callback: extern "C" fn(reason: i32, ctx: VoidPtr));
    pub fn app_message_register_outbox_sent(callback: extern "C" fn(iter: DictPtr, ctx: VoidPtr));
    pub fn app_message_register_outbox_failed(
        callback: extern "C" fn(iter: DictPtr, reason: i32, ctx: VoidPtr),
    );

    // Outgoing Messages
    pub fn app_message_outbox_begin(iterator: *mut DictPtr) -> i32;
    pub fn app_message_outbox_send() -> i32;

    // EVENTS
    // Battery
    pub fn battery_state_service_subscribe(handler: extern "C" fn(state: BatteryChargeState));
    pub fn battery_state_service_unsubscribe();
    pub fn battery_state_service_peek() -> BatteryChargeState;

    // Connection
    pub fn connection_service_peek_pebble_app_connection() -> bool;
    pub fn connection_service_peek_pebblekit_connection() -> bool;
    pub fn connection_service_unsubscribe();
    pub fn connection_service_subscribe(handlers: ConnectionHandlers);

    // Logging
    pub fn app_log(level: u8, filename: *const c_char, line_num: u32, msg: *const c_char, ...);

    // Persistent Storage
    pub fn persist_exists(key: u32) -> bool;
    pub fn persist_get_size(key: u32) -> i32;
    pub fn persist_read_bool(key: u32) -> bool;
    pub fn persist_read_int(key: u32) -> i32;
    pub fn persist_read_data(key: u32, buffer: *mut c_void, buffer_size: usize) -> i32;
    pub fn persist_read_string(key: u32, buffer: *mut c_char, buffer_size: usize) -> i32;

    pub fn persist_write_bool(key: u32, value: bool) -> Status;
    pub fn persist_write_int(key: u32, value: i32) -> Status;
    pub fn persist_write_data(key: u32, data: *const c_void, size: usize) -> i32;
    pub fn persist_write_string(key: u32, cstring: *const c_char) -> i32;

    pub fn persist_delete(key: u32) -> Status;

    // Vibration
    pub fn vibes_cancel();
    pub fn vibes_short_pulse();
    pub fn vibes_long_pulse();
    pub fn vibes_double_pulse();
    pub fn vibes_enqueue_custom_pattern(pattern: VibePattern);

    // App Timer
    pub fn psleep(millis: i32);

    pub fn app_timer_register(
        timeout_ms: u32,
        callback: AppTimerCallback,
        callback_data: *mut c_void,
    ) -> *mut AppTimer;

    pub fn app_timer_reschedule(timer_handle: *mut AppTimer, new_timeout_ms: u32) -> bool;

    pub fn app_timer_cancel(timer_handle: *mut AppTimer);

    // Wakeups
    pub fn wakeup_service_subscribe(handler: WakeupHandler);
    pub fn wakeup_schedule(timestamp: time_t, cookie: i32, notify_if_missed: bool) -> WakeupId;
    pub fn wakeup_cancel(wakeup_id: WakeupId);
    pub fn wakeup_cancel_all();
    pub fn wakeup_get_launch_event(wakeup_id: *mut WakeupId, cookie: *mut i32) -> bool;
    pub fn wakeup_query(wakeup_id: WakeupId, timestamp: *mut time_t) -> bool;

    // Launch reason
    pub fn launch_reason() -> u32;
    pub fn launch_get_args() -> u32;

    // Action Bar Layer
    pub fn action_bar_layer_create() -> *mut ActionBarLayer;
    pub fn action_bar_layer_destroy(action_bar_layer: *mut ActionBarLayer);
    pub fn action_bar_layer_get_layer(action_bar_layer: *mut ActionBarLayer) -> *mut Layer;
    pub fn action_bar_layer_set_context(
        action_bar: *mut ActionBarLayer,
        context: *mut core::ffi::c_void,
    );
    pub fn action_bar_layer_set_click_config_provider(
        action_bar: *mut ActionBarLayer,
        click_config_provider: ClickConfigProvider,
    );
    pub fn action_bar_layer_set_icon(
        action_bar: *mut ActionBarLayer,
        button_id: ButtonId,
        icon: *const GBitmap,
    );
    pub fn action_bar_layer_clear_icon(action_bar: *mut ActionBarLayer, button_id: ButtonId);
    pub fn action_bar_layer_add_to_window(action_bar: *mut ActionBarLayer, window: *mut Window);
    pub fn action_bar_layer_remove_from_window(action_bar: *mut ActionBarLayer);
    pub fn action_bar_layer_set_background_color(
        action_bar: *mut ActionBarLayer,
        background_color: GColor,
    );
    pub fn action_bar_layer_set_icon_animated(
        action_bar: *mut ActionBarLayer,
        button_id: ButtonId,
        icon: *const GBitmap,
        animated: bool,
    );
    pub fn action_bar_layer_set_icon_press_animation(
        action_bar: *mut ActionBarLayer,
        button_id: ButtonId,
        animation: ActionBarLayerIconPressAnimation,
    );

    // Status Bar Layer
    pub fn status_bar_layer_create() -> *mut StatusBarLayer;
    pub fn status_bar_layer_destroy(status_bar_layer: *mut StatusBarLayer);
    pub fn status_bar_layer_get_layer(status_bar_layer: *mut StatusBarLayer) -> *mut Layer;
    pub fn status_bar_layer_get_background_color(status_bar_layer: *const StatusBarLayer)
    -> GColor;
    pub fn status_bar_layer_get_foreground_color(status_bar_layer: *const StatusBarLayer)
    -> GColor;
    pub fn status_bar_layer_set_colors(
        status_bar_layer: *mut StatusBarLayer,
        background: GColor,
        foreground: GColor,
    );
    pub fn status_bar_layer_set_separator_mode(
        status_bar_layer: *mut StatusBarLayer,
        mode: StatusBarLayerSeparatorMode,
    );

    // Click Recognizer
    pub fn click_number_of_clicks_counted(recognizer: ClickRecognizerRef) -> u8;
    pub fn click_recognizer_get_button_id(recognizer: ClickRecognizerRef) -> ButtonId;
    pub fn click_recognizer_is_repeating(recognizer: ClickRecognizerRef) -> bool;
}
