use yew::prelude::*;
use html_minifier::HTMLMinifier;
use std::ops::Deref;


// fn inheritance_of_text_area(text_area: HtmlTextAreaElement) {
//     // HtmlTextAreaElement is <textarea> in html.
//     let html_element: &HtmlElement = text_area.deref();
//
//     let element: &Element = html_element.deref();
//
//     let node: &Node = element.deref();
//
//     let event_target: &EventTarget = node.deref();
//
//     // Notice we've moved from web-sys types now into built-in
//     // JavaScript types which are in the js-sys crate.
//     let object: &js_sys::Object = event_target.deref();
//
//     // Notice we've moved from js-sys type to the root JsValue from
//     // the wasm-bindgen crate.
//     let js_value: &wasm_bindgen::JsValue = object.deref();
//
//     // Using deref like this means we have to manually traverse
//     // the inheritance tree, however, you can call JsValue methods
//     // on the HtmlTextAreaElement type.
//     // The `is_string` method comes from JsValue.
//     assert!(!text_area.is_string());
//
//     // empty function just to prove we can pass HtmlTextAreaElement as a
//     // &EventTarget.
//     fn this_function_only_takes_event_targets(targets: &EventTarget) {};
//
//     // The compiler will walk down the deref chain in order to match the types here.
//     this_function_only_takes_event_targets(&text_area);
//
//     // The AsRef implementations allow you to treat the HtmlTextAreaElement
//     // as an &EventTarget.
//
//     let event_target: &EventTarget = text_area.as_ref();
//
// }


#[function_component(App)]
pub fn app() -> Html {
    let onkeydown: Callback<KeyboardEvent> = Callback::from(|e: KeyboardEvent| {
        if e.key() == "Enter" {
            e.prevent_default();
        }
    });
    let window: web_sys::Window = web_sys::window().expect("window not available");
    window.alert_with_message("hello from wasm!").expect("alert failed");


    html! {
    <>
    <h1 onkeydown={&onkeydown} contenteditable="true" >{"title"}</h1>
    <p  onkeydown={&onkeydown} clas="editable" data-placeholder="Edit me..." contenteditable="true" >{ "paragraph" }</p>
    </>
    }
}