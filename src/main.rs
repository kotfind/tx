use clap::Parser;
use printer::{Printer, PrinterStyle};
use splitter::Splitter;
use std::io::{stdin, BufRead};

mod printer;
mod query;
mod query_parser;
mod splitter;

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

    /// Don't format an output as a pretty table. May be more effective.
    #[arg(long)]
    no_pretty: bool,

    /// Sepparate each row by string (char) pattern
    #[arg(long)]
    sep: Option<String>,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let splitter = Splitter::from_cli(&cli);
    let query = query_parser::parse(&cli.query_string)?;
    let mut printer = Printer::new(PrinterStyle::from_cli(&cli));

    for line in stdin().lock().lines() {
        let line = line?;
        printer.push_line(query.process_line(&splitter.split(&line))?);
    }

    printer.finish();

    Ok(())
}
