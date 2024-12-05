//! Common tools for advent of code problems

pub fn load_env() {
    // load environment variables
    dotenv::dotenv().ok();
    // initialize the logger
    pretty_env_logger::init();
}
