// Copyright 2021 Hwakyeom Kim(=just-do-halee)
//! `superlox-rs`

mod cmn;
use cmn::*;

mod cli;
mod compiler;

fn main() {
    compiler::run();
}
