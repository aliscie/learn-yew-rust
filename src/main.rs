extern crate yew;
extern crate web_sys;
extern crate wasm_bindgen;

// use yew::prelude::*;
mod editor;
use editor::App;


fn main() {
    yew::start_app::<App>();
}

// #[wasm_bindgen(start)]
// pub fn run_app() {App::<App>::new().mount_to_body();}