# Reddit Scraper

The purpose of the reddit scraper is to gather reddit posts into a database and to produce sentiments (positive, negative, neutral) on the post content.  The use of this is to keep a pulse on the kubernetes community, seeing if the sentiment of posts are good or bad and which way they are trending.

Target audience: Devops and SRE type roles.

The reddit scraper is separated into multiple components:

- database (sqlite)
- queue (rabbitmq)
- scraper (rust)
- worker (rust)
- analyzer (rust + sentiment analysis)
- REST API (with metrics/healthcheck endpoint)
- Web frontend (ReactJS)
- prometheus (monitoring)
- github actions for CI/CD

A user interacts with the web frontend, retrieving information and sentiment related to a reddit post.  This is gathered by querying the analyze function, which makes use of the REST API.

The scraper works in the background to pull in Reddit posts and store them in the queue.

The worker grabs messages from the queue and stores posts into the database for "analysis" by the analyze and sentiment function.

I chose Rust for the backend for its speed and memory safety.  I chose React for the frontend for it's penetration in the market as a great framework to use.  I chose rabbitmq for the queue as it's robust and production ready.
