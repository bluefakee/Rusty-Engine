pub mod debug;

pub fn startup() {
    debug::try_initialize_logger();
}