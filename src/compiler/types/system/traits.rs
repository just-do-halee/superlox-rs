// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

pub trait FreeErrorConverter {
    fn to_error_with_kind<D: Display>(&self, kind: ErrKind, message: D) -> Error;
    #[inline]
    fn into_error_with_kind<D: Display>(self, kind: ErrKind, message: D) -> Error
    where
        Self: Sized,
    {
        self.to_error_with_kind(kind, message)
    }
}

pub trait ErrorConverter {
    fn to_error<D: Display>(&self, message: D) -> Error;
    #[inline]
    fn into_error<D: Display>(self, message: D) -> Error
    where
        Self: Sized,
    {
        self.to_error(message)
    }
}

pub trait Chopable<'s> {
    type Out;
    fn chop<A: Into<Span>>(&'s self, span: A) -> Option<Self::Out>;
    #[inline]
    fn chops<A: Into<Span>>(&'s self, spans: Vec<A>) -> Vec<Option<Self::Out>> {
        spans.into_iter().map(|span| self.chop(span)).collect()
    }
}

pub trait Cursor {
    type Item: Clone;
    type Iter: Iterator<Item = Self::Item>;

    /// EOF_ITEM
    #[allow(non_snake_case)]
    fn EOF(&self) -> Self::Item;

    /// returns a `Self::Iter` over the remaining items
    fn clone_items(&self) -> Self::Iter;

    /// returns a current item as mutable borrowed
    fn current(&mut self) -> &mut Self::Item;

    /// advances the iterator and returns the next value
    fn next(&mut self) -> Option<Self::Item>;

    /// update inner state
    fn flush(&mut self);

    /// advances the iterator and returns the next value
    #[inline]
    fn next_unwrap_or_eof(&mut self) -> Self::Item {
        self.next().unwrap_or_else(|| self.EOF())
    }

    /// moves to the next character
    #[inline]
    fn bump(&mut self) -> Self::Item {
        *self.current() = self.next_unwrap_or_eof();
        self.flush();
        self.current().clone()
    }

    /// moves to the next character
    /// without flushing [pos/line/column]
    #[inline]
    fn bump_without_flush(&mut self) -> Self::Item {
        *self.current() = self.next_unwrap_or_eof();
        self.current().clone()
    }

    /// returns nth items relative to the current cursor position
    /// if requested position doesn't exist, `Self::EOF` is returned
    #[inline]
    fn nth_items(&self, n: usize) -> Self::Item {
        self.clone_items().nth(n).unwrap_or_else(|| self.EOF())
    }

    /// peeks the next symbol from the input stream without consuming it
    #[inline]
    fn first(&self) -> Self::Item {
        self.nth_items(0)
    }

    /// peeks the second symbol from the input stream without consuming it
    #[inline]
    fn second(&self) -> Self::Item {
        self.nth_items(1)
    }
}

pub trait Visitor<T> {
    fn visit<E: AsMut<T>>(&self, expr: E);
}

pub type Caught = bool;

pub trait Parser<'s> {
    /// CAUGHT
    #[allow(non_snake_case)]
    fn CAUGHT(&self) -> bool;

    fn init(&mut self) -> Result<()>;

    #[inline]
    fn parse(&mut self) -> (Caught, Result<Expr<'s>>) {
        if let Err(e) = self.init() {
            return (false, Err(e));
        }
        let res = match self.expression() {
            Err(e) => self.catch(e),
            v => v,
        };
        (self.CAUGHT(), res)
    }
    fn catch(&mut self, e: Error) -> Result<Expr<'s>>;
    fn synchronize(&mut self);
    fn expression(&mut self) -> Result<Expr<'s>>;
    fn equality(&mut self) -> Result<Expr<'s>>;
    fn comparison(&mut self) -> Result<Expr<'s>>;
    fn bitwise(&mut self) -> Result<Expr<'s>>;
    fn term(&mut self) -> Result<Expr<'s>>;
    fn factor(&mut self) -> Result<Expr<'s>>;
    fn unary(&mut self) -> Result<Expr<'s>>;
    fn primary(&mut self) -> Result<Expr<'s>>;
}
