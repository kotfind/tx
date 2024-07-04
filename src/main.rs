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

    /// Pretty print output as a table
    #[arg(short, long)]
    pretty: bool,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let query = query_parser::parse(&cli.query_string)?;
    let table_style = TableStyle::from_cli(&cli);
    let mut table = Table::new();

    for line in stdin().lock().lines() {
        let line = line?;
        table.push_line(query.process_line(&line)?)
    }

    table.print(&table_style);

    Ok(())
}
