use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedditPost {
    pub title: String,
    pub selftext: String,
    pub created_utc: f64,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct RedditResponse {
    pub data: RedditData,
}

#[derive(Debug, Deserialize)]
pub struct RedditData {
    pub children: Vec<RedditChild>,
}

#[derive(Debug, Deserialize)]
pub struct RedditChild {
    pub data: RedditPost,
}
