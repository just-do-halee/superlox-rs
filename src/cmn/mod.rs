// Copyright 2021 Hwakyeom Kim(=just-do-halee)

#[macro_use]
mod macros;

pub use macros::*;

pub use argone::*;

pub use collectfiles::*;

pub use rayon::prelude::*;

pub use std::path::PathBuf;

pub use crate::cli::ARGS;
