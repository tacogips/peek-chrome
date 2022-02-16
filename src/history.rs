use super::db::*;
use rusqlite::{Error as SqliteError, Result as SqliteResult};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HistoryError {
    #[error("{0}")]
    SqliteError(#[from] SqliteError),
}

pub type Result<T> = std::result::Result<T, HistoryError>;

#[derive(Debug)]
pub struct History {
    pub url: String,
    pub title: String,
    pub last_visit_time: u64,
    pub visit_count: u64,
}

pub fn fetch_history(db: &DB, limit: Option<usize>) -> Result<Vec<History>> {
    let mut fetch_query = " SELECT url,title,last_visit_time,visit_count
            FROM urls
            ORDER BY last_visit_time DESC "
        .to_string();

    if let Some(limit) = limit {
        fetch_query = format!(
            "{fetch_query} {limit}",
            limit = format_args!(" LIMIT {limit} ")
        );
    }

    let mut stmt = db.conn.prepare_cached(&fetch_query)?;

    let records = stmt.query_and_then([], |row| -> SqliteResult<History> {
        let url: String = row.get(0)?;
        let title: String = row.get(1)?;
        let last_visit_time = row.get(2)?;
        let visit_count: u64 = row.get(3)?;

        Ok(History {
            url,
            title,
            last_visit_time,
            visit_count,
        })
    })?;
    let mut scores = Vec::new();
    for each in records {
        scores.push(each?);
    }
    Ok(scores)
}
