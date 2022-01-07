// Copyright 2021 Hwakyeom Kim(=just-do-halee)

#[macro_use]
mod macros;

pub use argone::{prelude::lazy_static, *};

pub use collectfiles::*;

pub use rayon::prelude::*;

pub use derive_new::*;

pub use anyhow::{anyhow, Context, Error, Result};

pub use console::{style, Term};

pub use phf::phf_map;

pub use crate::cli::ARGS;

pub use cursor::*;

pub use std::{
    cmp::*,
    collections::HashMap,
    env, error,
    ffi::{OsStr, OsString},
    fmt::{self, Display},
    fs::{self, File},
    hash::{Hash, Hasher},
    io::{self, BufWriter, Write},
    iter::{Iterator, Peekable},
    ops::{self, Deref, DerefMut, Range},
    path::{Path, PathBuf},
    slice::Iter,
    str::{Chars, FromStr},
    vec::IntoIter,
};

lazy_static! {
    pub static ref SRC_DIR: PathBuf = CURRENT_DIR.clone().join("src");
}

#[inline]
pub fn __boxed<T>(s: T) -> Box<T> {
    Box::new(s)
}
