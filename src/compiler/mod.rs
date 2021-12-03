// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

mod types;
use types::*;

mod lexer;

#[inline]
pub fn run() -> Vec<ProcessResult> {
    if ARGS.io.is_some() {
        // --io <INPUT>
        vec![process(None)]
    } else {
        // files
        collect_src_files!()
            .into_par_iter()
            .map(|path| -> ProcessResult {
                // inner thread
                process(Some(path))
            })
            .collect()
    }
}

#[inline]
pub fn process(some_path: Option<PathBuf>) -> ProcessResult {
    let source = {
        match some_path {
            // files
            Some(path) => {
                let body = fs::read_to_string(&path)
                    .with_context(fnerr!("couldn't read: {}", path.display()))?;
                Source::new(path, body)
            }
            // --io <INPUT>
            None => match &ARGS.io {
                Some(input) => Source::new(pathbuf!("::io::"), input.clone()),
                None => reterr!("'--io <INPUT>' requires a value."),
            },
        }
    };

    // main process

    let out = lexer::run(source);

    Ok(out.path)
}
