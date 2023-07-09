use std::env;

fn main() {
    if std::env::var_os("CARGO_CFG_TEST").is_some() {
        kernel_build::build_main();
    }
}
