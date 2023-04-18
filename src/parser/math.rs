use chumsky::prelude::*;

use super::AcrylError;

#[derive(Debug)]
pub enum Expr {
  Num(i64),
  Neg(Box<Self>),
  Add(Box<Self>, Box<Self>),
  Sub(Box<Self>, Box<Self>),
  Mul(Box<Self>, Box<Self>),
  Div(Box<Self>, Box<Self>),
}

impl Expr {
  pub fn eval(&self) -> i64 {
    match self {
      Expr::Num(value) => *value,
      Expr::Neg(value) => -value.eval(),
      Expr::Add(a, b) => a.eval() + b.eval(),
      Expr::Sub(a, b) => a.eval() - b.eval(),
      Expr::Mul(a, b) => a.eval() * b.eval(),
      Expr::Div(a, b) => a.eval() / b.eval(),
    }
  }
}

pub fn parser<'src>() -> impl Parser<'src, &'src str, Expr, AcrylError<'src>> {
  recursive(|expr| {
    let num = text::int(10)
      .map(|s: &str| Expr::Num(s.parse().unwrap()))
      .padded();

    let atom = num.or(expr.delimited_by(just('('), just(')'))).padded();

    let op = |c| just(c).padded();

    let unary = op('-')
      .repeated()
      .foldr(atom, |_op, rhs| Expr::Neg(Box::new(rhs)));

    let product = unary.clone().foldl(
      choice((
        op('*').to(Expr::Mul as fn(_, _) -> _),
        op('/').to(Expr::Div as fn(_, _) -> _),
      )).then(unary.clone()).repeated(),
      |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
    );

    let sum = product.clone().foldl(
      choice((
        op('+').to(Expr::Add as fn(_, _) -> _),
        op('-').to(Expr::Sub as fn(_, _) -> _),
      )).then(product.clone()).repeated(),
      |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
    );

    sum
  })
}
