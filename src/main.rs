pub mod args;
mod config;
mod globals;
mod parse_cli;
mod subcommands;
mod tests;

pub use args::Args;
pub use config::get_config;

fn main() {
    Args::parse().unwrap();
    // println!("{:#?}", Args::parse());
}
