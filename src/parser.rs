use std::collections::{hash_map::Entry, HashMap};

use crate::query::Query;
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct QueryParser;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum ParseError {
    PestError(#[from] pest::error::Error<Rule>),
    InvalidHeaderError(#[from] SameColumnNamesError),
    ColumnNotFoundError(#[from] ColumnNotFoundError),
}

#[derive(Debug, Error)]
#[error("columns {} and {} has the same name: {}", first_column_id + 1, second_column_id + 1, column_name)]
pub struct SameColumnNamesError {
    first_column_id: usize,
    second_column_id: usize,
    column_name: String,
}

#[derive(Debug, Error)]
#[error("column with name {column_name} not found")]
pub struct ColumnNotFoundError {
    column_name: String,
}

#[derive(Debug)]
pub struct QueryParserAns {
    pub query: Query,
    pub header_required: bool,
}

pub fn parse(query_string: &str, first_row: &Vec<String>) -> Result<QueryParserAns, ParseError> {
    // Lazyly create map from first_row as a header
    let mut header_used = false;
    let mut header: HashMap<String, usize> = HashMap::new();
    let mut get_column_number = |column_name: &str| -> Result<usize, ParseError> {
        if !header_used {
            header_used = true;
            header = HashMap::new();

            for (col_id, col_name) in first_row.iter().enumerate() {
                match header.entry(col_name.to_string()) {
                    Entry::Occupied(entry) => {
                        return Err(SameColumnNamesError {
                            first_column_id: *entry.get(),
                            second_column_id: col_id,
                            column_name: col_name.clone(),
                        }
                        .into())
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(col_id);
                    }
                }
            }
        }

        header
            .get(column_name)
            .ok_or(
                ColumnNotFoundError {
                    column_name: column_name.to_string(),
                }
                .into(),
            )
            .copied()
    };

    // Actually parse
    let pairs = QueryParser::parse(Rule::query, query_string)?
        .next()
        .unwrap()
        .into_inner();

    let mut column_ids = Vec::new();
    for pair in pairs {
        if pair.as_rule() == Rule::EOI {
            break;
        }

        assert!(pair.as_rule() == Rule::column);
        let pair = pair.into_inner().next().unwrap();

        column_ids.push(match pair.as_rule() {
            Rule::column_number => pair.as_str().parse::<usize>().unwrap() - 1,
            Rule::column_name => get_column_number(pair.as_str())?,
            _ => unreachable!(),
        })
    }

    Ok(QueryParserAns {
        query: Query { column_ids },
        header_required: header_used,
    })
}
