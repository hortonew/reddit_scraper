use models::RedditPost;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rusqlite::{Connection, Result};
use sentiment::analyze;

pub fn get_analysis() -> Result<Option<(RedditPost, f32)>> {
    let conn = Connection::open(
        std::env::var("DB_PATH").unwrap_or("services/reddit_scraper.db".to_string()),
    )?;

    let mut stmt = conn.prepare("SELECT title, selftext, created_utc, url FROM posts")?;
    let post_iter = stmt.query_map([], |row| {
        Ok(RedditPost {
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

    // Select a random post from the list
    let random_post = posts.as_slice().choose(&mut thread_rng()).cloned();

    if let Some(post) = random_post {
        // Perform sentiment analysis on the selftext of the selected post
        let sentiment_result = analyze(post.selftext.clone());
        let sentiment_score = sentiment_result.score;
        return Ok(Some((post, sentiment_score)));
    }

    Ok(None)
}

pub fn sentiment_label(score: f32) -> &'static str {
    if score > 0.0 {
        "positive"
    } else if score < 0.0 {
        "negative"
    } else {
        "neutral"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::{params, Connection};
    use tempfile::TempDir;

    fn setup_db_with_posts() -> (Connection, TempDir) {
        let dir = TempDir::new().unwrap(); // Create a TempDir that lives as long as we need it
        let db_path = dir.path().join("test_reddit_scraper.db");
        let conn = Connection::open(&db_path).expect("Failed to create test database");

        conn.execute(
            "CREATE TABLE posts (title TEXT, selftext TEXT, created_utc REAL, url TEXT)",
            [],
        )
        .expect("Failed to create posts table");

        conn.execute(
            "INSERT INTO posts (title, selftext, created_utc, url) VALUES (?1, ?2, ?3, ?4)",
            params![
                "Test Post 1",
                "This is a positive post.",
                1.0,
                "http://example.com/1"
            ],
        )
        .expect("Failed to insert post");

        conn.execute(
            "INSERT INTO posts (title, selftext, created_utc, url) VALUES (?1, ?2, ?3, ?4)",
            params![
                "Test Post 2",
                "This is a negative post.",
                2.0,
                "http://example.com/2"
            ],
        )
        .expect("Failed to insert post");

        (conn, dir) // Return the connection and the TempDir
    }

    #[test]
    fn test_get_analysis_with_posts() {
        let (_conn, dir) = setup_db_with_posts();
        std::env::set_var(
            "DB_PATH",
            dir.path().join("test_reddit_scraper.db").to_str().unwrap(),
        );

        let analysis = get_analysis().expect("Failed to get analysis");
        assert!(analysis.is_some(), "Expected a post and sentiment score");

        if let Some((post, score)) = analysis {
            assert!(score != 0.0, "Sentiment score should not be neutral");
            assert!(!post.title.is_empty(), "Post title should not be empty");
            assert!(
                !post.selftext.is_empty(),
                "Post selftext should not be empty"
            );
        }
    }

    #[test]
    fn test_get_analysis_no_posts() {
        let dir = TempDir::new().unwrap();
        let db_path = dir.path().join("test_reddit_scraper.db");
        let conn = Connection::open(&db_path).expect("Failed to create test database");

        conn.execute(
            "CREATE TABLE posts (title TEXT, selftext TEXT, created_utc REAL, url TEXT)",
            [],
        )
        .expect("Failed to create posts table");

        std::env::set_var("DB_PATH", db_path.to_str().unwrap());

        let analysis = get_analysis().expect("Failed to get analysis");
        assert!(
            analysis.is_none(),
            "Expected no post when database is empty"
        );
    }

    #[test]
    fn test_sentiment_label() {
        assert_eq!(
            sentiment_label(1.0),
            "positive",
            "Expected 'positive' label for positive score"
        );
        assert_eq!(
            sentiment_label(-1.0),
            "negative",
            "Expected 'negative' label for negative score"
        );
        assert_eq!(
            sentiment_label(0.0),
            "neutral",
            "Expected 'neutral' label for neutral score"
        );
    }
}
