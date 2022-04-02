use wasm_bindgen::UnwrapThrowExt;
use web_sys::{window};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let html = r#"<span class="text_editor">
    <h1 draggable="true" title="title is here">hello world</h1>
    <h2 title="title is here">heading 2</h2>
    <h3 title="title is here">heading 2</h3>
    <h4 title="title is here">heading 2</h4>
    <h5 title="title is here">heading 2</h5>
    <h6 title="title is here">heading 2</h6>
    <p>hi paragraph</p></span>"#;


    let my_text_handle = use_state(|| "".to_string());
    let my_text = (*my_text_handle).clone();
    let doc = window().unwrap_throw().document().unwrap_throw();
    use_effect_with_deps(move |my_text| {
        let my_dom_element = &doc.get_element_by_id("1").unwrap_throw();
        my_dom_element.set_inner_html(html);
        &doc.query_selector("h1").unwrap().unwrap().set_attribute("class", "text-3xl");
        &doc.query_selector("h2").unwrap().unwrap().set_attribute("class", "text-2xl");
        &doc.query_selector("h3").unwrap().unwrap().set_attribute("class", "text-xl");
        &doc.query_selector("h4").unwrap().unwrap().set_attribute("class", "text-lg");
        &doc.query_selector("h5").unwrap().unwrap().set_attribute("class", "text-base");
        &doc.query_selector("h6").unwrap().unwrap().set_attribute("class", "text-sm");

        // &doc.query_selector("h1").unwrap().unwrap().insert_before("<span>⠿</span>", TextHtml);
        || {}
    }, my_text);


    let onkeydown: Callback<KeyboardEvent> = Callback::from(|e: KeyboardEvent| {
        if e.key() == "Enter" {
            e.prevent_default();
        }
    });


    html! {
    <div id="1" class="text_editor" contenteditable="true">
    </div>
    }
}