// Copyright 2021 Hwakyeom Kim(=just-do-halee)

#[macro_export]
macro_rules! collect_src_files {
    (
        @extension
    ) => {
        ".lox"
    };
    (
        @collect $path:expr
    ) => {
        if $path.is_file() {

            vec![$path.to_owned()]

        } else if $path.is_dir() {

            let mut cf = CollectFiles($path)
                            .with_target_regex(
                                &format!(".{}$",
                                collect_src_files!(@extension))
                            )
                            .with_unwrap_or_else(|e| {
                                panic!("source target path: {:?}", e.kind());
                            });

            if let Some(level) = ARGS.depth { // depth
                cf = cf.with_depth(level);
            }

            cf.collect()

        } else {

            panic!("target path {:?}: NotFound", $path)

        }
    };
    (@collect_tree $target:expr) => {
        {
            let mut tree = Vec::new();
            for path in $target {
                tree.extend(collect_src_files!(@collect path).into_iter());
            }
            tree.sort_unstable();
            tree.dedup();
            tree
        }
    };
    (
        $($path:expr),+
    ) => {
        collect_src_files!(@collect_tree [$($path),*])
    };
    () => {
        collect_src_files!(@collect_tree &ARGS.src)
    };
}
pub use collect_src_files;
