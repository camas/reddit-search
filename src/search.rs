use crate::data::{CommentData, CommentsData};
use log::*;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

#[derive(Debug)]
pub enum Msg {
    DoSearch,
    SearchSuccess(Vec<CommentData>),
    SearchFail(String),
}

pub struct Search {
    fetch_service: FetchService,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
    searching: bool,
    comment_data: Vec<CommentData>,
}

impl Search {
    fn search(&mut self) -> FetchTask {
        let callback = self
            .link
            .callback(|resp: Response<Json<Result<CommentsData, _>>>| {
                let (meta, Json(data)) = resp.into_parts();
                info!("Response from pushshift");
                if meta.status.is_success() {
                    match data {
                        Ok(d) => Msg::SearchSuccess(d.data),
                        Err(e) => Msg::SearchFail(format!("{}", e)),
                    }
                } else {
                    Msg::SearchFail(format!("Error: {}", meta.status))
                }
            });
        let url = "https://api.pushshift.io/reddit/comment/search";
        let req = Request::get(url).body(Nothing).unwrap();
        self.fetch_service.fetch(req, callback)
    }
}

impl Component for Search {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Search {
            fetch_service: FetchService::new(),
            link,
            searching: false,
            task: None,
            comment_data: Vec::new(),
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        info!("{:?}", message);
        match message {
            Msg::DoSearch => {
                self.searching = true;
                self.task = Some(self.search());
                true
            }
            Msg::SearchSuccess(data) => {
                self.comment_data.extend(data);
                true
            }
            Msg::SearchFail(_) => true,
        }
    }

    fn view(&self) -> Html {
        let mut lines: String = String::new();
        for c in self.comment_data.iter().map(|x| x.body.clone()) {
            lines += "\n";
            lines.push_str(&c);
        }
        html! {
            <>
                <button onclick=self.link.callback(|_| Msg::DoSearch)>{"Search"}</button>
                <h3>{"Comments"}</h3>
                <div>
                    { for self.comment_data.iter().map(|x| view_comment(x)) }
                </div>
            </>
        }
    }
}

fn view_comment(data: &CommentData) -> Html {
    html! {
        <>
            <div>{&data.author}</div>
            <div>{&data.body}</div>
        </>
    }
}
