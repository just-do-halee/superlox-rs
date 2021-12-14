// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

#[macro_use]
mod macros;

mod types;
use types::*;

mod lexer;
mod parser;

#[inline]
pub fn run() -> Vec<ProcessResult> {
    if ARGS.io.is_none() {
        // files
        collect_src_files!()
            .into_par_iter()
            .map(|path| -> ProcessResult {
                // inner thread
                process(Some(path))
            })
            .collect()
    } else {
        // --io <INPUT>
        vec![process(None)]
    }
}

#[inline]
pub fn process(some_path: Option<PathBuf>) -> ProcessResult {
    let source = {
        match some_path {
            // files
            Some(path) => Source::try_from(path)?,
            // --io <INPUT>
            None => match &ARGS.io {
                Some(input) => Source::new(input),
                None => reterr!("'--io <INPUT>' requires a value."),
            },
        }
    };

    // main process

    let out = lexer::run(&source)?;

    let out = parser::run(out)?;

    eprintln!("{}", out);

    Ok(source.head)
}
