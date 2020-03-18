//! This module contains the entry points callable from javascript

use wasm_bindgen::prelude::*;

use crate::context;
use rask_wasm_shared::get_double_buffer;
use rask_wasm_shared::wasm_log::WasmLog;

/// This function is being exposed to javascript
#[wasm_bindgen]
pub fn initialise_graphics_context(canvas: web_sys::OffscreenCanvas) {
    unsafe {
        crate::ALLOCATOR.reset();
    }

    log::set_boxed_logger(Box::new(WasmLog::new()))
        .map(|()| log::set_max_level(log::LevelFilter::Debug))
        .unwrap();
    log::info!("graphics entry reached");

    context::set_context(
        context::Context::new(canvas)
            .map_err(|e| log::error!("{}", e))
            .unwrap(),
    );
}

/// This function is being exposed to javascript
#[wasm_bindgen]
pub fn draw_frame() {
    context::context_mut()
        .render()
        .map_err(|e| log::error!("{}", e))
        .unwrap();
}
