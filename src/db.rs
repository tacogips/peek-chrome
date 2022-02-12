use rusqlite::{Connection, Error as SqliteError};
use std::path::Path;
use thiserror::Error;

pub struct DB {
    pub conn: Connection,
}

#[derive(Error, Debug)]
pub enum DBError {
    #[error("{0}")]
    SqliteError(#[from] SqliteError),

    #[error("failed to getg $HOME_DIR")]
    HomeDirError,
}

pub type Result<T> = std::result::Result<T, DBError>;

pub mod default_paths {
    use super::{DBError, Result};

    use dirs::home_dir;
    use std::path::PathBuf;

    #[cfg(target_os = "linux")]
    fn chrome_dir() -> Result<PathBuf> {
        let mut dir = home_dir().ok_or(DBError::HomeDirError)?;
        dir.push(".config/BraveSoftware/Brave-Browser/Default");
        Ok(dir)
    }

    pub fn history() -> Result<PathBuf> {
        let mut dir = chrome_dir()?;
        dir.push("History");
        Ok(dir)
    }
}

impl DB {
    pub fn new<P>(dbpath: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let conn_str = format!("file:{}?immutable=1&mode=ro", dbpath.as_ref().display());
        let conn = Connection::open(conn_str)?;
        Ok(Self { conn })
    }
}
