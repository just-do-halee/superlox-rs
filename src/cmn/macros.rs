// Copyright 2021 Hwakyeom Kim(=just-do-halee)

#[macro_export]
macro_rules! name {
    (IO) => {
        "<::IO::>"
    };
}


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
                                panic!("\n\nsource target path: {:?}\n\n", e.kind());
                            });

            if let Some(level) = ARGS.depth { // depth
                cf = cf.with_depth(level);
            }

            cf.collect()

        } else {

            panic!("\n\ntarget path {:?}: NotFound\n\n", $path)

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

#[macro_export]
macro_rules! pathbuf {
    ($name:expr) => {
        PathBuf::from($name)
    };
}
pub use pathbuf;

#[macro_export]
macro_rules! makeerr {
    ($msg:literal $(,)?) => ({
        anyhow!($msg)
    });
    ($err:expr $(,)?) => ({
        anyhow!($err)
    });
    ($fmt:expr, $($arg:tt)*) => {
        anyhow!($fmt, $($arg)*)
    };
}
pub use makeerr;
#[macro_export]
macro_rules! reterr {
    ($msg:literal $(,)?) => ({
        return Err(anyhow!($msg))
    });
    ($err:expr $(,)?) => ({
        return Err(anyhow!($err))
    });
    ($fmt:expr, $($arg:tt)*) => {
        return Err(anyhow!($fmt, $($arg)*))
    };
}
pub use reterr;

#[macro_export]
macro_rules! fnerr {
    ($msg:literal $(,)?) => ({
        || format!($msg)
    });
    ($err:expr $(,)?) => ({
        || format!($err)
    });
    ($fmt:expr, $($arg:tt)*) => {
        || format!($fmt, $($arg)*)
    };
}
pub use fnerr;

#[macro_export]
macro_rules! derive_debug_partials {
    (
        $(
            $i:item
        )*
    ) => {
        $(
            #[derive(Debug, PartialEq, Eq)]
            $i
        )*
    };
}
pub use derive_debug_partials;
