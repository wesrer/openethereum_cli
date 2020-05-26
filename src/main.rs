pub mod args;
mod globals;
mod parse_cli;
mod subcommands;
mod tests;
mod config;

pub use args::Args;

fn main() {

    crate::config::print_default();
    // Args::parse().unwrap();
    // println!("{:#?}", Args::parse());
}
