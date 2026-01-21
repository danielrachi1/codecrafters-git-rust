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
