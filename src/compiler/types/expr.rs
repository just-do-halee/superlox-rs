// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

mod_boxed!(Expr<'s>);

derive_debug_partials! {
    #[derive(Clone)]
    pub enum Expr<'s> {
        Binary(boxed::Expr<'s>, Token<'s>, boxed::Expr<'s>),
        Grouping(boxed::Expr<'s>),
        Literal(TokenLiteral),
        Unary(Token<'s>, boxed::Expr<'s>),
    }
}
