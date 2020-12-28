use chrono::prelude::Local;
use std::fmt;
use std::io;

pub trait LogOutput {
    fn log(&mut self, msg: &str) -> io::Result<()>;
    fn format(&mut self, lvl: Level);
    fn name(&self) -> &str;
}

pub struct Logger {
    level: Level,
    outputs: Vec<Box<dyn LogOutput>>,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            level: Level::Warning,
            outputs: vec![],
        }
    }

    pub fn set_level(&mut self, level: Level) {
        self.level = level;
    }

    pub fn add_output(&mut self, log: Box<dyn LogOutput>) {
        self.outputs.push(log);
    }

    /*     fn remove_output(&mut self, index: u8) -> (String, Box<dyn Write>) {}
     */
    pub fn log(&mut self, level: Level, args: fmt::Arguments) {
        if level as i32 > self.level as i32 {
            return;
        }

        if self.outputs.is_empty() {
            return;
        }

        let now = Local::now();
        let string = format!(
            "[{}.{}] {}",
            now.format("%H:%M:%S"),
            now.timestamp_subsec_millis(),
            args
        );
        for out in self.outputs.iter_mut() {
            out.format(level);
            match out.log(&string[..]) {
                Ok(_) => {}
                Err(err) => eprintln!(
                    "Failed to write to the output: {}.\n Error: {}",
                    out.name(),
                    err
                ),
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

pub mod console {
    use super::*;
    use std::io::{self, Write};

    pub struct ConsoleLog {
        stdout: io::Stdout,
    }

    impl ConsoleLog {
        pub fn new() -> ConsoleLog {
            ConsoleLog {
                stdout: io::stdout(),
            }
        }
    }

    impl LogOutput for ConsoleLog {
        fn log(&mut self, msg: &str) -> io::Result<()> {
            writeln!(self.stdout, "{}", msg)
        }

        fn name(&self) -> &str {
            "Terminal log"
        }

        fn format(&mut self, _lvl: Level) {}
    }
}

pub mod file {
    use super::*;
    use std::fs::File;
    use std::io::{self, Write};

    pub struct FileLog {
        file: File,
        name: String,
    }

    impl FileLog {
        pub fn new(filename: &str) -> io::Result<FileLog> {
            let file = File::create(filename)?;
            Ok(FileLog {
                file,
                name: String::from(filename),
            })
        }
    }

    impl LogOutput for FileLog {
        fn log(&mut self, msg: &str) -> io::Result<()> {
            writeln!(self.file, "{}", msg)
        }

        fn name(&self) -> &str {
            &self.name[..]
        }

        fn format(&mut self, _lvl: Level) {}
    }
}
