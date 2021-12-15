// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

mod err;
mod traits;
pub use err::*;
pub use traits::*;

mod cursor;
mod source;
mod token;
pub use cursor::*;
pub use source::*;
pub use token::*;

mod number;
mod object;
pub use number::*;
pub use object::*;

mod expr;
mod visitor;
pub use expr::*;
pub use visitor::*;

pub type ProcessResult = Result<SourceHeader>;
