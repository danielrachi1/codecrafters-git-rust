use std::fs;

mod blob_object;
mod cli;
mod ls_tree;
mod mode;
mod object;
mod tree_entry;
mod tree_object;
mod utils;

use crate::{blob_object::BlobObject, object::Object, tree_object::TreeObject};
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

            let blob_object = BlobObject::read_db(&args.hash);

            if mode.show_type {
                print!("{}", blob_object.r#type());
            } else if mode.size {
                print!("{}", blob_object.size());
            } else if mode.print {
                print!("{}", blob_object.0);
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

            let blob_object = BlobObject::new(&content);

            if args.write {
                blob_object.write_db();
                print!("{}", blob_object.hashed_store())
            } else {
                print!("{}", blob_object.hashed_store())
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
