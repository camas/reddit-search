use crate::data::{CommentsData, Entry, PostsData};
use log::*;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use yew::components::Select;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

#[derive(Clone)]
struct SearchOptions {
    author: Option<String>,
    subreddit: Option<String>,
    search_term: Option<String>,
    search_type: Option<SearchType>,
    num_returned: Option<u32>,
    score_filter: Option<String>,
}

impl SearchOptions {
    fn new() -> Self {
        SearchOptions {
            author: None,
            subreddit: None,
            search_term: None,
            search_type: Some(SearchType::Comments),
            num_returned: Some(100),
            score_filter: None,
        }
    }
}

#[derive(Clone, Debug, Display, EnumString, EnumIter, PartialEq)]
pub enum SearchType {
    Posts,
    Comments,
}

impl SearchType {
    fn to_endpoint(&self) -> String {
        match self {
            SearchType::Posts => "submission".to_string(),
            SearchType::Comments => "comment".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Msg {
    DoSearch,
    SearchSuccess(Vec<Box<dyn Entry>>),
    SearchFail(String),
    AuthorInput(String),
    SubredditInput(String),
    SearchInput(String),
    DoMore,
    SearchTypeInput(SearchType),
    ScoreFilterInput(String),
    NumReturnedInput(u32),
}

pub struct Search {
    fetch_service: FetchService,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
    searching: bool,
    entry_data: Vec<Box<dyn Entry>>,
    last_error: Option<String>,
    show_more_button: bool,
    moreing: bool,
    more_task: Option<FetchTask>,
    options: SearchOptions,
    used_options: SearchOptions,
}

impl Search {
    /// Creates and runs a request to the pushshift.io api
    fn search(&mut self, get_more: bool) -> FetchTask {
        // Create callback
        let search_type = self.used_options.search_type.clone().unwrap();
        // Create arg string
        let mut inputs = HashMap::new();
        let true_string = String::from("true");
        inputs.insert("html_decode", &true_string);
        if let Some(author) = &self.used_options.author {
            inputs.insert("author", &author);
        }
        if let Some(subreddit) = &self.used_options.subreddit {
            inputs.insert("subreddit", &subreddit);
        }
        if let Some(search_term) = &self.used_options.search_term {
            inputs.insert("q", &search_term);
        }
        let num_str;
        if let Some(num_returned) = &self.used_options.num_returned {
            num_str = num_returned.to_string();
            inputs.insert("size", &num_str);
        }
        if let Some(score_filter) = &self.used_options.score_filter {
            inputs.insert("score", &score_filter);
        }
        let created = if self.entry_data.is_empty() {
            0.to_string()
        } else {
            self.entry_data.last().unwrap().get_created().to_string()
        };
        if get_more && !self.entry_data.is_empty() {
            inputs.insert("before", &created);
        }
        let arg_str = if inputs.is_empty() {
            "".to_string()
        } else {
            let mut final_args = String::new();
            for (i, (k, v)) in inputs.iter().enumerate() {
                if i == 0 {
                    final_args.push('?');
                }
                final_args = format!("{}&{}={}", final_args, k, url_encode(v));
            }
            final_args
        };
        // Run request
        let endpoint = search_type.to_endpoint();
        let url = format!(
            "https://api.pushshift.io/reddit/{}/search{}",
            endpoint, arg_str
        );
        info!("{}", url);
        let req = Request::get(url).body(Nothing).unwrap();
        match search_type {
            SearchType::Comments => {
                let callback =
                    self.link
                        .callback(|resp: Response<Json<Result<CommentsData, _>>>| {
                            let (meta, Json(data)) = resp.into_parts();
                            info!("Response from pushshift");
                            if meta.status.is_success() {
                                match data {
                                    Ok(d) => Msg::SearchSuccess(
                                        d.data
                                            .into_iter()
                                            .map(|x| Box::new(x) as Box<dyn Entry>)
                                            .collect(),
                                    ),
                                    Err(e) => Msg::SearchFail(format!("{}", e)),
                                }
                            } else {
                                Msg::SearchFail(format!("Error: {}", meta.status))
                            }
                        });
                self.fetch_service.fetch(req, callback)
            }
            SearchType::Posts => {
                let callback = self
                    .link
                    .callback(|resp: Response<Json<Result<PostsData, _>>>| {
                        let (meta, Json(data)) = resp.into_parts();
                        info!("Response from pushshift");
                        if meta.status.is_success() {
                            match data {
                                Ok(d) => Msg::SearchSuccess(
                                    d.data
                                        .into_iter()
                                        .map(|x| Box::new(x) as Box<dyn Entry>)
                                        .collect(),
                                ),
                                Err(e) => Msg::SearchFail(format!("{}", e)),
                            }
                        } else {
                            Msg::SearchFail(format!("Error: {}", meta.status))
                        }
                    });
                self.fetch_service.fetch(req, callback)
            }
        }
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
            entry_data: Vec::new(),
            last_error: None,
            show_more_button: false,
            moreing: false,
            more_task: None,
            options: SearchOptions::new(),
            used_options: SearchOptions::new(),
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::DoSearch => {
                self.searching = true;
                self.used_options = self.options.clone();
                self.task = Some(self.search(false));
                true
            }
            Msg::DoMore => {
                self.moreing = true;
                self.more_task = Some(self.search(true));
                true
            }
            Msg::SearchSuccess(data) => {
                self.last_error = None;
                self.searching = false;
                self.show_more_button = !data.is_empty();
                if !self.moreing {
                    self.entry_data.clear();
                }
                self.moreing = false;
                self.entry_data.extend(data);
                true
            }
            Msg::SearchFail(err) => {
                error!("{}", err);
                self.last_error = Some(err);
                self.searching = false;
                self.show_more_button = false;
                true
            }
            Msg::AuthorInput(d) => {
                self.options.author = Some(d);
                false
            }
            Msg::SubredditInput(d) => {
                self.options.subreddit = Some(d);
                false
            }
            Msg::SearchInput(d) => {
                self.options.search_term = Some(d);
                false
            }
            Msg::SearchTypeInput(d) => {
                self.options.search_type = Some(d);
                false
            }
            Msg::ScoreFilterInput(d) => {
                self.options.score_filter = Some(d);
                false
            }
            Msg::NumReturnedInput(d) => {
                self.options.num_returned = Some(d);
                false
            }
        }
    }

    fn view(&self) -> Html {
        let button_text = if self.searching {
            "Searching..."
        } else {
            "Search"
        };
        let more_text = if self.moreing { "Moreing..." } else { "More" };
        let error_str = self.last_error.clone().unwrap_or_default();
        html! {
            <>
            <div class="flex flex-col mx-auto max-w-3xl pb-1 mb-3">
                <div class="sm:flex">
                    <div class="sm:w-1/2">
                        <label class="block text-xs uppercase font-bold">{"Author"}</label>
                        <input oninput=self.link.callback(|e: InputData| Msg::AuthorInput(e.value)) class="text-gray-900 bg-gray-400 focus:bg-gray-100 w-full py-2 px-1" />
                    </div>
                    <div class="sm:w-1/2 sm:ml-1">
                        <label class="block text-xs uppercase font-bold">{"Subreddit"}</label>
                        <input oninput=self.link.callback(|e: InputData| Msg::SubredditInput(e.value)) class="text-gray-900 bg-gray-400 focus:bg-gray-100 w-full py-2 px-1" />
                    </div>
                </div>
                <div class="sm:flex">
                    <div class="sm:w-1/3">
                        <label class="block text-xs uppercase font-bold">{"Search for"}</label>
                        <div class="relative" id="select-type">
                            <Select<SearchType>
                                selected=self.options.search_type.clone()
                                options=SearchType::iter().collect::<Vec<_>>()
                                onchange=self.link.callback(Msg::SearchTypeInput)
                            />
                            <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700">
                                <svg class="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20"><path d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z"/></svg>
                            </div>
                        </div>
                    </div>
                    <div class="sm:w-1/3 sm:ml-1">
                        <label class="block text-xs uppercase font-bold">{"Num. Returned"}</label>
                        <input oninput=self.link.callback(|e: InputData| Msg::NumReturnedInput(e.value.parse().unwrap())) class="text-gray-900 bg-gray-300 focus:bg-gray-100 w-full py-2 px-1" type="number" min="25" step="25" value="100" />
                    </div>
                    <div class="sm:w-1/3 sm:ml-1">
                        <label class="block text-xs uppercase font-bold">{"Score Filter"}</label>
                        <input oninput=self.link.callback(|e: InputData| Msg::ScoreFilterInput(e.value)) class="text-gray-900 bg-gray-300 focus:bg-gray-100 w-full py-2 px-1" placeholder="e.g. >10 <100 >100,<900" />
                    </div>
                </div>
                <div>
                    <label class="block text-xs uppercase font-bold">{"Search Term"}</label>
                    <input oninput=self.link.callback(|e: InputData| Msg::SearchInput(e.value)) class="text-gray-900 bg-gray-400 focus:bg-gray-100 w-full py-2 px-1" />
                </div>
                <button type={"button"} onclick=self.link.callback(|_| Msg::DoSearch) class="bg-red-900 hover:bg-red-800 font-bold mt-4 py-2 px-4">{button_text}</button>
                <p class="text-red-200 text-center">{ error_str }</p>
            </div>
            {if self.entry_data.is_empty() { html! {
                <p class="text-gray-500 text-center">{ "No results" }</p>
            }} else { html! {
                <div class="flex flex-col px-auto max-w-5xl mx-auto">
                    { for self.entry_data.iter().map(|x| x.view()) }
                { if self.show_more_button { html! {
                <button type="button" onclick=self.link.callback(|_| Msg::DoMore) class="bg-red-900 hover:bg-red-800 font-bold py-2 mb-1">{more_text}</button>
                }} else { html! {}}}
                </div>
            }}}
            </>
        }
    }
}

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

fn url_encode(input: &str) -> String {
    utf8_percent_encode(input, FRAGMENT).to_string()
}
