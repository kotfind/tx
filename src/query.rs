use itertools::Itertools;
use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum LineProcessError {
    ColumnOutOfRangeError(#[from] ColumnOutOfRangeError),
}

#[derive(Debug, Error)]
#[error(
    "Cannot get column number {col_num} as there are only {col_num} columns. \
Line: \"{line}\""
)]
pub struct ColumnOutOfRangeError {
    line: String,
    col_num: usize,
    col_count: usize,
    // TODO: line_num
}

#[derive(Debug)]
pub struct Query {
    pub column_ids: Vec<usize>,
}

impl Query {
    pub fn process_line(&self, line: &str) -> Result<Vec<String>, LineProcessError> {
        let mut ans = Vec::new();

        // Two pointers algorithm
        let items = line.split_whitespace().collect_vec();

        for col_id in self.column_ids.iter() {
            match items.get(*col_id) {
                Some(s) => ans.push(s.to_string()),
                None => {
                    return Err(ColumnOutOfRangeError {
                        line: line.to_owned(),
                        col_num: col_id + 1,
                        col_count: items.len(),
                    }
                    .into());
                }
            }
        }

        Ok(ans)
    }
}
