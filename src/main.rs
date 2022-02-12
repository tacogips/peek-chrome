mod cmd;

mod db;
mod history;

use chrono::Utc;
use clap::Parser;
use cmd::*;
use db::*;
use std::io::{stdout, Error as IOError, Write};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CmdError {
    #[error("{0}")]
    DBError(#[from] DBError),

    #[error("{0}")]
    IOError(#[from] IOError),
}

pub type Result<T> = std::result::Result<T, CmdError>;

#[derive(Parser, Debug)]
#[clap(version = "0.1.0", author = "tacogips")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[clap(about = "Show histories")]
    History(History),
}

fn show_only_path<W>(dest: &mut W, path: &str, _: f64) -> Result<()>
where
    W: Write,
{
    writeln!(dest, "{path}")?;
    Ok(())
}

fn show_with_score<W>(dest: &mut W, path: &str, score: f64) -> Result<()>
where
    W: Write,
{
    writeln!(dest, "{score}    {path}")?;
    Ok(())
}

fn run() -> Result<()> {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::History(history) => {
            let db_path = match history.db_path {
                Some(db_path) => {
                    let mut pathbuf = PathBuf::new();
                    pathbuf.push(db_path);
                    pathbuf
                }
                None => default_paths::history()?,
            };

            let mut db = DB::new(db_path)?;
        }
    }
    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error:{:?}", err);
            1
        }
    })
}
