// Copyright 2021 Hwakyeom Kim(=just-do-halee)

#[macro_use]
mod macros;

pub use macros::*;

pub use argone::*;

pub use collectfiles::*;

pub use rayon::prelude::*;

pub use derive_new::*;

pub use anyhow::{anyhow, Context, Result};

pub use crate::cli::ARGS;

pub use std::{
    fs,
    path::{Path, PathBuf},
};
