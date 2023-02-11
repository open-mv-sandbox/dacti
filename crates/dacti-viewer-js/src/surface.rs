use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle, WebDisplayHandle,
    WebWindowHandle,
};
use web_sys::HtmlCanvasElement;
use wgpu::{Instance, Surface};

pub fn create(instance: &Instance, context: &HtmlCanvasElement) -> Surface {
    // Attach a reference to the canvas to find it
    let id = (js_sys::Math::random() * 100_000_000.0) as u32;
    let id_str = id.to_string();
    context.dataset().set("rawHandle", id_str.as_str()).unwrap();
    let handle = RawCanvasHandle(id);

    unsafe { instance.create_surface(&handle) }.unwrap()
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
