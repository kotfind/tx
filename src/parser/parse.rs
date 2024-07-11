use super::{ParseError, QueryParser, Rule};
use crate::{
    condition::{Condition, ConditionExpr, ConditionOperand, EqCondition, TrueCondition},
    query::Query,
};
use itertools::Itertools;
use pest::iterators::Pair;

type ParseResult<T> = Result<T, ParseError>;

impl<'a> QueryParser<'a> {
    pub(super) fn parse_query(&mut self, query: Pair<Rule>) -> ParseResult<Query> {
        assert!(query.as_rule() == Rule::query);
        let pairs = query.into_inner();

        let mut column_ids = Vec::new();
        let mut cond_expr = ConditionExpr::Condition(Box::new(TrueCondition));

        for pair in pairs {
            match pair.as_rule() {
                Rule::EOI => {
                    break;
                }
                Rule::column => {
                    column_ids.push(self.parse_column(pair)?);
                }
                Rule::_if => {
                    let inner = pair.into_inner().next().unwrap();
                    cond_expr = self.parse_cond_expr(inner)?;
                }
                _ => unreachable!(),
            }
        }

        Ok(Query {
            column_ids,
            cond_expr,
        })
    }

    /// Returns col_id.
    fn parse_column(&mut self, column: Pair<Rule>) -> ParseResult<usize> {
        assert!(column.as_rule() == Rule::column);

        let pair = column.into_inner().next().unwrap();

        Ok(match pair.as_rule() {
            Rule::column_number => pair.as_str().parse::<usize>().unwrap() - 1,
            Rule::column_name => self.get_column_number(pair.as_str())?,
            _ => unreachable!(),
        })
    }

    fn parse_cond_expr(&mut self, cond_expr: Pair<Rule>) -> ParseResult<ConditionExpr> {
        assert!(cond_expr.as_rule() == Rule::cond_expr);
        let pairs = cond_expr.into_inner();

        self.pratt
            .map_primary(|cond| Ok(ConditionExpr::Condition(self.parse_cond(cond)?)))
            .map_infix(|lhs, op, rhs| {
                let lhs = Box::new(lhs?);
                let rhs = Box::new(rhs?);
                Ok(match op.as_rule() {
                    Rule::cond_expr_op_or => ConditionExpr::Or(lhs, rhs),
                    Rule::cond_expr_op_and => ConditionExpr::And(lhs, rhs),
                    _ => unreachable!(),
                })
            })
            .parse(pairs)
    }

    fn parse_cond(&mut self, cond: Pair<Rule>) -> ParseResult<Box<dyn Condition>> {
        assert!(cond.as_rule() == Rule::cond);
        let mut pairs = cond.into_inner();

        let lhs = self.parse_cond_operand(pairs.next().unwrap())?;
        let op = pairs.next().unwrap();
        let rhs = self.parse_cond_operand(pairs.next().unwrap())?;
        assert!(pairs.next().is_none());

        Ok(match op.as_rule() {
            Rule::cond_op_eq => Box::new(EqCondition(lhs, rhs)),
            _ => unreachable!(),
        })
    }

    fn parse_cond_operand(&mut self, cond_operand: Pair<Rule>) -> ParseResult<ConditionOperand> {
        assert!(cond_operand.as_rule() == Rule::cond_operand);
        let inner = cond_operand.into_inner().next().unwrap();

        Ok(match inner.as_rule() {
            Rule::column => ConditionOperand::ColumnId(self.parse_column(inner)?),
            Rule::string => ConditionOperand::Const(self.parse_str(inner)?),
            _ => unreachable!(),
        })
    }

    fn parse_str(&mut self, s: Pair<Rule>) -> ParseResult<String> {
        assert!(s.as_rule() == Rule::string);
        let mut chars = s.as_str().chars().collect_vec();
        assert!(chars.len() >= 2 && chars[0] == '"' && *chars.last().unwrap() == '"');
        chars.pop();
        chars.remove(0);
        Ok(chars.iter().join(""))
    }
}
