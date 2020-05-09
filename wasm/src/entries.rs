//! This module contains the entry points callable from javascript

extern crate console_error_panic_hook;
use std::panic;
use wasm_bindgen::prelude::*;

use crate::game_context::GameContext;
use rask_wasm_shared::get_double_buffer;
use rask_wasm_shared::mem;
use rask_wasm_shared::state::State;
use std::panic;
use wasm_bindgen::prelude::*;

use crate::context;

fn reset_state() {
    let mut writer = get_double_buffer().borrow_writer();
    writer.set(State::default());
}

fn wait_for_main_thread_notify() {
    unsafe { mem::SynchronizationMemory::get_mut() }.wait_for_main_thread_notify()
}

/// Initialize the gamestate, communicate with
/// the graphics worker and set up networking.
/// This function is being exposed to javascript
#[wasm_bindgen]
pub fn run_main_loop() {
    unsafe {
        rask_wasm_shared::alloc::reset_heap(&crate::ALLOCATOR, log::LevelFilter::Debug);
    }
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    log::info!("table count: {}", mem::RESOURCE_TABLE_ELEMENT_COUNT);
    log::info!("queue count: {}", mem::MESSAGE_QUEUE_ELEMENT_COUNT);
    log::info!("buffer count: {}", mem::DOUBLE_BUFFER_SPRITE_COUNT);
    reset_state();
    let mut game = GameContext::new().unwrap_or_else(|e| panic!("{}", e));
    loop {
        log::info!("a");
        wait_for_main_thread_notify();
        log::info!("b");
        game.tick().map_err(|e| panic!("{}", e)).unwrap();
        log::info!("wait_for_main_thread_notify()");
    }
}

#[allow(dead_code)]
/// This function is being exposed to javascript
#[wasm_bindgen]
pub fn initialise_graphics_context(canvas: web_sys::OffscreenCanvas) {
    unsafe {
        rask_wasm_shared::alloc::reset_heap(&crate::ALLOCATOR, log::LevelFilter::Debug);
    }
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    log::info!("graphics entry reached");

    context::set_context(
        context::Context::new(canvas)
            .map_err(|e| panic!("{}", e))
            .unwrap(),
    );
}

#[allow(dead_code)]
/// This function is being exposed to javascript
#[wasm_bindgen]
pub fn draw_frame() {
    let _ = context::context_mut()
        .render()
        .map_err(|e| log::error!("{}", e));
}