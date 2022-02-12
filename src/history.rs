use super::db::*;
use rusqlite::{Connection, Error as SqliteError, Result as SqliteResult};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FrecencyError {
    #[error("{0}")]
    SqliteError(#[from] SqliteError),

    #[error("invalid max visit num")]
    InvalidMaxVisitNum,
}

pub type Result<T> = std::result::Result<T, FrecencyError>;
#[derive(Debug)]
pub struct History {}

pub fn fetch_history(db: &DB, limit: Option<usize>) -> Result<Vec<(String, f64)>> {
    let mut fetch_query = " SELECT url,title,last_visit_time,visit_count
            FROM urls
            ORDER BY last_visit_time DESC "
        .to_string();

    if let Some(limit) = limit {
        fetch_query = format!("{fetch_query} {limit}", limit = format!(" LIMIT {limit} "));
    }

    let mut stmt = db.conn.prepare_cached(&fetch_query)?;

    let records = stmt.query_and_then([], |row| -> SqliteResult<(String, f64)> {
        //TODO(tacogips) for debugging

        let score: f64 = row.get(0)?;
        let path: String = row.get(1)?;
        Ok((path, score))
    })?;
    let mut scores = Vec::new();
    for each in records {
        let (path, score) = each?;
        scores.push((path, score));
    }
    Ok(scores)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_db() {
        let db = DB::new(default_paths::history().unwrap()).unwrap();
        let histories = fetch_history(&db, None).unwrap();
    }
}
