// src/main.rs
/*
 * Main executable for MockConnector
 */

use clap::Parser;
use mockconnector::{Result, run};

#[derive(Parser)]
#[command(version, about = "MockConnector - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Input file path
    #[arg(short, long)]
    input: Option<String>,
    
    /// Output file path
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose, args.input, args.output)
}
