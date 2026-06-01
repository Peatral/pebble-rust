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
#![allow(unused)]

use core::mem;

use crate::pebble::internal::types::*;
use core::ffi::{CStr, c_char, c_void};

use crate::pebble::internal::functions::declarations;
use crate::types::{DictPtr, VoidPtr};

pub fn app_event_loop() {
    unsafe {
        declarations::app_event_loop();
    }
}

pub fn window_create() -> WindowPtr {
    unsafe { declarations::window_create() }
}

pub fn window_destroy(window: WindowPtr) {
    unsafe {
        declarations::window_destroy(window);
    }
}

pub fn window_set_click_config_provider(
    window: *mut Window,
    provider: Option<ClickConfigProvider>,
) {
    unsafe { declarations::window_set_click_config_provider(window, provider) }
}

pub fn window_set_click_config_provider_with_context(
    window: *mut Window,
    provider: Option<ClickConfigProvider>,
    context: *mut c_void,
) {
    unsafe {
        declarations::window_set_click_config_provider_with_context(window, provider, context)
    }
}

pub fn window_set_click_context(button_id: ButtonId, context: *mut c_void) {
    unsafe { declarations::window_set_click_context(button_id, context) }
}

pub fn window_set_window_handlers(window: WindowPtr, handlers: WindowHandlers) {
    unsafe {
        declarations::window_set_window_handlers(window, handlers);
    }
}

pub fn window_set_background_color(window: WindowPtr, color: GColor) {
    unsafe {
        declarations::window_set_background_color(window, color);
    }
}

pub fn window_set_user_data<T>(window: WindowPtr, data: *mut T) {
    unsafe {
        declarations::window_set_user_data(window, data as *mut c_void);
    }
}

pub fn window_get_user_data<T>(window: WindowPtr) -> *mut T {
    unsafe { declarations::window_get_user_data(window) as *mut T }
}

pub fn window_stack_push(window: WindowPtr, animated: bool) {
    unsafe { declarations::window_stack_push(window, animated) }
}

pub fn window_stack_pop(animated: bool) -> WindowPtr {
    unsafe { declarations::window_stack_pop(animated) }
}

pub fn window_stack_pop_all(animated: bool) {
    unsafe { declarations::window_stack_pop_all(animated) }
}

pub fn window_stack_remove(window: WindowPtr, animated: bool) -> bool {
    unsafe { declarations::window_stack_remove(window, animated) }
}

pub fn window_stack_get_top_window() -> WindowPtr {
    unsafe { declarations::window_stack_get_top_window() }
}

pub fn window_stack_contains_window(window: WindowPtr) -> bool {
    unsafe { declarations::window_stack_contains_window(window) }
}

pub fn window_get_root_layer(window: WindowPtr) -> *mut Layer {
    unsafe { declarations::window_get_root_layer(window) }
}

pub fn window_single_click_subscribe(button_id: ButtonId, handler: Option<ClickHandler>) {
    unsafe { declarations::window_single_click_subscribe(button_id, handler) }
}

pub fn window_single_repeating_click_subscribe(
    button_id: ButtonId,
    repeat_interval_ms: u16,
    handler: Option<ClickHandler>,
) {
    unsafe {
        declarations::window_single_repeating_click_subscribe(
            button_id,
            repeat_interval_ms,
            handler,
        )
    }
}

pub fn window_multi_click_subscribe(
    button_id: ButtonId,
    min_clicks: u8,
    max_clicks: u8,
    timeout: u16,
    last_click_only: bool,
    handler: Option<ClickHandler>,
) {
    unsafe {
        declarations::window_multi_click_subscribe(
            button_id,
            min_clicks,
            max_clicks,
            timeout,
            last_click_only,
            handler,
        )
    }
}

pub fn window_long_click_subscribe(
    button_id: ButtonId,
    delay_ms: u16,
    down_handler: Option<ClickHandler>,
    up_handler: Option<ClickHandler>,
) {
    unsafe {
        declarations::window_long_click_subscribe(button_id, delay_ms, down_handler, up_handler)
    }
}

pub fn window_raw_click_subscribe(
    button_id: ButtonId,
    down_handler: Option<ClickHandler>,
    up_handler: Option<ClickHandler>,
    context: *mut c_void,
) {
    unsafe {
        declarations::window_raw_click_subscribe(button_id, down_handler, up_handler, context)
    }
}

pub fn layer_create(bounds: GRect) -> *mut Layer {
    unsafe { declarations::layer_create(bounds) }
}

pub fn layer_destroy(layer: *mut Layer) {
    unsafe {
        declarations::layer_destroy(layer);
    }
}

pub fn layer_get_frame(layer: *mut Layer) -> GRect {
    unsafe { declarations::layer_get_frame(layer) }
}

pub fn layer_get_bounds(layer: *mut Layer) -> GRect {
    unsafe { declarations::layer_get_bounds(layer) }
}

pub fn layer_add_child(layer: *mut Layer, child: *mut Layer) {
    unsafe {
        declarations::layer_add_child(layer, child);
    }
}

pub fn layer_mark_dirty(layer: *mut Layer) {
    unsafe {
        declarations::layer_mark_dirty(layer);
    }
}

pub fn layer_set_update_proc(layer: *mut Layer, func: extern "C" fn(*mut Layer, *mut GContext)) {
    unsafe {
        declarations::layer_set_update_proc(layer, func);
    }
}

pub fn text_layer_create(bounds: GRect) -> *mut TextLayer {
    unsafe { declarations::text_layer_create(bounds) }
}
pub fn text_layer_destroy(text_layer: *mut TextLayer) {
    unsafe {
        declarations::text_layer_destroy(text_layer);
    }
}

pub fn text_layer_set_text(layer: *mut TextLayer, text: &CStr) {
    unsafe {
        declarations::text_layer_set_text(layer, text.as_ptr());
    }
}

pub fn text_layer_set_font(layer: *mut TextLayer, font: GFont) {
    unsafe {
        declarations::text_layer_set_font(layer, font);
    }
}

pub fn text_layer_get_layer(layer: *mut TextLayer) -> *mut Layer {
    unsafe { declarations::text_layer_get_layer(layer) }
}

pub fn text_layer_set_text_alignment(layer: *mut TextLayer, text_alignment: GTextAlignment) {
    unsafe { declarations::text_layer_set_text_alignment(layer, text_alignment) }
}

pub fn gbitmap_create_with_resource(id: u32) -> *mut GBitmap {
    unsafe { declarations::gbitmap_create_with_resource(id) }
}

pub fn bitmap_layer_create(frame: GRect) -> *mut BitmapLayer {
    unsafe { declarations::bitmap_layer_create(frame) }
}

pub fn bitmap_layer_destroy(bitmap_layer: *mut BitmapLayer) {
    unsafe {
        declarations::bitmap_layer_destroy(bitmap_layer);
    }
}

pub fn bitmap_layer_set_bitmap(layer: *mut BitmapLayer, bitmap: *mut GBitmap) {
    unsafe {
        declarations::bitmap_layer_set_bitmap(layer, bitmap);
    }
}

pub fn bitmap_layer_set_compositing_mode(layer: *mut BitmapLayer, mode: GCompOp) {
    unsafe {
        declarations::bitmap_layer_set_compositing_mode(layer, mode);
    }
}

pub fn bitmap_layer_get_layer(layer: *mut BitmapLayer) -> *mut Layer {
    unsafe { declarations::bitmap_layer_get_layer(layer) }
}

pub fn menu_layer_create(bounds: GRect) -> *mut MenuLayer {
    unsafe { declarations::menu_layer_create(bounds) }
}

pub fn menu_layer_destroy(menu_layer: *mut MenuLayer) {
    unsafe {
        declarations::menu_layer_destroy(menu_layer);
    }
}
pub fn menu_layer_get_layer(menu_layer: *mut MenuLayer) -> *mut Layer {
    unsafe { declarations::menu_layer_get_layer(menu_layer) }
}

pub fn menu_layer_set_callbacks(
    menu_layer: *mut MenuLayer,
    context: *mut c_void,
    callbacks: MenuLayerCallbacks,
) {
    unsafe {
        declarations::menu_layer_set_callbacks(menu_layer, context, callbacks);
    }
}

pub fn menu_layer_set_click_config_onto_window(menu_layer: *mut MenuLayer, window: WindowPtr) {
    unsafe {
        declarations::menu_layer_set_click_config_onto_window(menu_layer, window);
    }
}
pub fn menu_layer_reload_data(menu_layer: *mut MenuLayer) {
    unsafe {
        declarations::menu_layer_reload_data(menu_layer);
    }
}

pub fn graphics_context_set_fill_color(ctx: *mut GContext, color: GColor) {
    unsafe {
        declarations::graphics_context_set_fill_color(ctx, color);
    }
}

pub fn graphics_fill_circle(ctx: *mut GContext, center: GPoint, radius: u16) {
    unsafe {
        declarations::graphics_fill_circle(ctx, center, radius);
    }
}

pub fn clock_is_24h_style() -> bool {
    unsafe {
        let response = declarations::clock_is_24h_style();
        response != 0
    }
}

pub fn tick_timer_service_subscribe(unit: TimeUnits, func: extern "C" fn(*mut tm, TimeUnits)) {
    unsafe {
        declarations::tick_timer_service_subscribe(unit, func);
    }
}

pub fn time() -> time_t {
    unsafe { declarations::time(core::ptr::null_mut()) }
}

pub fn localtime(now: time_t) -> *mut tm {
    unsafe {
        let now_ptr = &now as *const time_t;
        declarations::localtime(now_ptr)
    }
}

pub fn gmtime(now: time_t) -> *mut tm {
    unsafe {
        let now_ptr = &now as *const time_t;
        declarations::gmtime(now_ptr)
    }
}

pub fn app_log(level: u8, msg: &CStr, name: &CStr) {
    unsafe {
        declarations::app_log(level, name.as_ptr(), 2, msg.as_ptr());
    }
}

pub fn vibes_cancel() {
    unsafe { declarations::vibes_cancel() }
}
pub fn vibes_short_pulse() {
    unsafe { declarations::vibes_short_pulse() }
}
pub fn vibes_long_pulse() {
    unsafe { declarations::vibes_long_pulse() }
}
pub fn vibes_double_pulse() {
    unsafe { declarations::vibes_double_pulse() }
}
pub fn vibes_enqueue_custom_pattern(durations: &'static [u32]) {
    let pattern = VibePattern {
        durations: durations.as_ptr(),
        num_segments: durations.len() as u32,
    };
    unsafe { declarations::vibes_enqueue_custom_pattern(pattern) }
}

pub fn psleep(millis: i32) {
    unsafe { declarations::psleep(millis) }
}

pub fn app_timer_register(
    timeout_ms: u32,
    callback: AppTimerCallback,
    callback_data: *mut c_void,
) -> *mut AppTimer {
    unsafe { declarations::app_timer_register(timeout_ms, callback, callback_data) }
}

pub fn app_timer_reschedule(timer_handle: *mut AppTimer, new_timeout_ms: u32) -> bool {
    unsafe { declarations::app_timer_reschedule(timer_handle, new_timeout_ms) }
}

pub fn app_timer_cancel(timer_handle: *mut AppTimer) {
    unsafe { declarations::app_timer_cancel(timer_handle) }
}

pub fn persist_exists(key: u32) -> bool {
    unsafe { declarations::persist_exists(key) }
}

pub fn persist_get_size(key: u32) -> i32 {
    unsafe { declarations::persist_get_size(key) }
}

pub fn persist_read_bool(key: u32) -> bool {
    unsafe { declarations::persist_read_bool(key) }
}

pub fn persist_read_int(key: u32) -> i32 {
    unsafe { declarations::persist_read_int(key) }
}

pub fn persist_read_data(key: u32, buffer: *mut c_void, buffer_size: usize) -> i32 {
    unsafe { declarations::persist_read_data(key, buffer, buffer_size) }
}

pub fn persist_read_string(key: u32, buffer: *mut c_char, buffer_size: usize) -> i32 {
    unsafe { declarations::persist_read_string(key, buffer, buffer_size) }
}

pub fn persist_write_bool(key: u32, value: bool) -> Status {
    unsafe { declarations::persist_write_bool(key, value) }
}

pub fn persist_write_int(key: u32, value: i32) -> Status {
    unsafe { declarations::persist_write_int(key, value) }
}

pub fn persist_write_data(key: u32, data: *const c_void, size: usize) -> i32 {
    unsafe { declarations::persist_write_data(key, data, size) }
}

pub fn persist_write_string(key: u32, cstring: &CStr) -> i32 {
    unsafe { declarations::persist_write_string(key, cstring.as_ptr()) }
}

pub fn persist_delete(key: u32) -> Status {
    unsafe { declarations::persist_delete(key) }
}

pub fn wakeup_service_subscribe(handler: WakeupHandler) {
    unsafe { declarations::wakeup_service_subscribe(handler) }
}

pub fn wakeup_schedule(timestamp: time_t, cookie: i32, notify_if_missed: bool) -> WakeupId {
    unsafe { declarations::wakeup_schedule(timestamp, cookie, notify_if_missed) }
}

pub fn wakeup_cancel(wakeup_id: WakeupId) {
    unsafe { declarations::wakeup_cancel(wakeup_id) }
}

pub fn wakeup_cancel_all() {
    unsafe { declarations::wakeup_cancel_all() }
}

pub fn wakeup_get_launch_event(wakeup_id: *mut WakeupId, cookie: *mut i32) -> bool {
    unsafe { declarations::wakeup_get_launch_event(wakeup_id, cookie) }
}

pub fn wakeup_query(wakeup_id: WakeupId, timestamp: *mut time_t) -> bool {
    unsafe { declarations::wakeup_query(wakeup_id, timestamp) }
}

pub fn launch_reason() -> u32 {
    unsafe { declarations::launch_reason() }
}

pub fn launch_get_args() -> u32 {
    unsafe { declarations::launch_get_args() }
}

pub fn menu_cell_basic_draw(
    ctx: *mut GContext,
    cell_layer: *const Layer,
    title: &CStr,
    subtitle: &CStr,
    icon: *mut GBitmap,
) {
    unsafe {
        declarations::menu_cell_basic_draw(ctx, cell_layer, title.as_ptr(), subtitle.as_ptr(), icon)
    }
}
pub fn menu_cell_title_draw(ctx: *mut GContext, cell_layer: *const Layer, title: &CStr) {
    unsafe { declarations::menu_cell_title_draw(ctx, cell_layer, title.as_ptr()) }
}
pub fn menu_cell_basic_header_draw(ctx: *mut GContext, cell_layer: *const Layer, title: &CStr) {
    unsafe { declarations::menu_cell_basic_header_draw(ctx, cell_layer, title.as_ptr()) }
}
pub fn menu_index_compare(a: *const MenuIndex, b: *const MenuIndex) -> i16 {
    unsafe { declarations::menu_index_compare(a, b) }
}
pub fn menu_layer_get_scroll_layer(menu_layer: *const MenuLayer) -> *mut Layer {
    unsafe { declarations::menu_layer_get_scroll_layer(menu_layer) }
}
pub fn menu_layer_set_selected_next(
    menu_layer: *mut MenuLayer,
    up: bool,
    scroll_align: MenuRowAlign,
    animated: bool,
) {
    unsafe { declarations::menu_layer_set_selected_next(menu_layer, up, scroll_align, animated) }
}
pub fn menu_layer_set_selected_index(
    menu_layer: *mut MenuLayer,
    index: MenuIndex,
    scroll_align: MenuRowAlign,
    animated: bool,
) {
    unsafe {
        declarations::menu_layer_set_selected_index(menu_layer, index, scroll_align, animated)
    }
}
pub fn menu_layer_get_selected_index(menu_layer: *const MenuLayer) -> MenuIndex {
    unsafe { declarations::menu_layer_get_selected_index(menu_layer) }
}
pub fn menu_cell_layer_is_highlighted(cell_layer: *const Layer) -> bool {
    unsafe { declarations::menu_cell_layer_is_highlighted(cell_layer) }
}
pub fn menu_layer_set_normal_colors(
    menu_layer: *mut MenuLayer,
    background: GColor,
    foreground: GColor,
) {
    unsafe { declarations::menu_layer_set_normal_colors(menu_layer, background, foreground) }
}
pub fn menu_layer_set_highlight_colors(
    menu_layer: *mut MenuLayer,
    background: GColor,
    foreground: GColor,
) {
    unsafe { declarations::menu_layer_set_highlight_colors(menu_layer, background, foreground) }
}
pub fn menu_layer_pad_bottom_enable(menu_layer: *mut MenuLayer, enable: bool) {
    unsafe { declarations::menu_layer_pad_bottom_enable(menu_layer, enable) }
}
pub fn menu_layer_get_center_focused(menu_layer: *const MenuLayer) -> bool {
    unsafe { declarations::menu_layer_get_center_focused(menu_layer) }
}
pub fn menu_layer_set_center_focused(menu_layer: *mut MenuLayer, center_focused: bool) {
    unsafe { declarations::menu_layer_set_center_focused(menu_layer, center_focused) }
}
pub fn menu_layer_is_index_selected(menu_layer: *const MenuLayer, index: *const MenuIndex) -> bool {
    unsafe { declarations::menu_layer_is_index_selected(menu_layer, index) }
}

pub fn app_message_open(size_inbound: u32, size_outbound: u32) -> i32 {
    unsafe { declarations::app_message_open(size_inbound, size_outbound) }
}

pub fn app_message_inbox_size_maximum() -> u32 {
    unsafe { declarations::app_message_inbox_size_maximum() }
}

pub fn app_message_outbox_size_maximum() -> u32 {
    unsafe { declarations::app_message_outbox_size_maximum() }
}

pub fn app_message_register_inbox_received(callback: extern "C" fn(iter: DictPtr, ctx: VoidPtr)) {
    unsafe { declarations::app_message_register_inbox_received(callback) }
}

pub fn app_message_register_inbox_dropped(callback: extern "C" fn(reason: i32, ctx: VoidPtr)) {
    unsafe { declarations::app_message_register_inbox_dropped(callback) }
}

pub fn app_message_register_outbox_sent(callback: extern "C" fn(iter: DictPtr, ctx: VoidPtr)) {
    unsafe { declarations::app_message_register_outbox_sent(callback) }
}

pub fn app_message_register_outbox_failed(
    callback: extern "C" fn(iter: DictPtr, reason: i32, ctx: VoidPtr),
) {
    unsafe { declarations::app_message_register_outbox_failed(callback) }
}

pub fn app_message_outbox_begin(iterator: *mut DictPtr) -> i32 {
    unsafe { declarations::app_message_outbox_begin(iterator) }
}

pub fn app_message_outbox_send() -> i32 {
    unsafe { declarations::app_message_outbox_send() }
}

pub fn action_bar_layer_create() -> *mut ActionBarLayer {
    unsafe { declarations::action_bar_layer_create() }
}

pub fn action_bar_layer_destroy(action_bar_layer: *mut ActionBarLayer) {
    unsafe { declarations::action_bar_layer_destroy(action_bar_layer) }
}

pub fn action_bar_layer_get_layer(action_bar_layer: *mut ActionBarLayer) -> *mut Layer {
    unsafe { declarations::action_bar_layer_get_layer(action_bar_layer) }
}

pub fn action_bar_layer_set_context(
    action_bar: *mut ActionBarLayer,
    context: *mut core::ffi::c_void,
) {
    unsafe { declarations::action_bar_layer_set_context(action_bar, context) }
}

pub fn action_bar_layer_set_click_config_provider(
    action_bar: *mut ActionBarLayer,
    click_config_provider: ClickConfigProvider,
) {
    unsafe {
        declarations::action_bar_layer_set_click_config_provider(action_bar, click_config_provider)
    }
}

pub fn action_bar_layer_set_icon(
    action_bar: *mut ActionBarLayer,
    button_id: ButtonId,
    icon: *const GBitmap,
) {
    unsafe { declarations::action_bar_layer_set_icon(action_bar, button_id, icon) }
}

pub fn action_bar_layer_clear_icon(action_bar: *mut ActionBarLayer, button_id: ButtonId) {
    unsafe { declarations::action_bar_layer_clear_icon(action_bar, button_id) }
}

pub fn action_bar_layer_add_to_window(action_bar: *mut ActionBarLayer, window: *mut Window) {
    unsafe { declarations::action_bar_layer_add_to_window(action_bar, window) }
}

pub fn action_bar_layer_remove_from_window(action_bar: *mut ActionBarLayer) {
    unsafe { declarations::action_bar_layer_remove_from_window(action_bar) }
}

pub fn action_bar_layer_set_background_color(
    action_bar: *mut ActionBarLayer,
    background_color: GColor,
) {
    unsafe { declarations::action_bar_layer_set_background_color(action_bar, background_color) }
}

pub fn action_bar_layer_set_icon_animated(
    action_bar: *mut ActionBarLayer,
    button_id: ButtonId,
    icon: *const GBitmap,
    animated: bool,
) {
    unsafe {
        declarations::action_bar_layer_set_icon_animated(action_bar, button_id, icon, animated)
    }
}

pub fn action_bar_layer_set_icon_press_animation(
    action_bar: *mut ActionBarLayer,
    button_id: ButtonId,
    animation: ActionBarLayerIconPressAnimation,
) {
    unsafe {
        declarations::action_bar_layer_set_icon_press_animation(action_bar, button_id, animation)
    }
}

pub fn status_bar_layer_create() -> *mut StatusBarLayer {
    unsafe { declarations::status_bar_layer_create() }
}

pub fn status_bar_layer_destroy(status_bar_layer: *mut StatusBarLayer) {
    unsafe { declarations::status_bar_layer_destroy(status_bar_layer) }
}

pub fn status_bar_layer_get_layer(status_bar_layer: *mut StatusBarLayer) -> *mut Layer {
    unsafe { declarations::status_bar_layer_get_layer(status_bar_layer) }
}

pub fn status_bar_layer_get_background_color(status_bar_layer: *const StatusBarLayer) -> GColor {
    unsafe { declarations::status_bar_layer_get_background_color(status_bar_layer) }
}

pub fn status_bar_layer_get_foreground_color(status_bar_layer: *const StatusBarLayer) -> GColor {
    unsafe { declarations::status_bar_layer_get_foreground_color(status_bar_layer) }
}

pub fn status_bar_layer_set_colors(
    status_bar_layer: *mut StatusBarLayer,
    background: GColor,
    foreground: GColor,
) {
    unsafe { declarations::status_bar_layer_set_colors(status_bar_layer, background, foreground) }
}

pub fn status_bar_layer_set_separator_mode(
    status_bar_layer: *mut StatusBarLayer,
    mode: StatusBarLayerSeparatorMode,
) {
    unsafe { declarations::status_bar_layer_set_separator_mode(status_bar_layer, mode) }
}

pub fn click_number_of_clicks_counted(recognizer: ClickRecognizerRef) -> u8 {
    unsafe { declarations::click_number_of_clicks_counted(recognizer) }
}

pub fn click_recognizer_get_button_id(recognizer: ClickRecognizerRef) -> ButtonId {
    unsafe { declarations::click_recognizer_get_button_id(recognizer) }
}

pub fn click_recognizer_is_repeating(recognizer: ClickRecognizerRef) -> bool {
    unsafe { declarations::click_recognizer_is_repeating(recognizer) }
}
