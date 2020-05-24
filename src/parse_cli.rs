use structopt::StructOpt;

use crate::globals::Globals;
use crate::subcommands::SubCommands;

#[derive(StructOpt, Debug, Clone, Default)]
pub struct ArgsInput {
    #[structopt(subcommand)]
    pub subcommands: Option<SubCommands>,
    #[structopt(flatten)]
    pub globals: Globals,
}
