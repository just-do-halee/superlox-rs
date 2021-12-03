// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

#[derive(Debug, new)]
pub struct Source {
    pub path: PathBuf,
    pub body: String,
}
