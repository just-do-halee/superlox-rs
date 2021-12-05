// Copyright 2021 Hwakyeom Kim(=just-do-halee)
//! `superlox-rs`
mod cmn;
use cmn::*;

mod cli;
mod compiler;

fn main() {
    let results = compiler::run();

    let term = Term::stderr();

    results.into_iter().for_each(|res| {
        writeln!(&term, "{n:⸻>40}Finished: {:?}", res, n = nl!())
            .with_context(fnerr!("print"))
            .unwrap()
    });
}
