use models::RedditResponse;
use reqwest::Client;
use rusqlite::{params, Connection, Result};
use std::error::Error;

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
    let conn = Connection::open("services/reddit_scraper.db")?;

    // Create the posts table with a UNIQUE constraint on url
    conn.execute(
        "CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            selftext TEXT NOT NULL,
            created_utc REAL NOT NULL,
            url TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    // Create the checkpoint table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS last_checkpoint (
            id INTEGER PRIMARY KEY,
            last_utc REAL NOT NULL
        )",
        [],
    )?;

    // Retrieve the last checkpoint
    let last_utc: f64 = conn
        .query_row(
            "SELECT last_utc FROM last_checkpoint ORDER BY id DESC LIMIT 1",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0.0);

    // Construct the API request URL
    let url = format!(
        "https://www.reddit.com/r/kubernetes/new.json?before={}",
        last_utc
    );
    let response = client
        .get(&url)
        .header("User-Agent", "rust:reddit-k8s:v0.1 (by /u/hortonew)")
        .send()
        .await?
        .json::<RedditResponse>()
        .await?;

    // Process the posts
    for child in response.data.children.iter() {
        let post = &child.data;
        if !post.selftext.is_empty() && post.selftext.contains('?') {
            // Check if the post already exists
            let mut stmt = conn.prepare("SELECT COUNT(*) FROM posts WHERE url = ?1")?;
            let count: i64 = stmt.query_row(params![post.url], |row| row.get(0))?;

            if count == 0 {
                // Insert the post into the SQLite database
                conn.execute(
                    "INSERT INTO posts (title, selftext, created_utc, url) VALUES (?1, ?2, ?3, ?4)",
                    params![post.title, post.selftext, post.created_utc, post.url],
                )?;
                println!("Inserted post into database");
            } else {
                // println!("Post already exists in the database");
            }
        }
    }

    // Update the checkpoint with the latest timestamp
    let max_utc = response
        .data
        .children
        .iter()
        .map(|child| child.data.created_utc)
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or(last_utc);

    conn.execute(
        "INSERT INTO last_checkpoint (last_utc) VALUES (?1)",
        params![max_utc],
    )?;

    Ok(())
}
