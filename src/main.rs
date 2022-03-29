extern crate yew;
extern crate web_sys;

// use yew::prelude::*;
mod app;

use app::App;


fn main() {
    yew::start_app::<App>();
}

// #[wasm_bindgen(start)]
// pub fn run_app() {App::<App>::new().mount_to_body();}