// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

#[derive(Debug, Clone)]
pub struct TokenParser<'s> {
    cursor: TokenCursor<'s>,
    caught: bool,
}

impl<'s> TokenParser<'s> {
    #[inline]
    pub fn new<T: Into<Tokens<'s>>>(tokens: T) -> Result<Self> {
        let cursor = TokenCursor::new(tokens)?;
        Ok(Self {
            cursor,
            caught: false,
        })
    }
}

impl<'s> Parser<'s> for TokenParser<'s> {
    #[inline]
    fn CAUGHT(&self) -> bool {
        self.caught
    }

    #[inline]
    fn init(&mut self) -> Result<()> {
        self.caught = false;
        Ok(())
    }

    #[inline]
    /// discards tokens until founds a statement boundary
    fn synchronize(&mut self) {
        loop {
            let token = self.cursor.first(); // peek the next
            match token.kind {
                TokenKind::Semicolon => {
                    self.cursor.bump(); // eat that semicolon
                    break;
                }
                TokenKind::Class
                | TokenKind::Fun
                | TokenKind::Var
                | TokenKind::For
                | TokenKind::If
                | TokenKind::While
                | TokenKind::Print
                | TokenKind::Return => break, // stop right in front of keyword
                _ => {
                    self.cursor.bump();
                    continue; // skip
                }
            }
        }
    }

    #[inline]
    fn catch(&mut self, e: Error) -> Result<Expr<'s>> {
        if let Some(err) = e.downcast_ref::<Err>() {
            if err.kind == ErrKind::Parse {
                self.caught = true;
                // ...
                return Ok(Expr::None);
            }
        }
        Err(e)
    }

    #[inline]
    fn expression(&mut self) -> Result<Expr<'s>> {
        // + comma expressions
        // TODO: make an exception function call's argument list
        let expr = self.equality()?;

        Ok(if self.cursor.first().kind == TokenKind::Comma {
            self.cursor.bump();
            let mut vec = vec![expr, self.equality()?];

            while self.cursor.first().kind == TokenKind::Comma {
                self.cursor.bump();
                vec.push(self.equality()?);
            }

            Expr::Comma(vec)
        } else {
            expr
        })
    }

    #[inline]
    fn equality(&mut self) -> Result<Expr<'s>> {
        let mut expr = self.comparison()?;

        while let TokenKind::EqualEqual | TokenKind::BangEqual = self.cursor.first().kind {
            let left = expr.into();
            let operator = self.cursor.bump();
            let right = self.comparison()?.into();

            expr = Expr::Binary(left, operator, right);
        }

        Ok(expr)
    }

    #[inline]
    fn comparison(&mut self) -> Result<Expr<'s>> {
        let mut expr = self.bitwise()?;

        while let TokenKind::Less
        | TokenKind::LessEqual
        | TokenKind::Greater
        | TokenKind::GreaterEqual = self.cursor.first().kind
        {
            let left = expr.into();
            let operator = self.cursor.bump();
            let right = self.bitwise()?.into();

            expr = Expr::Binary(left, operator, right);
        }

        Ok(expr)
    }

    #[inline]
    fn bitwise(&mut self) -> Result<Expr<'s>> {
        let mut expr = self.term()?;

        while let TokenKind::Ampersand | TokenKind::VerticalBar | TokenKind::Circumflex =
            self.cursor.first().kind
        {
            let left = expr.into();
            let operator = self.cursor.bump();
            let right = self.term()?.into();

            expr = Expr::Binary(left, operator, right);
        }

        Ok(expr)
    }

    #[inline]
    fn term(&mut self) -> Result<Expr<'s>> {
        let mut expr = self.factor()?;

        while let TokenKind::Plus | TokenKind::Minus = self.cursor.first().kind {
            let left = expr.into();
            let operator = self.cursor.bump();
            let right = self.factor()?.into();

            expr = Expr::Binary(left, operator, right);
        }

        Ok(expr)
    }

    #[inline]
    fn factor(&mut self) -> Result<Expr<'s>> {
        let mut expr = self.unary()?;

        while let TokenKind::Star | TokenKind::Slash = self.cursor.first().kind {
            let left = expr.into();
            let operator = self.cursor.bump();
            let right = self.unary()?.into();

            expr = Expr::Binary(left, operator, right);
        }

        Ok(expr)
    }

    #[inline]
    fn unary(&mut self) -> Result<Expr<'s>> {
        if let TokenKind::Bang | TokenKind::Minus = self.cursor.first().kind {
            let operator = self.cursor.bump();
            let right = self.unary()?.into();

            Ok(Expr::Unary(operator, right))
        } else {
            self.primary()
        }
    }

    #[inline]
    fn primary(&mut self) -> Result<Expr<'s>> {
        let token = self.cursor.bump();
        Ok(match token.kind {
            TokenKind::True
            | TokenKind::False
            | TokenKind::Nil
            | TokenKind::Number
            | TokenKind::String => Expr::Literal(token.literal),
            TokenKind::LeftParen => {
                let expr = self.expression()?.into();
                let next_token = self.cursor.bump();

                if next_token.kind != TokenKind::RightParen {
                    reterr!(next_token
                        .to_error_with_kind(ErrKind::Parse, "Expect ')' after expression."));
                }
                Expr::Grouping(expr)
            }
            _ => reterr!(token.to_error_with_kind(ErrKind::Parse, "Expect expression.")),
        })
    }
}
