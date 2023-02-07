use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    set_panic_hook();
    alert("Hello, omfobject-viewer!");
}

fn set_panic_hook() {
    console_error_panic_hook::set_once();
}
