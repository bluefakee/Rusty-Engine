pub mod debug;

pub fn startup() {
    if let Err(err) = debug::try_initialize_logger() {
        debug::error!("Loggerinitialization failed ({}:?)", err);
        return;
    }

    debug::info!("Logger initialized");
}