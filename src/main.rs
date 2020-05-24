pub mod args;
mod globals;
mod parse_cli;
mod subcommands;
mod tests;

pub use args::Args;

fn main() {
    Args::parse().unwrap();
    // println!("{:#?}", Args::parse());
}
