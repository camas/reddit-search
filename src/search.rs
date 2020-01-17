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
                self.searching = false;
                self.comment_data.clear();
                self.comment_data.extend(data);
                true
            }
            Msg::SearchFail(_) => {
                self.searching = false;
                true
            }
        }
    }

    fn view(&self) -> Html {
        let button_text = if self.searching {
            "Searching..."
        } else {
            "Search"
        };
        html! {
            <>
            <div class="flex flex-col mx-auto px-1 max-w-3xl pb-1 mb-3">
                <div class="sm:flex">
                    <div class="sm:w-1/2">
                        <label class="block text-xs uppercase font-bold">{"Author"}</label>
                        <input class="text-gray-900 bg-gray-400 focus:bg-gray-100 w-full py-2 px-1" />
                    </div>
                    <div class="sm:w-1/2 sm:ml-1">
                        <label class="block text-xs uppercase font-bold">{"Subreddit"}</label>
                        <input class="text-gray-900 bg-gray-400 focus:bg-gray-100 w-full py-2 px-1" />
                    </div>
                </div>
                <div>
                    <label class="block text-xs uppercase font-bold">{"Search Term"}</label>
                    <input class="text-gray-900 bg-gray-400 focus:bg-gray-100 w-full py-2 px-1" />
                </div>
                <button type={"button"} onclick=self.link.callback(|_| Msg::DoSearch) class="bg-red-900 hover:bg-red-800 font-bold mt-4 py-2 px-4">{button_text}</button>
            </div>
            <div class="max-w-5xl m-auto">
                { for self.comment_data.iter().map(|x| view_comment(x)) }
            </div>
            </>
        }
    }
}

fn view_comment(data: &CommentData) -> Html {
    let naive = chrono::NaiveDateTime::from_timestamp(data.created_utc, 0);
    let created = chrono::DateTime::<chrono::Utc>::from_utc(naive, chrono::Utc);
    let time_string = created.format("%Y-%m-%d %H:%M:%S").to_string();
    html! {
        <div class="bg-gray-900 mx-1 px-1 mb-1">
            <div class="flex">
                <a href={format!("https://reddit.com/r/{}", data.subreddit)}>
                    <div class="text-sm text-red-500">{format!("/r/{}", data.subreddit)}</div>
                </a>
                <a href={format!("https://reddit.com/u/{}", data.author)}>
                    <div class="text-sm text-red-500 ml-2">{format!("/u/{}", data.author)}</div>
                </a>
                <div class="text-sm text-red-500 ml-auto">{time_string}</div>
            </div>
            <a href={format!("https://reddit.com{}", data.permalink)}>
                <div>{&data.body}</div>
            </a>
        </div>
    }
}
