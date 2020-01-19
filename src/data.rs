use yew::{html, Html};

pub trait Entry: std::fmt::Debug {
    fn view(&self) -> Html;
    fn get_created(&self) -> i64;
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentsData {
    pub data: Vec<CommentData>,
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentData {
    pub author: String,
    pub body: String,
    #[serde(rename = "created_utc")]
    pub created_utc: i64,
    pub id: String,
    pub score: i64,
    pub subreddit: String,
    pub permalink: Option<String>,
}

impl Entry for CommentData {
    fn view(&self) -> Html {
        let naive = chrono::NaiveDateTime::from_timestamp(self.created_utc, 0);
        let created = chrono::DateTime::<chrono::Utc>::from_utc(naive, chrono::Utc);
        let time_string = created.format("%Y-%m-%d %H:%M:%S").to_string();
        let permalink = self.permalink.clone().unwrap_or_default();
        html! {
            <div class="bg-gray-900 px-1 mb-1">
                <div class="flex">
                    <a href={format!("https://reddit.com/r/{}", self.subreddit)}>
                        <div class="text-sm text-red-500">{format!("/r/{}", self.subreddit)}</div>
                    </a>
                    <a href={format!("https://reddit.com/u/{}", self.author)}>
                        <div class="text-sm text-red-500 ml-2">{format!("/u/{}", self.author)}</div>
                    </a>
                    <div class="text-sm text-red-500 ml-auto">{time_string}</div>
                </div>
                <a href={format!("https://reddit.com{}", permalink)}>
                    <div class="whitespace-pre-wrap">{&self.body}</div>
                </a>
            </div>
        }
    }

    fn get_created(&self) -> i64 {
        self.created_utc
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostsData {
    pub data: Vec<PostData>,
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostData {
    pub author: String,
    pub title: String,
    pub url: String,
    pub selftext: String,
    #[serde(rename = "created_utc")]
    pub created_utc: i64,
    pub id: String,
    pub score: i64,
    pub subreddit: String,
    pub permalink: Option<String>,
    pub thumbnail: Option<String>,
}

impl PostData {
    fn view_thumbnail(&self) -> Html {
        if let Some(thumbnail) = &self.thumbnail {
            if thumbnail.starts_with("http") {
                return html! {
                    <div class="mr-1 mb-1 w-32 h-32 overflow-hidden flex-shrink-0">
                        <img class="w-full h-full object-cover" src={thumbnail} />
                    </div>
                };
            }
        }
        let extension = extension(&self.url);
        if extension == ".png" || extension == ".jpg" {
            return html! {
                <div class="mr-1 mb-1 w-32 h-32 overflow-hidden flex-shrink-0">
                    <img class="w-full h-full object-cover" src={&self.url} />
                </div>
            };
        }
        html! {}
    }
}

impl Entry for PostData {
    fn view(&self) -> Html {
        let naive = chrono::NaiveDateTime::from_timestamp(self.created_utc, 0);
        let created = chrono::DateTime::<chrono::Utc>::from_utc(naive, chrono::Utc);
        let time_string = created.format("%Y-%m-%d %H:%M:%S").to_string();
        let permalink = self.permalink.clone().unwrap_or_default();
        html! {
            <div class="bg-gray-900 px-1 mb-1">
                <div class="flex">
                    <a href={format!("https://reddit.com/r/{}", self.subreddit)}>
                        <div class="text-sm text-red-500">{format!("/r/{}", self.subreddit)}</div>
                    </a>
                    <a href={format!("https://reddit.com/u/{}", self.author)}>
                        <div class="text-sm text-red-500 ml-2">{format!("/u/{}", self.author)}</div>
                    </a>
                    <div class="text-sm text-red-500 ml-auto">{time_string}</div>
                </div>
                <div class="flex">
                    { self.view_thumbnail() }
                    <div>
                        <a href={format!("https://reddit.com{}", permalink)}>
                            <div class="font-bold">{&self.title}</div>
                        </a>
                        { if self.selftext.len() > 0 {
                            html! {
                                <div class="whitespace-pre-wrap">{ &self.selftext }</div>
                            }
                        } else {
                            html! {
                                <a href={ format!("{}", &self.url) }>
                                    <div>{ &self.url }</div>
                                </a>
                            }
                        }}
                    </div>
                </div>
            </div>
        }
    }

    fn get_created(&self) -> i64 {
        self.created_utc
    }
}

fn extension(filename: &str) -> &str {
    filename
        .rfind('.')
        .map(|idx| &filename[idx..])
        .filter(|ext| ext.chars().skip(1).all(|c| c.is_ascii_alphanumeric()))
        .unwrap_or("")
}
