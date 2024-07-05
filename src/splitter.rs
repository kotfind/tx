use itertools::Itertools;

use crate::Cli;

#[derive(Debug)]
pub struct Splitter<LINES> {
    lines: LINES,
}

impl<LINES> Splitter<LINES> {
    pub fn from_cli_and_lines(_cli: &Cli, lines: LINES) -> Self {
        Self { lines }
    }
}

impl<LINES: Iterator<Item = String>> Iterator for Splitter<LINES> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines
            .next()
            .map(|line| line.split_whitespace().map(|s| s.to_string()).collect_vec())
    }
}
