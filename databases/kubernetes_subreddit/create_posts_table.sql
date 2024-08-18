CREATE TABLE IF NOT EXISTS posts (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    selftext TEXT NOT NULL,
    created_utc REAL NOT NULL,
    url TEXT NOT NULL UNIQUE
);
