// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

pub fn run() {
    collect_src_files!().into_par_iter().for_each(|path| {
        println!("pwd: {:?}\n{:#?}, src: {:#?}", *CURRENT_DIR, path, ARGS.src);
    });
}
