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
    let videos = vec![
        Video {
            id: 1,
            title: "Building and breaking things".to_string(),
            speaker: "John Doe".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 2,
            title: "The development process".to_string(),
            speaker: "Jane Smith".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 3,
            title: "The Web 7.0".to_string(),
            speaker: "Matt Miller".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 4,
            title: "Mouseless development".to_string(),
            speaker: "Tom Jerry".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
    ];

    html! {
    <>
    <p>{&props.title}</p>
    <div id="introductions">
        {
            videos.into_iter().map(|name| {
                html!{<div class="card mb-3" style="max-width: 18rem;">
                <div class="card-title" key={name.id}>{ format!("Title: {}!",name.title) }</div>
                <div class="card-text" key={name.id}>{ format!("speaker: {}!",name.speaker) }</div>
                <a href={format!("{}",name.url)} class="btn btn-primary" key={name.id}>{ "watch now"}</a>
                </div>}
            }).collect::<Html>()
        }
    </div>

    </>
    }
}