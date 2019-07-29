#[macro_export]
macro_rules! print
{
    ($($arg:tt)*) =>
    {{
        use $crate::io::Write;
        drop($crate::io::stdout().write_fmt(format_args!($($arg)*)));
    }};
}

#[allow_internal_unstable(format_args_nl)]
#[macro_export]
macro_rules! println
{
    ($($arg:tt)*) =>
    {{
        use $crate::io::Write;
        drop($crate::io::stdout().write_fmt(format_args_nl!($($arg)*)));
    }};
}
