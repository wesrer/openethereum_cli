use structopt::StructOpt;

use crate::globals::Globals;
use crate::subcommands::SubCommands;

#[derive(StructOpt, Debug, Clone)]
pub struct ArgsInput {
    #[structopt(subcommand)]
    pub subcommands: SubCommands,
    #[structopt(flatten)]
    pub globals: Globals,
}
