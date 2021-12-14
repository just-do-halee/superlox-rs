// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

derive_debug_partials! {
    #[derive(Clone)]
    pub enum Object {
        Identifier(String),
        String(String),
        Number(Number),
        Boolean(bool),
        Nil,
        None,
    }
}

impl Display for Object {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Identifier(v) => write!(f, "{}", v),
            Object::String(v) => write!(f, "{:?}", v),
            Object::Number(v) => write!(f, "{}", v),
            Object::Boolean(v) => write!(f, "{}", v),
            Object::Nil => write!(f, "nil"),
            Object::None => write!(f, ""),
        }
    }
}

impl TryFrom<(TokenKind, Option<&str>)> for Object {
    type Error = Error;
    #[inline]
    fn try_from(value: (TokenKind, Option<&str>)) -> Result<Self, Self::Error> {
        let (kind, s) = value;
        let res = match kind {
            TokenKind::Identifier if s.is_some() => Object::Identifier(s.unwrap().to_string()),
            TokenKind::String if s.is_some() => Object::String(s.unwrap().to_string()),
            TokenKind::Number if s.is_some() => Object::Number(s.unwrap().try_into()?),
            TokenKind::True => Object::Boolean(true),
            TokenKind::False => Object::Boolean(false),
            TokenKind::Nil => Object::Nil,
            _ if s.is_none() => Object::None,
            _ => reterr!("{:?} (parse) ", s),
        };
        Ok(res)
    }
}
