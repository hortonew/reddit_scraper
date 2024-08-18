# Reddit Scraper

Pet project to work on Rust project structure.

## Prerequisites

- Docker
- Rust/Cargo (Optional) - only if running outside of Docker

## Getting Started

This project is built in Rust, but uses docker compose to run the project (you shouldn't require Rust on your system).

```sh
# Build the dockerfiles, and run them with compose
make test
make run
```

Navigate to [http://localhost:8000](http://localhost:8000) to see the results from the database.
