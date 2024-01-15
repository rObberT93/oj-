pub use clap::Parser;
use serde::Serialize;

#[derive(Parser, Debug, Serialize)]
pub struct Args {
    #[clap(short, long, value_parser)]
    pub config: String,
    #[clap(short, long = "flush-data")]
    pub flush_data: bool,
}