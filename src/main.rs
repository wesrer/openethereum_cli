mod args;
mod globals;
mod parse_cli;
mod subcommands;

use args::Args;
use parse_cli::ArgsInput;
use structopt::StructOpt;

fn main() {
    let input = ArgsInput::from_args();
    let mut args: Args = Default::default();

    // println!("{:#?}", args);

    args.from_cli(input);

    println!("{:#?}", args);
}
