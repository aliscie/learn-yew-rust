use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Video {
    pub(crate) id: usize,
    pub(crate) title: String,
    pub(crate) speaker: String,
    pub(crate) url: String,
}

#[derive(Properties, PartialEq)]
pub struct RenderedAtProps {
    pub title: String,
    pub videos: Vec<Video>,
}


#[function_component(Videos)]
pub fn videos(props: &RenderedAtProps) -> Html {
    html! {
    <>
    <p>{&props.title}</p>
    <div id="introductions">
        {
             props.videos.clone().into_iter().map(|name| {
                html!{<div class="card mb-3" style="max-width: 18rem;">
                <div  class="card-title" key={name.id}>{ format!("Title: {}!",name.title) }</div>
                <div class="card-text" key={name.id}>{ format!("speaker: {}!",name.speaker) }</div>
                <a href={format!("{}",name.url)} class="btn btn-primary" key={name.id}>{ "watch now"}</a>
                </div>}
            }).collect::<Html>()
        }
    </div>

    </>
    }
}