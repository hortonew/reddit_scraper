use reqwest::Client;
use rusqlite::{params, Connection, Result};
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

    // Open or create the SQLite database
    let conn = Connection::open("services/reddit_scraper.db")?;

    // Create a table for storing Reddit posts if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            selftext TEXT NOT NULL,
            created_utc REAL NOT NULL,
            url TEXT NOT NULL
        )",
        [],
    )?;

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
            // println!("Title: {}", post.title);
            // println!("Text: {}", post.selftext);
            // println!("URL: {}", post.url);
            // println!("---");

            // Insert the post into the SQLite database
            conn.execute(
                "INSERT INTO posts (title, selftext, created_utc, url) VALUES (?1, ?2, ?3, ?4)",
                params![post.title, post.selftext, post.created_utc, post.url],
            )?;

            println!("Inserted post into database");
        }
    }

    Ok(())
}
