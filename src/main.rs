mod parse_cli;

use parse_cli::ArgsInput;
use structopt::StructOpt;

fn main() {
    let input = ArgsInput::from_args();

    println!("{:?}", input);
}
