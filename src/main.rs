mod args;
mod globals;
mod parse_cli;

use args::Args;
use parse_cli::ArgsInput;
use structopt::StructOpt;

fn main() {
    let input = ArgsInput::from_args();
    let args: Args = Default::default();

    println!("{:#?}", input);
}
