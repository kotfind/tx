pub use error::ParseError;

use crate::query::Query;
use error::*;
use pest::{
    pratt_parser::{Assoc, Op, PrattParser},
    Parser,
};
use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
};

mod error;
mod parse;

#[derive(pest_derive::Parser)]
#[grammar = "parser/grammar.pest"]
struct PestParser;

#[derive(Debug)]
pub struct QueryParseAns {
    pub is_header_required: bool,
    pub query: Query,
}

pub fn parse(query_string: &str, first_row: &Vec<String>) -> Result<QueryParseAns, ParseError> {
    let parser = QueryParser::from_first_row(first_row);
    let query = parser.parse(query_string)?;
    let is_header_required = parser.header.borrow().is_some();

    Ok(QueryParseAns {
        is_header_required,
        query,
    })
}

struct QueryParser<'a> {
    header: RefCell<Option<HashMap<String /* col_name */, usize /* col_id */>>>,
    first_row: &'a Vec<String>,
    pratt: PrattParser<Rule>,
}

impl<'a> QueryParser<'a> {
    fn from_first_row(first_row: &'a Vec<String>) -> QueryParser {
        let pratt = PrattParser::new()
            .op(Op::infix(Rule::cond_expr_op_or, Assoc::Left))
            .op(Op::infix(Rule::cond_expr_op_and, Assoc::Left));

        QueryParser {
            header: RefCell::new(None),
            pratt,
            first_row,
        }
    }

    fn parse(&self, query_string: &str) -> Result<Query, ParseError> {
        let query = PestParser::parse(Rule::query, query_string)?
            .next()
            .unwrap();

        self.parse_query(query)
    }

    fn init_header(&self) -> Result<(), SameColumnNamesError> {
        let mut header = HashMap::new();

        for (col_id, col_name) in self.first_row.iter().enumerate() {
            match header.entry(col_name.to_string()) {
                Entry::Occupied(entry) => {
                    return Err(SameColumnNamesError {
                        first_column_id: *entry.get(),
                        second_column_id: col_id,
                        column_name: col_name.clone(),
                    })
                }
                Entry::Vacant(entry) => {
                    entry.insert(col_id);
                }
            }
        }

        *self.header.borrow_mut() = Some(header);

        Ok(())
    }

    fn get_column_number(&self, column_name: &str) -> Result<usize, ParseError> {
        if self.header.borrow().is_none() {
            self.init_header()?;
        }

        self.header
            .borrow_mut()
            .as_ref()
            .unwrap()
            .get(column_name)
            .ok_or(
                ColumnNotFoundError {
                    column_name: column_name.to_string(),
                }
                .into(),
            )
            .copied()
    }
}
