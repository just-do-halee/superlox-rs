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
        None,
    }

}

impl<'s> Display for Expr<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Binary(l_e, t, r_e) => {
                write!(f, "({} {} {})", t.lexeme, l_e, r_e)
            }
            Expr::Grouping(e) => {
                write!(f, "(group {})", e)
            }
            Expr::Literal(tl) => {
                write!(f, "{}", tl)
            }
            Expr::Unary(t, e) => {
                write!(f, "({} {})", t.lexeme, e)
            }
            Expr::None => write!(f, ""),
        }
    }
}
