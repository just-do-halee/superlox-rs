// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

mod_boxed!(Expr<'s>);

derive_debug_partials! {

    #[derive(Clone)]
    pub enum Expr<'s> {
        Binary(SourceChunk<'s>, boxed::Expr<'s>, Token<'s>, boxed::Expr<'s>),
        Grouping(SourceChunk<'s>, boxed::Expr<'s>),
        Literal(SourceChunk<'s>, TokenLiteral),
        Unary(SourceChunk<'s>, Token<'s>, boxed::Expr<'s>),
        Comma(SourceChunk<'s>, Vec<Expr<'s>>),
        None,
    }

}

impl<'s> AsRef<Expr<'s>> for Expr<'s> {
    #[inline]
    fn as_ref(&self) -> &Expr<'s> {
        self
    }
}

impl<'s> Display for Expr<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Binary(_, l_e, t, r_e) => {
                write!(f, "({} {} {})", t.lexeme, l_e, r_e)
            }
            Expr::Grouping(_, e) => {
                write!(f, "(group {})", e)
            }
            Expr::Literal(_, tl) => {
                write!(f, "{}", tl)
            }
            Expr::Unary(_, t, e) => {
                write!(f, "({} {})", t.lexeme, e)
            }
            Expr::Comma(_, v) => {
                let mut it = v.iter();
                write!(f, "{}", it.next().unwrap())?;

                for e in it {
                    write!(f, ", {}", e)?
                }

                Ok(())
            }
            Expr::None => write!(f, ""),
        }
    }
}
