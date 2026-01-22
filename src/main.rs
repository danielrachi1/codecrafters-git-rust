use std::fs;

mod cat_file;
mod cli;
mod hash_object;
mod ls_tree;
mod mode;
mod tree_entry;
mod tree_object;

use clap::Parser;
use cli::{Cli, Commands};

use crate::tree_object::TreeObject;

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
        Commands::HashObject(args) => {
            let content = if let Some(path) = &args.path {
                fs::read_to_string(path).unwrap()
            } else if args.stdin {
                cli::read_from_stdin()
            } else {
                panic!("Must provide file path or --stdin")
            };

            let store = hash_object::store(&content);
            let hash = hash_object::hash(&store);

            if args.write {
                let compressed_data = hash_object::compress(&store);
                hash_object::write(&hash, compressed_data);
                print!("{hash}")
            } else {
                print!("{hash}")
            }
        }
        Commands::LsTree(args) => {
            let hash = &args.hash;
            let tree_object = TreeObject::from_hash(hash);

            if args.name_only {
                let names = tree_object.get_names();
                for name in names {
                    println!("{name}")
                }
            } else {
                for entry in tree_object.0 {
                    println!("{entry}")
                }
            }
        }
    }
}
