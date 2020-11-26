use std::fmt;
use std::fs::File;
use std::io::{self, Write};

struct Logger {
    level: Level,
    // Let's keep the output and logname separated, to keep the cache smaller.
    outputs: Vec<Box<dyn Write>>,
    lognames: Vec<String>,
}

impl Logger {
    fn new() -> Logger {
        Logger {
            level: Level::Warning,
            outputs: vec![],
            lognames: vec![],
        }
    }

    fn set_level(&mut self, level: Level) {
        self.level = level;
    }

    fn add_output(&mut self, name: String, output: Box<dyn Write>) {
        self.lognames.push(name);
        self.outputs.push(output);
    }

    /*     fn remove_output(&mut self, index: u8) -> (String, Box<dyn Write>) {}
     */
    fn log(&mut self, level: Level, args: fmt::Arguments) {
        if level as i32 <= self.level as i32 {
            return;
        }

        for o in &mut self.outputs {
            write!(o, "{}", args);
        }
    }
}

macro_rules! bit64 {
    ( $b:expr ) => {
        1 << $b
    };
}

#[derive(Clone, Copy)]
enum Level {
    Error = 0,
    Warning = 1,
    Info = 2,
    Debug = 3,
}

// ------------------------------------------------
// -------------------- Macros --------------------
// ------------------------------------------------

// ********** Generic **********
macro_rules! ddlog {
    ( $logger:expr, $level:expr, $fmt: expr $(, $args:expr )* ) => {
        $logger.log($level, format_args!($fmt, $( $args, )*));
    };
}

// ********** Level specific **********
#[cfg(feature = "debug_mode")]
macro_rules! ddlog_debug {
    ( $logger:expr, $( $args:expr ),* ) => {
        $logger.log(Level::Debug, format_args!($( $args, )*));
    };
}
#[cfg(not(feature = "debug_logs"))]
macro_rules! ddlog_debug {
    ( $logger:expr, $( $args:expr ),* ) => {};
}

macro_rules! ddlog_info {
    ( $logger:expr, $( $args:expr ),* ) => {
        $logger.log(Level::Info, format_args!($( $args, )*));
    };
}

macro_rules! ddlog_warn {
    ( $logger:expr, $( $args:expr ),* ) => {
        $logger.log(Level::Warning, format_args!($( $args, )*));
    };
}

macro_rules! ddlog_error {
    ( $logger:expr, $( $args:expr ),* ) => {
        $logger.log(Level::Error, format_args!($( $args, )*));
    };
}

#[cfg(test)]
mod tests {
    // Import all modules from outer scope
    use super::*;

    #[test]
    fn test_logger_new() {
        let logger = Logger::new();
        assert_eq!(logger.level as i32, Level::Warning as i32);
        assert_eq!(logger.outputs.is_empty(), true);
    }
}

fn main() {
    let mut logger = Logger::new();
    logger.set_level(Level::Error);
    logger.outputs.push(Box::new(io::stdout()));
    logger
        .outputs
        .push(Box::new(File::create("Foo.txt").expect("Bug")));

    ddlog!(logger, Level::Error, "\033[32;1m Hello world \033[0m\n");
    ddlog!(logger, Level::Error, "Hello {}, I am {} yo.", "Michał", 27);

    // Try with different levels
    logger.set_level(Level::Debug);
    println!("Set level to Error");
    ddlog!(logger, Level::Error, "Error log");
    ddlog_error!(logger, "Error log wrapper");
    ddlog!(logger, Level::Warning, "Warning log");
    ddlog_warn!(logger, "Warning log wrapper");
    ddlog!(logger, Level::Info, "Info log");
    ddlog_info!(logger, "Info log wrapper");
    ddlog!(logger, Level::Debug, "Debug log");
    ddlog_debug!(logger, "Debug log wrapper");
}
