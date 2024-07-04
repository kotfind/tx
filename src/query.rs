use derivative::Derivative;
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
    /// List of column indexes. Should be sorted.
    pub column_ids: Vec<Column>,
}

impl Query {
    pub fn process_line(&self, line: &str) -> Result<String, LineProcessError> {
        let mut ans = Vec::new();

        // Two pointers algorithm
        let items = line.split_whitespace().collect_vec();
        let items_count = items.len();
        let mut items = items.into_iter().enumerate().peekable();
        let mut column_ids = self.column_ids.iter().peekable();

        while items.peek().is_some() && column_ids.peek().is_some() {
            let items_num = items.peek().unwrap().0 as usize;
            let columns_num = column_ids.peek().unwrap().0;

            if items_num < columns_num {
                items.next();
            } else if items_num > columns_num {
                column_ids.next();
            } else {
                ans.push(items.next().unwrap().1.to_owned());
                column_ids.next();
            }
        }

        if column_ids.peek().is_some() {
            return Err(ColumnOutOfRangeError {
                line: line.to_owned(),
                col_num: column_ids.next().unwrap().0,
                col_count: items_count,
            }
            .into());
        }

        Ok(ans.join(" "))
    }
}

#[derive(Derivative)]
#[derivative(Debug = "transparent")]
pub struct Column(pub usize);
