use itertools::Itertools;

use crate::Cli;

#[derive(Debug)]
pub enum Splitter {
    Whitespace,
    String(String),
}

impl Splitter {
    pub fn from_cli(cli: &Cli) -> Self {
        if let Some(sep) = &cli.sep {
            Self::String(sep.to_string())
        } else {
            Self::Whitespace
        }
    }

    pub fn split(&self, input: &str) -> Vec<String> {
        match self {
            Splitter::Whitespace => input
                .split_whitespace()
                .map(|s| s.to_string())
                .collect_vec(),
            Splitter::String(sep) => input.split(sep).map(|s| s.to_string()).collect_vec(),
        }
    }
}
