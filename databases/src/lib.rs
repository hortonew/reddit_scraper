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
pub fn run_migrations(conn: &Connection, migrations_path: PathBuf) -> Result<(), MigrationError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::OptionalExtension;
    use rusqlite::Result;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    fn setup_db() -> Connection {
        Connection::open_in_memory().expect("Failed to create in-memory database")
    }

    #[test]
    fn test_run_migrations_success() -> Result<()> {
        let conn = setup_db();
        let dir = tempdir().unwrap();
        let migrations_path = dir.path().join("kubernetes_subreddit");
        fs::create_dir(&migrations_path).unwrap();

        // Create a sample SQL migration file
        let sql_file_path = migrations_path.join("001_create_table.sql");
        let mut file = File::create(&sql_file_path).unwrap();
        writeln!(file, "CREATE TABLE test (id INTEGER PRIMARY KEY);").unwrap();

        println!("Testing run_migrations_success...");
        let result = run_migrations(&conn, migrations_path.clone()); // Pass the migrations_path
        assert!(result.is_ok(), "run_migrations failed: {:?}", result);

        // Check if the table was created
        let mut stmt =
            conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='test';")?;
        let table_exists: Option<String> = stmt.query_row([], |row| row.get(0)).optional()?;
        assert_eq!(table_exists, Some("test".to_string()));

        Ok(())
    }

    #[test]
    fn test_run_migrations_missing_directory() {
        let conn = setup_db();
        let dir = tempdir().unwrap();
        let migrations_path = dir.path().join("non_existent_directory");

        println!("Testing run_migrations_missing_directory...");
        let result = run_migrations(&conn, migrations_path); // Pass the migrations_path
        assert!(
            matches!(result, Err(MigrationError::IoError(_))),
            "Expected IoError, got: {:?}",
            result
        );
    }

    #[test]
    fn test_run_migrations_invalid_sql() {
        let conn = setup_db();
        let dir = tempdir().unwrap();
        let migrations_path = dir.path().join("kubernetes_subreddit");
        fs::create_dir(&migrations_path).unwrap();

        // Create a sample SQL migration file with invalid SQL
        let sql_file_path = migrations_path.join("001_invalid.sql");
        let mut file = File::create(&sql_file_path).unwrap();
        writeln!(file, "INVALID SQL STATEMENT;").unwrap();

        println!("Testing run_migrations_invalid_sql...");
        let result = run_migrations(&conn, migrations_path.clone()); // Pass the migrations_path
        assert!(
            matches!(result, Err(MigrationError::RusqliteError(_))),
            "Expected RusqliteError, got: {:?}",
            result
        );
    }
}
