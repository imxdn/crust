use crate::scanner::types::Token;
use serde::{Deserialize, Serialize};

use crate::parser::visitor::Visitor;

#[derive(Serialize, Deserialize, Debug)]
pub enum Expr {
    Unary {
        op: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
    Literal {
        val: LiteralValue,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LiteralValue {
    Int(i64),
    Float(f64),
}

impl Expr {
    pub fn accept<R, T: Visitor<R>>(&self, visitor: &T) -> R {
        match self {
            Self::Unary { op, right } => visitor.visit_unary_expr(op, right),
            Self::Binary { left, op, right } => visitor.visit_binary_expr(left, op, right),
            Self::Grouping { expr } => visitor.visit_grouping_expr(expr),
            Self::Literal { val: value } => visitor.visit_literal_expr(value),
        }
    }
}
