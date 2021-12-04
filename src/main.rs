// Copyright 2021 Hwakyeom Kim(=just-do-halee)
//! `superlox-rs`
mod cmn;
use cmn::*;

mod cli;
mod compiler;

fn main() {
    let stderr = io::stderr();
    let mut handle = stderr.lock();
    compiler::run().into_iter().for_each(|res| {
        write!(handle, "{:⸻>40}\nFinished: {:?}\n", "", res)
            .with_context(fnerr!("print"))
            .unwrap()
    });
}
