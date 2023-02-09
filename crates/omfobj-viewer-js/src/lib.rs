mod instance;

use std::{cell::RefCell, rc::Rc};

use instance::ViewerInstance;
use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle, WebDisplayHandle,
    WebWindowHandle,
};
use tracing::{event, Level};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use wgpu::{Instance, Surface};

/// Create a viewer instance on a given canvas.
#[wasm_bindgen]
pub fn create_viewer(target: JsValue) {
    init_hooks();

    event!(Level::INFO, "creating viewer instance");

    let target: HtmlCanvasElement = target
        .dyn_into()
        .expect("given target is not a canvas element");

    // Attach a reference to the canvas to find it
    target.dataset().set("rawHandle", "42").unwrap();
    let handle = RawCanvasHandle(42);

    // Hook WGPU onto the canvas
    let instance = wgpu::Instance::default();
    let surface = unsafe { instance.create_surface(&handle) }.unwrap();

    // Create the JS viwere instance
    let future = spawn_instance(instance, surface);
    wasm_bindgen_futures::spawn_local(future);
}

async fn spawn_instance(instance: Instance, surface: Surface) {
    let viewer = ViewerInstance::new(instance, surface).await;
    let wrapper = ViewerWrapper {
        viewer,
        keepalive: None,
    };
    let handle = Rc::new(RefCell::new(wrapper));

    schedule_tick(handle);
}

fn tick(handle: ViewerHandle) {
    handle.borrow_mut().viewer.tick();

    schedule_tick(handle);
}

fn schedule_tick(handle: ViewerHandle) {
    let handle_c = handle.clone();
    let callback = Closure::<dyn Fn()>::new(move || tick(handle_c.clone()));

    let window = web_sys::window().unwrap();
    window
        .request_animation_frame(callback.as_ref().unchecked_ref())
        .unwrap();

    // Make closure own itself, thus the old one keeps getting cleaned up, eventually permanently when
    // the tick returns without re-scheduling.
    handle.borrow_mut().keepalive = Some(callback);
}

type ViewerHandle = Rc<RefCell<ViewerWrapper>>;

struct ViewerWrapper {
    viewer: ViewerInstance,
    keepalive: Option<Closure<dyn Fn()>>,
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
