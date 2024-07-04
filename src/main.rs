use clap::Parser;
use parser::QueryParserAns;
use printer::{Printer, PrinterStyle};
use splitter::Splitter;
use std::io::{stdin, BufRead};

mod parser;
mod printer;
mod query;
mod splitter;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum Error {
    QueryParseError(#[from] parser::ParseError),
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

    /// Print a header.
    #[arg(long)]
    print_header: bool,
}

fn real_main() -> Result<(), Error> {
    let cli = Cli::parse();

    let splitter = Splitter::from_cli(&cli);
    let mut printer = Printer::new(PrinterStyle::from_cli(&cli));

    let mut lines = stdin().lock().lines();
    let first_row = match lines.next() {
        Some(l) => l?,
        None => {
            eprintln!("warning: empty input");
            return Ok(());
        }
    };
    let first_row = splitter.split(&first_row);

    let QueryParserAns {
        query,
        header_required: _header_required,
    } = parser::parse(&cli.query_string, &first_row)?;

    printer.push_header(query.process_line(&first_row)?);
    for line in lines {
        let line = line?;
        printer.push_line(query.process_line(&splitter.split(&line))?);
    }

    printer.finish();

    Ok(())
}

fn main() {
    // Print error as Display, rather than as Debug
    if let Err(e) = real_main() {
        println!("{}", e);
    }
}
