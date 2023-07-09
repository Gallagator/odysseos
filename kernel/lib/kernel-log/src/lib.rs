#![no_std]

#[cfg(debug_assertions)]
pub use kernel_serial::serial_println;

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        kernel_log::serial_println(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! kprintln {
    () => (kernel_log::kprint!("\n"));
    ($fmt:expr) => (kernel_log::kprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (kernel_log::kprint!(
        concat!($fmt, "\n"), $($arg)*));
}
