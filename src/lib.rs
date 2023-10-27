#![cfg_attr(feature = "panic_backtrace_config", feature(panic_backtrace_config))]
use std::path::Path;

use error_handler::install_error_handler;
use panic::set_panic_hook;

mod error_handler;
mod panic;
mod source_span;

pub fn install(workspace_dir: impl AsRef<Path> + Send + Sync + 'static) {
    install_error_handler();
    set_panic_hook(workspace_dir);
}
