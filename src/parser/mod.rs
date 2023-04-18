use chumsky::prelude::*;

use self::math::Expr;

mod math;

type AcrylError<'src> = extra::Err<Rich<'src, char, SimpleSpan<usize>>>;

pub fn parser<'src>() -> impl Parser<'src, &'src str, Expr, AcrylError<'src>> {
    math::parser().then_ignore(end())
}
