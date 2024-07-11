use super::Rule;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("pest parser error")]
    PestError(#[from] pest::error::Error<Rule>),

    #[error(transparent)]
    SameColumnNamesError(#[from] SameColumnNamesError),

    #[error(transparent)]
    ColumnNotFoundError(#[from] ColumnNotFoundError),
}

#[derive(Debug, thiserror::Error)]
#[error("columns {} and {} has the same name: {}", first_column_id + 1, second_column_id + 1, column_name)]
pub struct SameColumnNamesError {
    pub(super) first_column_id: usize,
    pub(super) second_column_id: usize,
    pub(super) column_name: String,
}

#[derive(Debug, thiserror::Error)]
#[error("column with name {column_name} not found")]
pub struct ColumnNotFoundError {
    pub(super) column_name: String,
}
