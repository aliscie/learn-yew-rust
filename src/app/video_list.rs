use yew::prelude::*;

// #[derive(Clone, PartialEq)]
pub struct Video {
     id: usize,
     title: String,
     speaker: String,
     url: String,
}

#[derive(Properties, PartialEq)]
pub struct RenderedAtProps {
     pub title: String,
     pub videos:  Vec<Video>,
}


#[function_component(Videos)]
pub fn videos(props: &RenderedAtProps) -> Html {
    html! {
    <>
    <p>{&props.title}</p>
    // <div id="introductions">
    //     {
    //         names.into_iter().map(|name| {
    //             html!{<div key={name}>{ format!("Hello, I'am {}!",name) }</div>}
    //         }).collect::<Html>()
    //     }
    // </div>

    </>
    }
}