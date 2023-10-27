use backtrace::Backtrace;
use std::{fmt::Write, panic::Location, path::Path};
use thiserror::Error;

use miette::{Diagnostic, Report};

use crate::source_span::TestSourceSpan;

#[cfg(feature = "panic_backtrace_config")]
fn is_backtrace_enabled() -> bool {
    !matches!(
        std::panic::get_backtrace_style(),
        Some(BacktraceStyle::Off) | None
    )
}

#[cfg(not(feature = "panic_backtrace_config"))]
fn is_backtrace_enabled() -> bool {
    matches!(std::env::var("RUST_BACKTRACE"), Ok(var) if !var.is_empty() && var != "0")
}

/// Custom panic handler
/// This code is heavily based on miette::set_panic_hook (https://github.com/zkat/miette/blob/main/src/panic.rs)
pub(crate) fn set_panic_hook(workspace_dir: impl AsRef<Path> + Send + Sync + 'static) {
    std::panic::set_hook(Box::new(move |info| {
        let payload = info.payload();
        let message = if let Some(msg) = payload.downcast_ref::<&str>() {
            msg.to_string()
        } else if let Some(msg) = payload.downcast_ref::<String>() {
            msg.to_string()
        } else {
            "Something went wrong".to_string()
        };
        let panic = Panic(message);
        let mut report: Report = if let Some(loc) = info.location() {
            PanicLocation::new(panic, loc).into()
        } else {
            panic.into()
        };
        if let Ok(Some(mut src_span)) = TestSourceSpan::from_backtrace(&workspace_dir) {
            src_span.add_related(report);
            report = src_span.into();
        }
        eprintln!("{:?}", report);
    }));
}

#[derive(Debug, Error, Diagnostic)]
#[error("Panic at {}:{}:{}", filename, line, col)]
struct PanicLocation {
    #[source]
    #[diagnostic_source]
    panic: Panic,
    filename: String,
    line: u32,
    col: u32,
}

impl PanicLocation {
    fn new(panic: Panic, location: &Location) -> Self {
        Self {
            panic,
            filename: location.file().to_string(),
            line: location.line(),
            col: location.column(),
        }
    }
}

#[derive(Debug, Error, Diagnostic)]
#[error("{0}{}", Panic::backtrace())]
struct Panic(String);

impl Panic {
    /* Derived from https://docs.rs/miette/latest/src/miette/panic.rs.html#7-26 */
    fn backtrace() -> String {
        if !is_backtrace_enabled() {
            return String::new();
        }

        const HEX_WIDTH: usize = std::mem::size_of::<usize>() + 2;
        // Padding for next lines after frame's address
        const NEXT_SYMBOL_PADDING: usize = HEX_WIDTH + 6;
        let mut backtrace = String::new();
        let trace = Backtrace::new();
        let frames = backtrace_ext::short_frames_strict(&trace).enumerate();
        for (idx, (frame, sub_frames)) in frames {
            let ip = frame.ip();
            let _ = write!(backtrace, "\n{:4}: {:2$?}", idx, ip, HEX_WIDTH);

            let symbols = frame.symbols();
            if symbols.is_empty() {
                let _ = write!(backtrace, " - <unresolved>");
                continue;
            }

            for (idx, symbol) in symbols[sub_frames].iter().enumerate() {
                // Print symbols from this address,
                // if there are several addresses
                // we need to put it on next line
                if idx != 0 {
                    let _ = write!(backtrace, "\n{:1$}", "", NEXT_SYMBOL_PADDING);
                }

                if let Some(name) = symbol.name() {
                    let _ = write!(backtrace, " - {}", name);
                } else {
                    let _ = write!(backtrace, " - <unknown>");
                }

                // See if there is debug information with file name and line
                if let (Some(file), Some(line)) = (symbol.filename(), symbol.lineno()) {
                    let _ = write!(
                        backtrace,
                        "\n{:3$}at {}:{}",
                        "",
                        file.display(),
                        line,
                        NEXT_SYMBOL_PADDING
                    );
                }
            }
        }
        backtrace
    }
}
