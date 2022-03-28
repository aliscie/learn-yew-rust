use yew::prelude::*;

mod video_list;
use video_list::Videos;

// #[derive(Clone, PartialEq)]
// pub struct Video {
//     pub(crate) id: usize,
//     pub(crate) title: String,
//     pub(crate) speaker: String,
//     pub(crate) url: String,
// }

#[function_component(App)]
pub fn app() -> Html {
    // let _videos = vec![
    //     Video {
    //         id: 1,
    //         title: "Building and breaking things".to_string(),
    //         speaker: "John Doe".to_string(),
    //         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    //     },
    //     Video {
    //         id: 2,
    //         title: "The development process".to_string(),
    //         speaker: "Jane Smith".to_string(),
    //         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    //     },
    //     Video {
    //         id: 3,
    //         title: "The Web 7.0".to_string(),
    //         speaker: "Matt Miller".to_string(),
    //         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    //     },
    //     Video {
    //         id: 4,
    //         title: "Mouseless development".to_string(),
    //         speaker: "Tom Jerry".to_string(),
    //         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    //     },
    // ];
    //

    html! {
    <>
    <Videos />
      <h1>{ "From x." }</h1>
    </>
    }
}