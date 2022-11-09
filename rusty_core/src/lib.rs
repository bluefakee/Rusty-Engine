pub mod debug;

pub fn startup() {
    if !debug::try_initialize_logger() {
        debug::error!("Failed initializing the Logger");
        return;
    }

    debug::info!("Initialized Logger successfully");
}