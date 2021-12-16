// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

derive_debug_partials! {

    #[derive(Clone, Copy)]
    pub enum ErrKind {
        Cursor,
        Parse,
        Runtime,
        None,
    }

    #[derive(Clone, new)]
    pub struct ErrOpt {
        pub kind: Option<ErrKind>,
        pub message: Option<String>,
        pub red: Option<Span>,
    }

}

impl Default for ErrKind {
    #[inline]
    fn default() -> Self {
        ErrKind::None
    }
}

#[derive(Clone, new)]
pub struct Err {
    pub message: String,
    pub kind: ErrKind,
}

impl Display for Err {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for Err {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for Err {}
