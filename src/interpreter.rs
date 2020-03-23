use crate::ast::BinOp;
use crate::ast::Expr;
use crate::ast::Item;
use crate::ast::{ExprKind, LitKind};
use crate::lex;
use crate::parse;

pub struct Walker {}

impl Walker {
    pub fn new() -> Self {
        Walker {}
    }

    pub fn run(&self, src: &str) {
        let stream = parse::parse(lex::tokenize(src));
        for next in stream {
            match next {
                Ok(item) => {
                    let value = self.exec(item);
                    println!("{}", value);
                }
                Err(err) => {
                    eprintln!("{}", err);
                    break;
                }
            }
        }
    }

    fn exec(&self, item: Item) -> i64 {
        match item {
            Item::Expression(expr) => self.eval(expr),
        }
    }

    fn eval(&self, expr: Expr) -> i64 {
        match expr.kind {
            ExprKind::Bin(op, lhs, rhs) => self.eval_bin(op, *lhs, *rhs),
            ExprKind::Lit(kind) => self.eval_literal(kind),
        }
    }

    fn eval_bin(&self, op: BinOp, lhs: Expr, rhs: Expr) -> i64 {
        let lval = self.eval(lhs);
        let rval = self.eval(rhs);
        match op {
            BinOp::Add => lval + rval,
            BinOp::Sub => lval - rval,
        }
    }

    fn eval_literal(&self, lit: LitKind) -> i64 {
        match lit {
            LitKind::Int(num) => num as i64,
        }
    }
}
