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
        }
    }
    fn get_created(&self) -> i64 {
        self.created_utc
    }
}
