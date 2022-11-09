pub use log::{trace, debug, info, warn, error};

pub(crate) fn try_initialize_logger() -> bool {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(
                format_args!("{}[{}][{}] {}",
                    chrono::Local::now().format("[%H:%M:%S]"),
                    record.target(),
                    record.level(),
                    message));
        })
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply()
        .is_ok()
}
