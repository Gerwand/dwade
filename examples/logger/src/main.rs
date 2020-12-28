use dwade::log::console::ConsoleLog;
use dwade::log::file::FileLog;
use dwade::log::{self, Logger};
use dwade::{debug, error, info, warn};

fn main() {
    let mut logger = Logger::new();
    logger.add_output(Box::new(ConsoleLog::new()));

    let log_file = FileLog::new("logger_test.log").expect("Failed to create log file");
    logger.add_output(Box::new(log_file));

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
