mod args;
mod globals;
mod parse_cli;
mod subcommands;

use args::Args;

fn main() {
    println!("{:#?}", Args::parse());
}
