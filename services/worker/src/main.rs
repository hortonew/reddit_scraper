use amiquip::{
    Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result as AmiquipResult,
};
use rusqlite::{params, Connection as SqliteConnection};
use serde_json::Value;
use std::env;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let result = run();

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }

    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    // Connect to RabbitMQ with test-vhost
    let rabbitmq_url =
        env::var("RABBITMQ_URL").unwrap_or_else(|_| "localhost:5672/test-vhost".to_string());
    let mut mq_conn = Connection::insecure_open(&rabbitmq_url)?;
    let channel = mq_conn.open_channel(None)?;

    // Declare the queue if it doesn't already exist
    let queue = channel.queue_declare(
        "testqueue",
        QueueDeclareOptions {
            durable: true,
            ..QueueDeclareOptions::default()
        },
    )?;

    let consumer = queue.consume(ConsumerOptions::default())?;

    // Set up the SQLite database
    let db_path = std::env::var("DB_PATH").unwrap_or("services/reddit_scraper.db".to_string());
    println!("DB_PATH: {}", db_path);
    println!("CARGO_MANIFEST_DIR: {}", env!("CARGO_MANIFEST_DIR"));
    let conn = SqliteConnection::open(db_path)?;

    // Determine the migration path based on the environment
    let migrations_path = if cfg!(target_os = "linux") && std::env::var("DOCKER_ENV").is_ok() {
        // Docker environment path
        PathBuf::from("/usr/src/app/databases/kubernetes_subreddit")
    } else {
        // Local environment path
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../databases/kubernetes_subreddit")
    };

    // Run the migrations using the connection and the migration path
    databases::run_migrations(&conn, migrations_path)?;

    // Retrieve the last checkpoint
    let last_utc: f64 = conn
        .query_row(
            "SELECT last_utc FROM last_checkpoint ORDER BY id DESC LIMIT 1",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0.0);

    for message in consumer.receiver().iter() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let post: Value = serde_json::from_slice(&delivery.body)?;

                // Check if the post already exists in the database
                let mut stmt = conn.prepare("SELECT COUNT(*) FROM posts WHERE url = ?1")?;
                let count: i64 = stmt.query_row(params![post["url"].as_str()], |row| row.get(0))?;

                if count == 0 {
                    // Insert the post into the SQLite database
                    conn.execute(
                        "INSERT INTO posts (title, selftext, created_utc, url) VALUES (?1, ?2, ?3, ?4)",
                        params![
                            post["title"].as_str(),
                            post["selftext"].as_str(),
                            post["created_utc"].as_f64(),
                            post["url"].as_str()
                        ],
                    )?;
                    println!("Inserted post into database");
                } else {
                    println!("Post already exists in the database");
                }

                consumer.ack(delivery)?;
            }
            _ => (),
        }
    }

    // Close the connection to RabbitMQ
    mq_conn.close().map_err(|e| Box::new(e) as Box<dyn Error>)?;

    Ok(())
}
