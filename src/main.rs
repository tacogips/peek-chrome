mod cmd;

mod db;
mod history;

use clap::Parser;
use cmd::*;
use db::*;
use history::*;
use std::io::{stdout, Error as IOError, Write};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CmdError {
    #[error("{0}")]
    DBError(#[from] DBError),

    #[error("{0}")]
    HistoryError(#[from] HistoryError),

    #[error("{0}")]
    IOError(#[from] IOError),
}

pub type Result<T> = std::result::Result<T, CmdError>;

#[derive(Parser, Debug)]
#[clap(version = "0.1.0", author = "tacogips")]
struct Opts {
    #[clap(short, long)]
    pub delimiter: Option<String>,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[clap(about = "Show histories")]
    History(HistoryArg),
}

fn string_to_path(s: String) -> PathBuf {
    let mut pathbuf = PathBuf::new();
    pathbuf.push(s);
    pathbuf
}

fn run() -> Result<()> {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::History(history) => {
            let db_path = match history.db_path {
                Some(db_path) => string_to_path(db_path),
                None => default_paths::history()?,
            };

            let db = DB::new(db_path)?;
            let histories = fetch_history(&db, history.limit)?;

            let stdout = stdout();
            let mut stdout_lock = stdout.lock();
            let delimiter = opts.delimiter.unwrap_or_else(|| "\t".to_string());
            for each in histories {
                writeln!(
                    stdout_lock,
                    "{url}{delimiter}{title}{delimiter}{last_visit_time}{delimiter}{visit_count}",
                    url = each.url,
                    title = each.title,
                    last_visit_time = each.last_visit_time,
                    visit_count = each.visit_count,
                    delimiter = delimiter
                )?;
            }
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
