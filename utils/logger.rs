use log::{info, warn, error};

pub fn init() {
    env_logger::builder().init();
}

pub fn log_info(message: &str) {
    info!("{}", message);
}

pub fn log_warning(message: &str) {
    warn!("{}", message);
}

pub fn log_error(message: &str) {
    error!("{}", message);
}
