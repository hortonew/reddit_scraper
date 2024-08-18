use rusqlite::Connection;
use std::fs;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MigrationError {
    #[error("Rusqlite error")]
    RusqliteError(#[from] rusqlite::Error),

    #[error("IO error")]
    IoError(#[from] io::Error),
}

pub fn run_migrations(conn: &Connection) -> Result<(), MigrationError> {
    let migrations_path = if cfg!(target_os = "linux") && std::env::var("DOCKER_ENV").is_ok() {
        // Adjust the path for Docker environment
        PathBuf::from("/usr/src/app/databases/kubernetes_subreddit")
    } else {
        // Local environment path
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("kubernetes_subreddit")
    };

    println!("Migration path: {:?}", migrations_path);

    // Check if the migrations directory exists
    if !migrations_path.exists() {
        return Err(MigrationError::IoError(io::Error::new(
            io::ErrorKind::NotFound,
            "Migrations directory not found",
        )));
    }

    for entry in fs::read_dir(&migrations_path)? {
        let entry = entry?;
        let path = entry.path();

        println!("Processing file: {:?}", path);

        if path.extension().and_then(|s| s.to_str()) == Some("sql") {
            let sql = fs::read_to_string(&path)?;
            conn.execute_batch(&sql)?;
        }
    }

    Ok(())
}
