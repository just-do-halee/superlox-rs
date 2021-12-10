// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

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

pub trait Visitor<T> {
    fn visit<E: AsMut<T>>(&self, expr: E);
}
