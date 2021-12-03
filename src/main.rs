// Copyright 2021 Hwakyeom Kim(=just-do-halee)
//! `superlox-rs`

mod cmn;
use cmn::*;

mod cli;
mod compiler;

fn main() {
    for res in compiler::run() {
        eprintln!("{:⸻>40}\nFinished: {:?}", "", res);
    }
}
