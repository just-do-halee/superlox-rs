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
        /// .lox files or directories
        [Config] src: Vec<PathBuf> = vec![{
                let mut cd = CURRENT_DIR.clone();
                cd.push("src");
                cd
            }];

        /// Level of searching directory
        (short, long)
        [Config] depth: Option<usize>

        /// IO Stream
        (long, name = "INPUT")
        io: Option<String>
    }
}
