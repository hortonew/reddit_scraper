# Define variables for the project name and default targets
PROJECT_NAME := reddit_scraper
CARGO := cargo

# Default target: build all crates
all: build

# Build all crates
build:
	$(CARGO) build

web:
	$(CARGO) run -p web

scrape:
	$(CARGO) run -p scraper

# Clean build artifacts
clean:
	$(CARGO) clean

# Format the code according to Rust's style guidelines
fmt:
	$(CARGO) fmt

# Run tests for all crates
test:
	$(CARGO) test

# Default target when no arguments are provided
.PHONY: all build build-web build-scraper build-analyzer \
        run-web run-scraper run-analyzer clean check fmt test
