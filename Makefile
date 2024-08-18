# Define variables for the project name and default targets
CARGO := cargo
DOCKER := docker
PROJECT_NAME := reddit_scraper
VERSION := latest

# Default target: build all crates
all: build-scraper build-web

# Build all crates
build:
	$(CARGO) build

web:
	$(CARGO) run -p web

scrape:
	$(CARGO) run -p scraper

test:
	$(CARGO) test

# Build scraper service
build-scraper:
	# $(CARGO) build --release -p scraper
	$(DOCKER) build --build-arg APP_NAME=scraper -t $(PROJECT_NAME)_scraper:$(VERSION) .

# Build web service
build-web:
	# $(CARGO) build --release -p web
	$(DOCKER) build --build-arg APP_NAME=web -t $(PROJECT_NAME)_web:$(VERSION) .

run: build-scraper build-web
	docker compose up

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
.PHONY: all build build-web build-scraper \
        web scraper clean check fmt test
