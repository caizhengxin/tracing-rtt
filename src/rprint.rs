#[macro_export]
macro_rules! rprint {
    (=> $terminal:expr, $s:expr) => {
        $crate::print_impl::write_str($terminal, $s);
    };
    (=> $terminal:expr, $($arg:tt)*) => {
        $crate::print_impl::write_fmt($terminal, format_args!($($arg)*));
    };
    ($s:expr) => {
        $crate::print_impl::write_str(0, $s);
    };
    ($($arg:tt)*) => {
        $crate::print_impl::write_fmt(0, format_args!($($arg)*));
    };
}
