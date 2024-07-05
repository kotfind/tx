use itertools::Itertools;

use crate::Cli;

#[derive(Debug)]
pub enum Splitter<LINES> {
    Whitespace {
        lines: LINES,
    },
    Smart {
        lines: Vec<String>,
        cur: usize,

        // NOTE: column_ranges.last[1] may be greather that lengths of some strings
        column_ranges: Vec<(usize, usize)>,
    },
}

impl<LINES: Iterator<Item = String>> Splitter<LINES> {
    pub fn from_cli_and_lines(cli: &Cli, lines: LINES) -> Self {
        if cli.ws_sep {
            Self::Whitespace { lines }
        } else {
            let lines = lines.collect_vec();
            let column_ranges = Self::count_column_ranges(&lines);
            Self::Smart {
                lines,
                cur: 0,
                column_ranges,
            }
        }
    }

    fn count_column_ranges(lines: &Vec<String>) -> Vec<(usize, usize)> {
        let is_column_whitespace = |col_id: usize| -> bool {
            for line in lines {
                if !line.chars().skip(col_id).next().unwrap().is_whitespace() {
                    return false;
                }
            }
            return true;
        };

        let mut ans = Vec::new();

        let min_len = lines.iter().fold(usize::MAX, |ans, s| ans.min(s.len()));
        let max_len = lines.iter().fold(usize::MIN, |ans, s| ans.max(s.len()));

        let mut col_start = None;

        for col_id in 0..min_len {
            if is_column_whitespace(col_id) {
                if let Some(start) = col_start {
                    ans.push((start, col_id));
                    col_start = None;
                }
            } else {
                if col_start.is_none() {
                    col_start = Some(col_id);
                }
            }
        }

        if let Some(start) = col_start {
            ans.push((start, max_len));
        }

        ans
    }
}

impl<LINES: Iterator<Item = String>> Iterator for Splitter<LINES> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Splitter::Whitespace { lines } => lines
                .next()
                .map(|line| line.split_whitespace().map(|s| s.to_string()).collect_vec()),
            Splitter::Smart {
                cur,
                lines,
                column_ranges,
            } => {
                if *cur >= lines.len() {
                    return None;
                }

                let ans = column_ranges
                    .iter()
                    .map(|(begin, end)| {
                        lines[*cur]
                            .chars()
                            .skip(*begin)
                            .take(*end - *begin)
                            .join("")
                            .trim()
                            .to_string()
                    })
                    .collect_vec();
                *cur += 1;
                Some(ans)
            }
        }
    }
}
