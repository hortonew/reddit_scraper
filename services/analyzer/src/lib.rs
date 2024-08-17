use rusqlite::{Connection, Result};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Post {
    title: String,
    selftext: String,
    created_utc: f64,
    url: String,
}

pub fn get_analysis() -> Result<Vec<Post>> {
    let conn = Connection::open("services/reddit_scraper.db")?;

    let mut stmt = conn.prepare("SELECT title, selftext, created_utc, url FROM posts")?;
    let post_iter = stmt.query_map([], |row| {
        Ok(Post {
            title: row.get(0)?,
            selftext: row.get(1)?,
            created_utc: row.get(2)?,
            url: row.get(3)?,
        })
    })?;

    let mut posts = Vec::new();
    for post in post_iter {
        posts.push(post?);
    }

    Ok(posts)
}
