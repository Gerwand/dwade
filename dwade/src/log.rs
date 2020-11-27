use chrono::prelude::Local;
use std::fmt;
use std::io::Write;

pub struct Logger {
    level: Level,
    // Let's keep the output and logname separated, to keep the cache smaller.
    outputs: Vec<Box<dyn Write>>,
    lognames: Vec<String>,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            level: Level::Warning,
            outputs: vec![],
            lognames: vec![],
        }
    }

    pub fn set_level(&mut self, level: Level) {
        self.level = level;
    }

    pub fn add_output(&mut self, name: String, output: Box<dyn Write>) {
        self.lognames.push(name);
        self.outputs.push(output);
    }

    /*     fn remove_output(&mut self, index: u8) -> (String, Box<dyn Write>) {}
     */
    pub fn log(&mut self, level: Level, args: fmt::Arguments) {
        if level as i32 > self.level as i32 {
            return;
        }

        let now = Local::now();
        for (out, name) in self.outputs.iter_mut().zip(self.lognames.iter()) {
            match writeln!(
                out,
                "[{}.{}] {}",
                now.format("%H:%M:%S"),
                now.timestamp_subsec_millis(),
                args
            ) {
                Ok(_) => {}
                Err(err) => eprintln!("Failed to write to the output: {}.\n Error: {}", name, err),
            };
        }
    }
}

#[derive(Clone, Copy)]
pub enum Level {
    Error = 0,
    Warning = 1,
    Info = 2,
    Debug = 3,
}

// ------------------------------------------------
// -------------------- Macros --------------------
// ------------------------------------------------

// ********** Level specific **********
#[macro_export]
#[cfg(feature = "debug_logs")]
macro_rules! debug {
	( $logger:expr, $fmt:expr $(, $args:expr )* ) => {
	    $logger.log($crate::log::Level::Debug, format_args!($fmt $(, $args)*));
	};
}

#[macro_export]
#[cfg(not(feature = "debug_logs"))]
macro_rules! debug {
    ( $( $_:expr ),*) => {};
}

#[macro_export]
macro_rules! info {
	( $logger:expr, $fmt:expr $(, $args:expr )* ) => {
	    $logger.log($crate::log::Level::Info, format_args!($fmt $(, $args )*));
	};
}

#[macro_export]
macro_rules! warn {
	( $logger:expr, $fmt:expr $(, $args:expr )* ) => {
	    $logger.log($crate::log::Level::Warning, format_args!($fmt $(, $args )*));
    };
}

#[macro_export]
macro_rules! error {
    ( $logger:expr, $fmt:expr $(, $args:expr )* ) => {
        $logger.log($crate::log::Level::Error, format_args!($fmt $(, $args )*));
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
