use clap::Parser;

#[derive(Parser, Debug)]
pub struct History {
    #[clap(short, long)]
    pub db_path: Option<String>,
}
