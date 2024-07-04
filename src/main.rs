use clap::Parser;
use std::io::{stdin, BufRead};
use table::{Table, TableStyle};

mod query;
mod query_parser;
mod table;

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
    /// Query string
    // TODO: describe format
    query_string: String,

    /// Don't format output as pretty table. May be more effective.
    #[arg(long)]
    no_pretty: bool,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let query = query_parser::parse(&cli.query_string)?;
    let mut table = Table::new(TableStyle::from_cli(&cli));

    for line in stdin().lock().lines() {
        let line = line?;
        table.push_line(query.process_line(&line)?)
    }

    table.finish();

    Ok(())
}
