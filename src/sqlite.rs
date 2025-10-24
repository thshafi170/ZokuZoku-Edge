use anyhow::Result;
use rusqlite::{Connection, Result as SqliteResult};
use std::path::PathBuf;

pub struct SQLiteManager {
    connection: Connection,
    meta_path: Option<PathBuf>,
}

impl SQLiteManager {
    pub fn new(db_path: &PathBuf) -> Result<Self> {
        let connection = Connection::open(db_path)?;
        Ok(Self {
            connection,
            meta_path: None,
        })
    }

    pub fn set_meta_path(&mut self, path: PathBuf) {
        self.meta_path = Some(path);
    }

    pub fn query(&self, sql: &str) -> SqliteResult<Vec<Vec<String>>> {
        let mut stmt = self.connection.prepare(sql)?;
        let rows = stmt.query_map([], |row| {
            let mut values = Vec::new();
            for i in 0..row.as_ref().column_count() {
                values.push(row.get::<_, String>(i)?);
            }
            Ok(values)
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn query_mdb(&self, sql: &str) -> SqliteResult<Vec<Vec<String>>> {
        self.query(sql)
    }

    pub fn query_meta(&self, sql: &str) -> SqliteResult<Vec<Vec<String>>> {
        // For encrypted meta queries, we'll need to handle decryption separately
        // This is a placeholder
        self.query(sql)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_sqlite_manager() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        
        let manager = SQLiteManager::new(&db_path);
        assert!(manager.is_ok());
    }
}

