#[derive(Debug)]
pub enum ConditionExpr {
    Condition(Box<dyn Condition>),
    Or(Box<ConditionExpr>, Box<ConditionExpr>),
    And(Box<ConditionExpr>, Box<ConditionExpr>),
}

impl ConditionExpr {
    pub fn check(&self, row: &Vec<String>) -> bool {
        match &self {
            ConditionExpr::Condition(cond) => cond.check(row),
            ConditionExpr::Or(lhs, rhs) => lhs.check(row) || rhs.check(row),
            ConditionExpr::And(lhs, rhs) => lhs.check(row) && rhs.check(row),
        }
    }
}

pub trait Condition: std::fmt::Debug {
    fn check(&self, row: &Vec<String>) -> bool;
}

#[derive(Debug)]
pub enum ConditionOperand {
    ColumnId(usize),
    Const(String),
}

impl ConditionOperand {
    fn value<'a>(&'a self, row: &'a Vec<String>) -> &'a str {
        match self {
            ConditionOperand::ColumnId(col_id) => &row[*col_id],
            ConditionOperand::Const(c) => c,
        }
    }
}

#[derive(Debug)]
pub struct EqCondition(pub ConditionOperand, pub ConditionOperand);

impl Condition for EqCondition {
    fn check(&self, row: &Vec<String>) -> bool {
        self.0.value(row) == self.1.value(row)
    }
}

#[derive(Debug)]
pub struct TrueCondition;

impl Condition for TrueCondition {
    fn check(&self, _row: &Vec<String>) -> bool {
        true
    }
}
