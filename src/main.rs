mod args;
mod globals;
mod parse_cli;
mod subcommands;

use args::Args;
use parse_cli::ArgsInput;
use structopt::StructOpt;

fn main() {
    println!("{:?}", Args::parse());
}
