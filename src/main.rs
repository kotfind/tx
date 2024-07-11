use clap::Parser;
use parser::QueryParseAns;
use printer::Printer;
use splitter::Splitter;
use std::{
    error::Error,
    io::{stdin, BufRead},
};

mod condition;
mod parser;
mod printer;
mod query;
mod splitter;

#[derive(Debug, thiserror::Error)]
pub enum MainError {
    #[error("couldn't parse query string")]
    QueryParseError(#[from] parser::ParseError),

    #[error("couldn't process a line")]
    LineProcessError(#[from] query::LineProcessError),

    #[error("couldn't read a line")]
    ReadError(#[from] std::io::Error),
}

#[derive(Parser)]
#[command(about, long_about = None, disable_help_flag = true, disable_help_subcommand = true)]
struct Cli {
    /// Query string
    // TODO: describe format
    query_string: String,

    // Make help flag long-only
    #[arg(long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,

    /// Don't format an output as a pretty table. May be more effective.
    #[arg(long)]
    no_pretty: bool,

    /// Print a header.
    #[arg(long, short = 'h')]
    print_header: bool,

    /// Threats the first line of an input as a header. Default behaviour if named columns are used
    /// in a query string.
    #[arg(long, short = 'H')]
    has_header: bool,

    /// Sepparate each row by whitespace symbols. May be more effective than default smart separation.
    #[arg(long)]
    ws_sep: bool,
    // // TODO: enable this
    // /// Don't trim strings
    // #[arg(long, short = 't')]
    // no_trim: bool,
}

fn real_main() -> Result<(), MainError> {
    let cli = Cli::parse();

    let lines = stdin().lock().lines().map(|l| l.unwrap() /* FIXME: */);
    let mut splitter = Splitter::from_cli_and_lines(&cli, lines).peekable();

    let first_row = match splitter.peek() {
        Some(l) => l,
        None => {
            eprintln!("warning: empty input");
            return Ok(());
        }
    };

    let QueryParseAns {
        query,
        is_header_required,
    } = parser::parse(&cli.query_string, &first_row)?;

    let has_header = cli.has_header || cli.print_header || is_header_required;
    let print_header = cli.print_header;

    let mut printer = Printer::new(&cli, has_header, print_header);

    if has_header {
        printer.push_header(query.process_line_no_check(&splitter.next().unwrap())?);
    }
    for row in splitter {
        if let Some(row) = query.process_line(&row)? {
            printer.push_row(row);
        }
    }

    printer.finish();

    Ok(())
}

// Print error as Display, rather than as Debug
fn print_error(e: &dyn Error) {
    print!("{}", e);
    if let Some(next_e) = e.source() {
        print!(": ");
        print_error(next_e);
    }
}

fn main() {
    if let Err(e) = real_main() {
        print_error(&e);
    }
}
