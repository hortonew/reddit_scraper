use amiquip::{Connection, Exchange, Publish, Result};
use models::RedditResponse;
use reqwest::Client;
use serde_json;
use std::env;
use std::error::Error;
use std::io;
use tokio::time::{self, Duration};

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

    // Fetch posts from Reddit
    println!("Fetching posts from Reddit.");
    let response = fetch_reddit_posts(&client).await?;

    // Publish results to RabbitMQ
    publish_to_queue(&response).await?;

    Ok(())
}

async fn fetch_reddit_posts(client: &Client) -> Result<RedditResponse, Box<dyn Error>> {
    // Replace with your actual fetching logic
    let url = "https://www.reddit.com/r/kubernetes/new.json";

    let mut backoff = 1;
    loop {
        match client
            .get(url)
            .header("User-Agent", "rust:reddit-k8s:v0.1 (by /u/hortonew)")
            .send()
            .await
        {
            Ok(res) => match res.json::<RedditResponse>().await {
                Ok(parsed_response) => return Ok(parsed_response),
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
}

async fn publish_to_queue(response: &RedditResponse) -> Result<()> {
    // Connect to RabbitMQ with credentials
    let rabbitmq_url =
        env::var("RABBITMQ_URL").unwrap_or_else(|_| "localhost:5672/test-vhost".to_string());
    let mut connection = Connection::insecure_open(&rabbitmq_url)?;

    // Open a channel - None lets the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Get a handle to the direct exchange on our channel.
    let exchange = Exchange::direct(&channel);

    // Loop through the Reddit posts in the response
    for child in response.data.children.iter() {
        let post = &child.data;
        if !post.selftext.is_empty() && post.selftext.contains('?') {
            // Convert the post to a JSON string and handle potential errors
            let message = serde_json::to_string(post).map_err(|err| {
                amiquip::Error::IoErrorWritingSocket {
                    source: io::Error::new(io::ErrorKind::Other, err.to_string()),
                }
            })?;

            // Publish the post details to the "testqueue" queue
            exchange.publish(Publish::new(message.as_bytes(), "testqueue"))?;
        }
    }

    // Close the connection to RabbitMQ
    connection.close()
}
