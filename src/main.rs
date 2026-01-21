use std::fs;

mod cat_file;
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

            let (object_type, size, content) = cat_file::run(&args.hash);

            if mode.show_type {
                print!("{object_type}");
            } else if mode.size {
                print!("{size}");
            } else if mode.print {
                print!("{content}");
            } else if mode.exists {
                todo!();
            } else {
                unreachable!() // clap guarantees exactly one is true
            }
        }
    }
}
