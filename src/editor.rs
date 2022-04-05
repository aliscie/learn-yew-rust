use wasm_bindgen::UnwrapThrowExt;
use web_sys::{Element, window};
use yew::prelude::*;

use initial_value::initial_value;
use initial_value_handler::initial_value_handler;

mod initial_value_handler;

mod initial_value;

mod drag_handler;

#[function_component(App)]
pub fn app() -> Html {
    let html = initial_value();
    let my_text_handle = use_state(|| "".to_string());
    let my_text = (*my_text_handle).clone();
    let doc = window().unwrap_throw().document().unwrap_throw();
    use_effect_with_deps(move |_my_text| {
        let my_dom_element: &Element = &doc.get_element_by_id("1").unwrap_throw();
        initial_value_handler(my_dom_element, html);
        drag_handler::drag_handler(my_dom_element);
        || {}
    }, my_text);


    let _onkeydown: Callback<KeyboardEvent> = Callback::from(|e: KeyboardEvent| {
        if e.key() == "Enter" {
            e.prevent_default();
        }
    });
    html! {
    <div
        id="1"
        class="text_editor"
        contenteditable="true"
     >
    </div>
    }
}