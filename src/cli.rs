use std::io::Read;
use std::io::stdin;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    CatFile(CatFileArgs),
    HashObject(HashObjectArgs),
    LsTree(LsTreeArgs),
}

#[derive(Args)]
pub struct CatFileArgs {
    #[command(flatten)]
    pub mode: CatFileMode,

    pub hash: String,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct CatFileMode {
    #[arg(short = 't')]
    pub show_type: bool,

    #[arg(short = 's')]
    pub size: bool,

    #[arg(short = 'p')]
    pub print: bool,

    #[arg(short = 'e')]
    pub exists: bool,
}

#[derive(Args)]
pub struct HashObjectArgs {
    #[arg(group = "source")]
    pub path: Option<PathBuf>,

    #[arg(short = 'w')]
    pub write: bool,

    #[arg(long = "stdin", group = "source")]
    pub stdin: bool,
}

#[derive(Args)]
pub struct LsTreeArgs {
    pub hash: String,

    #[arg(long = "name-only")]
    pub name_only: bool,
}

pub fn read_from_stdin() -> String {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    buffer.trim().to_string()
}
