pub mod debug;



pub fn startup() {
    if let Err(err) = debug::try_initialize_logger() {
        panic!("Loggerinitialization failed ({}:?)", err);
    }

    debug::info!("Logger initialized");
}