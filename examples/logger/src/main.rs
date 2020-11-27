use dwade::log::{self, Logger};
use dwade::{debug, error, info, warn};
use std::fs::File;

use std::io;

fn main() {
    let mut logger = Logger::new();
    logger.add_output(String::from("stdout"), Box::new(io::stdout()));

    let log_file =
        Box::new(File::create("logger_test.log").expect("Failed to create logger file."));
    logger.add_output(String::from("file"), log_file);

    println!("Default logging lvl");
    error!(logger, "## Error log!");
    warn!(logger, "## Warning log!");
    info!(logger, "## Info log!");
    debug!(logger, "## Debug log!");

    println!("Error logging lvl");
    logger.set_level(log::Level::Error);
    error!(logger, "## Error log!");
    warn!(logger, "## Warning log!");
    info!(logger, "## Info log!");
    debug!(logger, "## Debug log!");

    println!("Info logging lvl");
    logger.set_level(log::Level::Info);
    error!(logger, "## Error log!");
    warn!(logger, "## Warning log!");
    info!(logger, "## Info log!");
    debug!(logger, "## Debug log!");

    println!("Debug logging lvl");
    logger.set_level(log::Level::Debug);
    error!(logger, "## Error log!");
    warn!(logger, "## Warning log!");
    info!(logger, "## Info log!");
    debug!(logger, "## Debug log!");
}
