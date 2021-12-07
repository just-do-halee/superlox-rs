// Copyright 2021 Hwakyeom Kim(=just-do-halee)

#[macro_export]
macro_rules! name {
    (IO) => {
        "<::IO::>"
    };
}

#[macro_export]
macro_rules! nl {
    () => {
        '\n'
    };
    (2) => {
        "\n\n"
    };
    (3) => {
        "\n\n\n"
    };
    (*$count:expr) => {
        "\n".repeat($count)
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

#[macro_export]
macro_rules! pathbuf {
    ($name:expr) => {
        PathBuf::from($name)
    };
}

#[macro_export]
macro_rules! makeerr {
    ($err:expr $(,)?) => ({
        anyhow!($err)
    });
    ($fmt:expr, $($arg:tt)*) => {
        anyhow!($fmt, $($arg)*)
    };
}

#[macro_export]
macro_rules! reterr {
    ($err:expr $(,)?) => ({
        return Err(makeerr!($err))
    });
    ($fmt:expr, $($arg:tt)*) => {
        return Err(makeerr!($fmt, $($arg)*))
    };
}
#[macro_export]
macro_rules! _reterr {
    ($err:expr $(,)?) => {{
        return Err($err);
    }};
}

#[macro_export]
macro_rules! fnerr {
    ($err:expr $(,)?) => ({
        || format!($err)
    });
    ($fmt:expr, $($arg:tt)*) => {
        || format!($fmt, $($arg)*)
    };
}

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
