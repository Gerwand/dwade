#[macro_use]
extern crate dwade;
use dwade::log::{self, Logger};
use std::fs::File;

use std::io;

fn main() {
    let mut logger = Logger::new();
    logger.add_output(String::from("stdout"), Box::new(io::stdout()));
    logger.add_output(
        String::from("file"),
        Box::new(File::create("logger_test.log").expect("Failed to create logger file.")),
    );

    println!("Default logging lvl");
    ddlog_error!(logger, "## Error log!");
    ddlog_warn!(logger, "## Warning log!");
    ddlog_info!(logger, "## Info log!");
    ddlog_debug!(logger, "## Debug log!");

    println!("Error logging lvl");
    logger.set_level(log::Level::Error);
    ddlog_error!(logger, "## Error log!");
    ddlog_warn!(logger, "## Warning log!");
    ddlog_info!(logger, "## Info log!");
    ddlog_debug!(logger, "## Debug log!");

    println!("Info logging lvl");
    logger.set_level(log::Level::Info);
    ddlog_error!(logger, "## Error log!");
    ddlog_warn!(logger, "## Warning log!");
    ddlog_info!(logger, "## Info log!");
    ddlog_debug!(logger, "## Debug log!");

    println!("Debug logging lvl");
    logger.set_level(log::Level::Debug);
    ddlog_error!(logger, "## Error log!");
    ddlog_warn!(logger, "## Warning log!");
    ddlog_info!(logger, "## Info log!");
    ddlog_debug!(logger, "## Debug log!");
}
