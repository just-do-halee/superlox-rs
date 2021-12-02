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
        {
            let mut cf = CollectFiles($path)
                            .with_target_regex(
                                &format!(".{}$",
                                collect_src_files!(@extension))
                            );

            if let Some(level) = ARGS.depth { // depth
                cf = cf.with_depth(level);
            }

            cf.collect()
        }
    };
    (
        $($path:expr),+
    ) => {
        {
            let mut tree = Vec::new();
            for path in [$($path),*] {
                tree.extend(collect_src_files!(@collect path).into_iter());
            }
            tree
        }
    };
    () => {
        {
            let mut tree = Vec::new();
            for path in &ARGS.src {
                tree.extend(collect_src_files!(@collect path).into_iter());
            }
            tree
        }
    };
}
pub use collect_src_files;
