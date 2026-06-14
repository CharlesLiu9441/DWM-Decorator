use std::panic;
use std::path::PathBuf;
use tracing::error;
use tracing_subscriber::{Registry, fmt, prelude::*};

fn get_log_path() -> PathBuf {
    let mut exe_path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    exe_path.pop();
    exe_path
}

pub fn init_logger() -> tracing_appender::non_blocking::WorkerGuard {
    let file_appender = tracing_appender::rolling::daily(get_log_path(), "dwm-decorator.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::layer().with_ansi(false).with_writer(non_blocking);
    let stdout_layer = fmt::layer().with_ansi(true);
    Registry::default()
        .with(stdout_layer)
        .with(file_layer)
        .init();
    guard
}

pub fn setup_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        let location = panic_info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "Unknown panic location".to_string());
        let payload = panic_info.payload();
        let message = if let Some(s) = payload.downcast_ref::<&str>() {
            *s
        } else if let Some(s) = payload.downcast_ref::<String>() {
            s.as_str()
        } else {
            "Unknown panic payload"
        };
        error!(target: "panic", position = %location, "Program crashed: {}", message);
    }));
}
