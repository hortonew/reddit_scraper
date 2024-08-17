use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct RedditPost {
    title: String,
    selftext: String,
    created_utc: f64,
    url: String,
}

#[derive(Debug, Deserialize)]
struct RedditResponse {
    data: RedditData,
}

#[derive(Debug, Deserialize)]
struct RedditData {
    children: Vec<RedditChild>,
}

#[derive(Debug, Deserialize)]
struct RedditChild {
    data: RedditPost,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let result = run().await;

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }

    Ok(())
}

async fn run() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = "https://www.reddit.com/r/kubernetes/new.json";

    let response = client
        .get(url)
        .header("User-Agent", "rust:reddit-k8s:v0.1 (by /u/hortonew)")
        .send()
        .await?
        .json::<RedditResponse>()
        .await?;

    for child in response.data.children.iter() {
        let post = &child.data;
        if !post.selftext.is_empty() && post.selftext.contains('?') {
            println!("Title: {}", post.title);
            println!("Text: {}", post.selftext);
            println!("URL: {}", post.url);
            println!("---");
        }
    }

    Ok(())
}
