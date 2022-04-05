use std::cell::Cell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{Element, HtmlFormElement, window};
use web_sys::console::log_1;
use std::thread::sleep;
use std::time::Duration;


pub fn drag_handler(editor: &Element) {
    let X_value = editor.get_bounding_client_rect().left() + 50 as f64;
    let doc = window().unwrap_throw().document().unwrap_throw();
    let body = doc.query_selector("body").unwrap_throw().unwrap_throw();
    let mut prev = 0__f64;
    // let window_width = window().unwrap().inner_width().unwrap();
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        let mut Y_value = event.page_y();
        // let mut X_value = event.page_y();

        let curr = doc.element_from_point(X_value as f32, Y_value as f32).unwrap();

        // toto
        //get nested elements like <li>
        log_1(&format!("{:#?}", curr.inner_html()).into());
        // for i in  doc.elements_from_point(X_value as f32, Y_value as f32).iter(){
        //     log_1(&format!("{:#?}", i.children()).into());
        // }

        if prev != curr.get_bounding_client_rect().top() {
            if doc.get_element_by_id("drag") != None {
                let prev_drag_icon = doc.query_selector("drag-icon").unwrap().unwrap();
                prev_drag_icon.remove();
                prev = curr.get_bounding_client_rect().top();
            }

            let type_: &str = &curr.node_name()[..];
            if !["DIV"].contains(&type_) {
                let drag_ic = doc.create_element("drag-icon").unwrap();
                let top = curr.get_bounding_client_rect().top() as f32;
                let left = curr.get_bounding_client_rect().left() as f32 - 18 as f32;
                drag_ic.set_inner_html(
                    &format!(r#"<drag-icon style="position: absolute; left:{}px;" id="drag"><svg viewBox="0 0 10 10" class="dragHandle" style="width: 14px; height: 14px; display: block; fill: inherit; flex-shrink: 0; backface-visibility: hidden;"><path d="M3,2 C2.44771525,2 2,1.55228475 2,1 C2,0.44771525 2.44771525,0 3,0 C3.55228475,0 4,0.44771525 4,1 C4,1.55228475 3.55228475,2 3,2 Z M3,6 C2.44771525,6 2,5.55228475 2,5 C2,4.44771525 2.44771525,4 3,4 C3.55228475,4 4,4.44771525 4,5 C4,5.55228475 3.55228475,6 3,6 Z M3,10 C2.44771525,10 2,9.55228475 2,9 C2,8.44771525 2.44771525,8 3,8 C3.55228475,8 4,8.44771525 4,9 C4,9.55228475 3.55228475,10 3,10 Z M7,2 C6.44771525,2 6,1.55228475 6,1 C6,0.44771525 6.44771525,0 7,0 C7.55228475,0 8,0.44771525 8,1 C8,1.55228475 7.55228475,2 7,2 Z M7,6 C6.44771525,6 6,5.55228475 6,5 C6,4.44771525 6.44771525,4 7,4 C7.55228475,4 8,4.44771525 8,5 C8,5.55228475 7.55228475,6 7,6 Z M7,10 C6.44771525,10 6,9.55228475 6,9 C6,8.44771525 6.44771525,8 7,8 C7.55228475,8 8,8.44771525 8,9 C8,9.55228475 7.55228475,10 7,10 Z"></path></svg></drag-icon>"#, left)
                );
                curr.insert_before(&drag_ic.clone(), curr.first_child().as_ref());
            }
        }
    }) as Box<dyn FnMut(_)>);
    body.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref());
    closure.forget();
}