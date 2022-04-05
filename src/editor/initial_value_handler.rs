use web_sys::{Element, window};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::console::log_1;

pub fn initial_value_handler(editor: &Element, value: &str) {
    let doc = window().unwrap_throw().document().unwrap_throw();
    editor.set_inner_html(value);
    // log_1(&format!("{:?}", &doc.query_selector_all("h2").unwrap()).into());
}