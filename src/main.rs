use log::{info};

mod config;
use config::load_config;

fn main() {
    env_logger::init();
    let conf = load_config();

    info!("Program running with: {:#?}", conf);
}
