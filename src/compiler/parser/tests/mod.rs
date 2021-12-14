// Copyright 2021 Hwakyeom Kim(=just-do-halee)
//! 'tests'

use super::*;

#[test]
fn displaying_expr() {
    let dummy = Source::new("-*");
    let minus = dummy.chop(0..1).unwrap();
    let multi = dummy.chop(1..2).unwrap();

    let expr = Expr::Binary(
        Expr::Unary(
            Token::new(TokenKind::Minus, minus, None),
            Expr::Literal(TokenLiteral::new(Object::Number(123f64.into()))).into(),
        )
        .into(),
        Token::new(TokenKind::Star, multi, None),
        Expr::Grouping(Expr::Literal(TokenLiteral::new(Object::String(45.67.to_string()))).into())
            .into(),
    );

    assert_eq!(expr.to_string(), "(* (- 123) (group \"45.67\"))");
}
