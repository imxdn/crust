use crate::{parser::expression::Expr, scanner::types::Token};

use super::expression::LiteralValue;

pub trait Visitor<R> {
    fn visit_unary_expr(&self, op: &Token, right: &Box<Expr>) -> R;
    fn visit_binary_expr(&self, left: &Box<Expr>, op: &Token, right: &Box<Expr>) -> R;
    fn visit_grouping_expr(&self, expr: &Box<Expr>) -> R;
    fn visit_literal_expr(&self, literal: &LiteralValue) -> R;
}
