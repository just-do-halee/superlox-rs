// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

mod traits;

pub use traits::*;

mod source;

pub use source::*;

pub type ProcessResult = Result<SourceHeader>;
