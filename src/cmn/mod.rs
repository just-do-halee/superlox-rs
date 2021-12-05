// Copyright 2021 Hwakyeom Kim(=just-do-halee)

#[macro_use]
mod macros;

pub use argone::*;

pub use collectfiles::*;

pub use rayon::prelude::*;

pub use derive_new::*;

pub use anyhow::{anyhow, Context, Error, Result};

pub use crate::cli::ARGS;

pub use std::{
    ffi::{OsStr, OsString},
    fmt::{self, Display},
    fs,
    io::{self, Write},
    ops::Range,
    path::{Path, PathBuf},
};
