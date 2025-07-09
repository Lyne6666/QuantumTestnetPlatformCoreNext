// src/main.rs
/*
 * Main executable for QuantumTestnetPlatformCoreNext
 */

use clap::Parser;
use quantumtestnetplatformcorenext::{Result, run};

#[derive(Parser)]
#[command(version, about = "QuantumTestnetPlatformCoreNext - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
