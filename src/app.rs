use yew::prelude::*;
use html_minifier::HTMLMinifier;
mod video_list;
use video_list::Videos;

#[function_component(App)]
pub fn app() -> Html {
    let onkeydown: Callback<KeyboardEvent> = Callback::from(|e: KeyboardEvent| {
        if e.key() == "Enter" {
            e.prevent_default();
        }
    });

    html! {
    <div class="text_editor">
    // <Videos/>
    <h1 onkeydown={&onkeydown} contenteditable="true" >{"title"}</h1>
    <p  onkeydown={&onkeydown} clas="editable" data-placeholder="Edit me..." contenteditable="true" >{ "" }</p>
    </div>
    }
}