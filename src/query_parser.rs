use crate::query::{Column, Query};
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
}

pub fn parse(query_string: &str) -> Result<Query, ParseError> {
    let pairs = QueryParser::parse(Rule::query, query_string)?
        .next()
        .unwrap()
        .into_inner();

    let mut columns = Vec::new();
    for pair in pairs {
        if pair.as_rule() == Rule::EOI {
            break;
        }
        assert!(pair.as_rule() == Rule::column);
        columns.push(Column(pair.as_str().parse::<usize>().unwrap() - 1))
    }

    columns.sort_by_key(|c| c.0);

    Ok(Query {
        column_ids: columns,
    })
}
