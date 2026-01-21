use std::fs;

mod cli;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
            println!("Initialized git directory")
        }
        Commands::CatFile(args) => {
            let mode = &args.mode;
            let hash = &args.hash;

            if mode.show_type {
                println!("-t: {hash}");
            } else if mode.size {
                println!("-s: {hash}");
            } else if mode.print {
                println!("-p: {hash}");
            } else if mode.exists {
                println!("-e: {hash}");
            } else {
                unreachable!() // clap guarantees exactly one is true
            }
        }
    }
}
