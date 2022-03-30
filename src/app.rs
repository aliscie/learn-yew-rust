use yew::prelude::*;
use html_minifier::HTMLMinifier;




#[function_component(App)]
pub fn app() -> Html {
    let onkeydown: Callback<KeyboardEvent> = Callback::from(|e: KeyboardEvent| {
        if e.key() == "Enter" {
            e.prevent_default();
        }
    });
    // let window: web_sys::Window = web_sys::window().expect("window not available");
    // window.alert_with_message("hello from wasm!").expect("alert failed");


    html! {
    <>
    <div class="max-w-md py-4 px-8 bg-white shadow-lg rounded-lg my-20">
  <div class="flex justify-center md:justify-end -mt-16">
    <img class="w-20 h-20 object-cover rounded-full border-2 border-indigo-500" src="https://images.unsplash.com/photo-1499714608240-22fc6ad53fb2?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=334&q=80"/>
  </div>
  <div>
    <h2 class="text-gray-800 text-3xl font-semibold">{"Hello tailwindui"}</h2>
    <p class="mt-2 text-gray-600">{"Lorem ipsum dolor sit amet consectetur adipisicing elit. Quae dolores deserunt ea doloremque natus error, rerum quas odio quaerat nam ex commodi hic, suscipit in a veritatis pariatur minus consequuntur!"}</p>
  </div>
  <div class="flex justify-end mt-4">
    <a href="#" class="text-xl font-medium text-indigo-500">{"John Doe"}</a>
  </div>
  </div>

    <h1 onkeydown={&onkeydown} contenteditable="true" >{"title"}</h1>
    <p  onkeydown={&onkeydown} clas="editable" data-placeholder="Edit me..." contenteditable="true" >{ "" }</p>
    </>
    }
}