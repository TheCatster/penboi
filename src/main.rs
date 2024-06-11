use cpu::Cpu;

use anyhow::Result;
use chrono::Local;
use clap::Parser;
use fern::{log_file, Dispatch};
use log::{info, LevelFilter};
use std::{fs, io};

pub mod cpu;

/// A struct to hold the Penboi emulator.
struct Penboi {
    cpu: cpu::Cpu,
}

/// A struct to hold command line arguments.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on.
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// Specify an assembly file to run.
    #[arg(short, long)]
    file: Option<String>,
}

/// The main function of the program.
///
/// # Returns
///
/// A `Result` containing the result of running the program.
///
/// # Examples
///
/// ```
/// let result = penboi::run();
/// assert!(result.is_ok());
/// ```
fn main() -> Result<()> {
    // Parse command line arguments using the Cli struct
    let cli = Cli::parse();

    // Configure the logger using Fern
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(io::stdout())
        .chain(log_file("penboi.log").expect("No permission to write to the current directory."))
        .apply()
        .expect("Failed to dispatch Fern logger!");

    // Log that the logger has been initialised
    info!("Logging initialised.");

    Ok(())
}
