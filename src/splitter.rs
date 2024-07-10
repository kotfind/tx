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

    // Returns a vector of pairs `column_ranges`.
    // `column_ranges[i]` is a pair of the start index (inclusive) and end index (exclusize)
    // of i-th column.
    // NOTE: column_ranges.last[1] may be greather that lengths of some strings.
    fn count_column_ranges(lines: &Vec<String>) -> Vec<(usize, usize)> {
        let max_len = lines.iter().fold(usize::MIN, |ans, s| ans.max(s.len()));

        let is_column_whitespace = Self::get_is_column_whitespace_vec(lines);
        let whitespace_prefix = Self::get_whitespace_prefix_vec(&is_column_whitespace);
        let separator_width = (*whitespace_prefix.iter().max().unwrap()).max(1);

        let mut ans = Vec::new();

        let mut col_start = None;
        for col_id in 0..max_len {
            if whitespace_prefix[col_id] == separator_width {
                if let Some(col_start) = col_start {
                    ans.push((col_start, col_id - separator_width + 1));
                }
                col_start = None;
            } else if !is_column_whitespace[col_id] && col_start.is_none() {
                col_start = Some(col_id);
            }
        }
        if let Some(col_start) = col_start {
            ans.push((col_start, max_len));
        }

        ans
    }

    /// Returns Vec<uszie> of the same length as the `is_column_whitespace` vec.
    /// `whitespace_prefix[col_id]` is a number of consequential whitespace columns before column
    /// `col_id`, including current.
    fn get_whitespace_prefix_vec(is_column_whitespace: &Vec<bool>) -> Vec<usize> {
        let len = is_column_whitespace.len();
        if len == 0 {
            return vec![];
        }

        let mut whitespace_prefix = vec![0; len];
        whitespace_prefix[0] = is_column_whitespace[0] as usize;

        for col_id in 1..len {
            if is_column_whitespace[col_id] {
                whitespace_prefix[col_id] = whitespace_prefix[col_id - 1] + 1;
            }
        }

        whitespace_prefix
    }

    /// Returns Vec<bool> of the same length as the longest string.
    /// `is_column_whitespace[col_id] = true` means that `col_id` column contains only whitespace
    /// characters.
    fn get_is_column_whitespace_vec(lines: &Vec<String>) -> Vec<bool> {
        let max_len = lines.iter().fold(usize::MIN, |ans, s| ans.max(s.len()));

        let mut is_column_whitespace = vec![true; max_len];

        for line in lines {
            for (col_id, c) in line.chars().enumerate() {
                if !c.is_whitespace() {
                    is_column_whitespace[col_id] = false;
                }
            }
        }

        is_column_whitespace
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
