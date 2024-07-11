use crate::condition::ConditionExpr;

#[derive(Debug, thiserror::Error)]
pub enum LineProcessError {
    #[error("column out of range error")]
    ColumnOutOfRangeError(#[from] ColumnOutOfRangeError),
}

#[derive(Debug, thiserror::Error)]
#[error(
    "cannot get column number {col_num} as there are only {col_num} columns. \
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
    pub cond_expr: ConditionExpr,
}

impl Query {
    pub fn process_line(&self, row: &Vec<String>) -> Result<Option<Vec<String>>, LineProcessError> {
        if self.cond_expr.check(row) {
            Ok(Some(self.get_columns(row)?))
        } else {
            Ok(None)
        }
    }

    pub fn process_line_no_check(
        &self,
        row: &Vec<String>,
    ) -> Result<Vec<String>, LineProcessError> {
        Ok(self.get_columns(row)?)
    }

    fn get_columns(&self, row: &Vec<String>) -> Result<Vec<String>, LineProcessError> {
        let mut ans = Vec::new();

        for col_id in self.column_ids.iter() {
            match row.get(*col_id) {
                Some(s) => ans.push(s.to_string()),
                None => {
                    return Err(ColumnOutOfRangeError {
                        line: format!("{row:?}"),
                        col_num: col_id + 1,
                        col_count: row.len(),
                    }
                    .into());
                }
            }
        }

        Ok(ans)
    }
}
