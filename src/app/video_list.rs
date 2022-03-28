#[derive(Clone, PartialEq)]
pub struct Video {
    pub(crate) id: usize,
    pub(crate) title: String,
    pub(crate) speaker: String,
    pub(crate) url: String,
}

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
}

#[function_component(VideosList)]
fn videos_list(VideosListProps { videos }: &VideosListProps) -> Html {
    videos.iter().map(|video| html! {
        <p>{format!("{}: {}", video.speaker, video.title)}</p>
    }).collect()
}