use clap::Parser;

#[derive(Parser, Debug)]
pub struct HistoryArg {
    #[clap(short, long)]
    pub db_path: Option<String>,

    #[clap(short, long)]
    pub limit: Option<usize>,
}
