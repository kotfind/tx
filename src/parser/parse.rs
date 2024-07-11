use super::{ParseError, QueryParser, Rule};
use crate::query::Query;
use pest::iterators::Pair;

impl<'a> QueryParser<'a> {
    pub(super) fn parse_query(&mut self, query: Pair<Rule>) -> Result<Query, ParseError> {
        assert!(query.as_rule() == Rule::query);
        let pairs = query.into_inner();

        let mut column_ids = Vec::new();
        for pair in pairs {
            if pair.as_rule() == Rule::EOI {
                break;
            }

            assert!(pair.as_rule() == Rule::column);
            let pair = pair.into_inner().next().unwrap();

            column_ids.push(match pair.as_rule() {
                Rule::column_number => pair.as_str().parse::<usize>().unwrap() - 1,
                Rule::column_name => self.get_column_number(pair.as_str())?,
                _ => unreachable!(),
            })
        }

        Ok(Query { column_ids })
    }
}
