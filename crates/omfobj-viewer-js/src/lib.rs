mod inner;
mod surface;

use std::{cell::RefCell, rc::Rc};

use tracing::{event, Level};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use wgpu::Instance;

use crate::inner::ViewerInner;

/// omfobj-viewer web instance.
#[wasm_bindgen]
pub struct Viewer {
    #[allow(dead_code)]
    handle: ViewerHandle,
}

#[wasm_bindgen]
impl Viewer {
    /// Create a new instance that renders to a given target canvas.
    pub async fn from_canvas(target: JsValue) -> Viewer {
        init_hooks();

        event!(Level::INFO, "creating omfobj-viewer");

        let instance = Instance::default();

        // Create the surface
        let target: HtmlCanvasElement = target
            .dyn_into()
            .expect("given target is not a canvas element");
        let surface = surface::create(&instance, &target);

        // Create the JS viewer instance
        let inner = ViewerInner::new(instance, surface).await;
        let handle = create_handle(inner);
        schedule_tick(handle.clone());

        Self { handle }
    }

    pub fn add_object(&self, _object: u32) {
        event!(Level::INFO, "adding object to viewer");
        // Placeholder function
    }
}

fn create_handle(inner: ViewerInner) -> ViewerHandle {
    let wrapper = ViewerWrapper {
        inner,
        keepalive: None,
    };
    Rc::new(RefCell::new(wrapper))
}

fn tick(handle: ViewerHandle) {
    handle.borrow_mut().inner.tick();

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
    inner: ViewerInner,
    keepalive: Option<Closure<dyn Fn()>>,
}

/// Initialize global hooks that may not yet be initialized.
fn init_hooks() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
}
