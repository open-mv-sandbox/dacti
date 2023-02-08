mod triangle;

use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle, WebDisplayHandle,
    WebWindowHandle,
};
use tracing::{event, Level};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

/// Create a viewer instance on a given canvas.
#[wasm_bindgen]
pub fn create_viewer(target: JsValue) {
    init_hooks();

    event!(Level::INFO, "creating viewer");

    let target: HtmlCanvasElement = target
        .dyn_into()
        .expect("given target is not a canvas element");

    // Attach a reference to the canvas to find it
    target.dataset().set("rawHandle", "42").unwrap();
    let handle = RawCanvasHandle(42);

    // Hook WGPU onto the canvas
    let instance = wgpu::Instance::default();
    let surface = unsafe { instance.create_surface(&handle) }.unwrap();

    wasm_bindgen_futures::spawn_local(triangle::run(instance, surface));
}

struct RawCanvasHandle(u32);

unsafe impl HasRawDisplayHandle for RawCanvasHandle {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        RawDisplayHandle::Web(WebDisplayHandle::empty())
    }
}

unsafe impl HasRawWindowHandle for RawCanvasHandle {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = WebWindowHandle::empty();
        handle.id = self.0;
        RawWindowHandle::Web(handle)
    }
}

/// Initialize global hooks that may not yet be initialized.
fn init_hooks() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
}
