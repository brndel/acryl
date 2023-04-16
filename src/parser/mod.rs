use chumsky::{prelude::Simple, primitive::end, Parser};

use self::math::Expr;

mod math;

pub fn parser() -> impl Parser<char, Expr, Error = Simple<char>> {
  math::parser().then_ignore(end())
}
