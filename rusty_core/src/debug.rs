pub use log::{trace, debug, info, warn, error};

pub(crate) fn try_initialize_logger() -> Result<(), log::SetLoggerError> {
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
}
