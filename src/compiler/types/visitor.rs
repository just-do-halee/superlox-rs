// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

derive_debug_partials! {

    #[derive(Clone, Copy)]
    pub struct ExprVisitor;

}

impl<'a, Expression> Visitor<Expression, Result<Object>> for ExprVisitor
where
    Expression: AsRef<Expr<'a>>,
{
    /// consume
    #[inline]
    fn visit(&self, e: Expression) -> Result<Object> {
        let e = e.as_ref();
        Ok(match e {
            Expr::Binary(l, token, r) => {
                let left = self.visit(l)?;
                let right = self.visit(r)?;
                let kind = token.kind;

                match (left, right, kind) {
                    (l, r, TokenKind::EqualEqual) => Object::Boolean(l == r),
                    (l, r, TokenKind::BangEqual) => Object::Boolean(l != r),

                    (Object::String(left), Object::String(right), _) => match kind {
                        TokenKind::Plus => Object::String(format!("{}{}", left, right)),
                        _ => reterr!(token.to_error_with_kind(
                            ErrKind::Parse,
                            "Please consider string concatenating operator '+'."
                        )),
                    },
                    (Object::Number(left), Object::Number(right), _) => match token.kind {
                        // arithmetic
                        TokenKind::Plus => Object::Number(left + right),
                        TokenKind::Star => Object::Number(left * right),
                        TokenKind::Minus => Object::Number(left - right),
                        TokenKind::Slash => Object::Number(left / right),

                        // comparison
                        TokenKind::Less => Object::Boolean(left < right),
                        TokenKind::LessEqual => Object::Boolean(left <= right),
                        TokenKind::Greater => Object::Boolean(left > right),
                        TokenKind::GreaterEqual => Object::Boolean(left >= right),

                        // bitwise
                        TokenKind::Ampersand => Object::Number(left & right),
                        TokenKind::VerticalBar => Object::Number(left | right),
                        TokenKind::Circumflex => Object::Number(left ^ right),

                        _ => reterr!(token.to_error_with_kind(ErrKind::Parse, "Unknown operator.")),
                    },
                    _ => reterr!(
                        token.to_error_with_kind(ErrKind::Parse, "Unexpected binary syntax.")
                    ),
                }
            }

            Expr::Grouping(expr) => self.visit(expr)?,

            Expr::Literal(literal) => literal.to_object(),

            Expr::Unary(token, expr) => {
                let right = self.visit(expr)?;
                match (token.kind, right) {
                    (TokenKind::Bang, object) => !object,
                    (TokenKind::Minus, Object::Number(number)) => Object::Number(-number),
                    _ => {
                        reterr!(token.to_error_with_kind(ErrKind::Parse, "Unexpected unary syntax."))
                    }
                }
            }

            Expr::Comma(exprs) => {
                let mut vec = Vec::new();

                for expr in exprs.iter() {
                    let expr = self.visit(expr)?;
                    vec.push(expr);
                }
                Object::TempArray(vec)
            }

            Expr::None => Object::None,
        })
    }
}
