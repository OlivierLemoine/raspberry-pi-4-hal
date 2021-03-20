#[macro_export]
macro_rules! eprint {
    ($($args:tt)*) => {
        $crate::uart::_print_internals(format_args!($($args)*))
    };
}

#[macro_export]
macro_rules! eprintln {
    () => {
        $crate::eprint!("\r\n")
    };
    ($($args:tt)*) => {
        $crate::uart::_print_internals(format_args!($($args)*));
        $crate::eprint!("\r\n")
    };
}
