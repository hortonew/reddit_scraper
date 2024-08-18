use models::RedditResponse;
use reqwest::Client;
use rusqlite::{params, Connection, Result};
use std::error::Error;
use tokio::time;
use tokio::time::Duration;
// use databases;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let result = run().await;

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }

    Ok(())
}

async fn run() -> Result<(), Box<dyn Error>> {
    let db_path = std::env::var("DB_PATH").unwrap_or("services/reddit_scraper.db".to_string());
    println!("DB_PATH: {}", db_path);
    let client = Client::new();
    let conn = Connection::open(db_path)?;

    // Create the posts table with a UNIQUE constraint on url
    println!("Creating table posts if it doesn't exist.");
    println!("Creating table last_checkpoint if it doesn't exist.");
    databases::run_migrations(&conn)?;

    // Retrieve the last checkpoint
    let last_utc: f64 = conn
        .query_row(
            "SELECT last_utc FROM last_checkpoint ORDER BY id DESC LIMIT 1",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0.0);

    // Construct the API request URL
    println!("Fetching posts from Reddit.");
    let url = format!(
        "https://www.reddit.com/r/kubernetes/new.json?before={}",
        last_utc
    );
    let mut response = None;
    let mut backoff = 1;

    loop {
        match client
            .get(&url)
            .header("User-Agent", "rust:reddit-k8s:v0.1 (by /u/hortonew)")
            .send()
            .await
        {
            Ok(res) => match res.json::<RedditResponse>().await {
                // Await the parsing of the JSON response
                Ok(parsed_response) => {
                    response = Some(parsed_response);
                    break;
                }
                Err(err) => {
                    eprintln!("Failed to parse JSON: {}", err);
                    time::sleep(Duration::from_secs(backoff)).await;
                    backoff *= 2;
                }
            },
            Err(err) => {
                eprintln!("Request failed: {}.  Backoff={}", err, backoff);
                time::sleep(Duration::from_secs(backoff)).await;
                backoff *= 2;
            }
        }
    }

    let response = response.ok_or("Request failed after multiple attempts")?;

    // Process the posts
    println!("Processing posts.");
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
                println!("Post already exists in the database");
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
