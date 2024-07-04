use clap::Parser;
use std::io::{stdin, BufRead};

mod query;
mod query_parser;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum Error {
    QueryParseError(#[from] query_parser::ParseError),
    LineProcessError(#[from] query::LineProcessError),
    ReadError(#[from] std::io::Error),
}

#[derive(Parser)]
#[command(about, long_about = None)]
struct Cli {
    query_string: String,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let query = query_parser::parse(&cli.query_string)?;
    for line in stdin().lock().lines() {
        let line = line?;
        println!("{}", query.process_line(&line)?.join(" "));
    }
    Ok(())
}
