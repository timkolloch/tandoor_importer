//! Holds all the command line parameters and the types associated with them
use clap::Parser;
use log::LevelFilter;

/// Struct containing all possible command line parameters.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args{
    /// Override existing properties
    #[arg(short, long="override", help = "When set the program overrides already present properties.")]
    pub override_properties: bool,

    /// Interactive mode
    #[arg(short, long, help = "When set the program asks the user to provide an FDC ID when none was found.")]
    pub interactive: bool,

    /// Log level
    #[arg(short, long, default_value = "info", help = "Sets the log level.",)]
    pub log_level: LogLevel,
}

/// Possible log levels.
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum LogLevel{
    Trace,
    Debug,
    Info,
    Warning,
    Error
}

impl Into<LevelFilter> for LogLevel{
    fn into(self) -> LevelFilter {
        match self {
            LogLevel::Trace => LevelFilter::Trace,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Warning => LevelFilter::Warn,
            LogLevel::Error => LevelFilter::Error
        }
    }
}