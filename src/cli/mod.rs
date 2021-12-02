// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

use argone::prelude::*;

ARGONE! {

    version = "0.1"
    author = "just-do-halee <just.do.halee@gmail.com>"
    about = "Lox interpreter"

    Config {
        file = "test/loxconfig"
        prefix = "LOX"
    }

    Args {
        [Config] src: Vec<PathBuf>

        (short, long)
        [Config] depth: Option<usize>
    }
}
