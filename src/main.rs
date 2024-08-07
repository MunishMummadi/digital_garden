mod cli;
mod note;
mod storage;
mod search;
mod editor;
mod visualizer;
mod sync;

use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::from_args();
    cli.run()
}