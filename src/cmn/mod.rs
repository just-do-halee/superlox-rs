// Copyright 2021 Hwakyeom Kim(=just-do-halee)

#[macro_use]
mod macros;

pub use argone::{prelude::lazy_static, *};

pub use collectfiles::*;

pub use rayon::prelude::*;

pub use derive_new::*;

pub use anyhow::{anyhow, Context, Error, Result};

pub use console::Term;

pub use phf::phf_map;

pub use crate::cli::ARGS;

pub use std::{
    collections::HashMap,
    env,
    ffi::{OsStr, OsString},
    fmt::{self, Display},
    fs::{self, File},
    io::{self, BufWriter, Write},
    ops::{Deref, DerefMut, Range},
    path::{Path, PathBuf},
    str::{Chars, FromStr},
};

lazy_static! {
    pub static ref SRC_DIR: PathBuf = CURRENT_DIR.clone().join("src");
}

#[inline]
pub fn __boxed<T>(s: T) -> Box<T> {
    Box::new(s)
}
