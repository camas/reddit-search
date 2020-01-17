#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentsData {
    pub data: Vec<CommentData>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentData {
    pub author: String,
    pub body: String,
    #[serde(rename = "created_utc")]
    pub created_utc: i64,
    pub edited: bool,
    pub id: String,
    pub score: i64,
    pub subreddit: String,
    pub permalink: String,
}
