use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

// #[derive(Clone, Properties, PartialEq)]
// struct VideosListProps {
//     videos: Vec<Video>,
//     on_click: Callback<Video>,
// }

#[derive(Properties, PartialEq)]
pub struct RenderedAtProps {
    pub new_prop: String,
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
    let names = vec!["Sam","Bob","Ray"];


    html! {
    <>
    <p>{"h"}</p>
    <div id="introductions">
        {
            names.into_iter().map(|name| {
                html!{<div key={name}>{ format!("Hello, I'am {}!",name) }</div>}
            }).collect::<Html>()
        }
    </div>

    </>
    }
}