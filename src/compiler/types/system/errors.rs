// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

derive_debug_partials! {
    #[derive(Clone, Copy)]
    pub enum ErrKind {
        Cursor,
        Parse,
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
