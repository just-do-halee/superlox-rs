// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

pub fn run() {
    println!("{:#?}", *ARGS);
    collect_src_files!().into_par_iter().for_each(|path| {
        println!("\n{:#?}", path);
    });
}
