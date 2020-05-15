use serde_derive::Deserialize;
use structopt::StructOpt;
use structopt_toml::StructOptToml;

use crate::globals::Globals;
use crate::subcommands::SubCommands;

#[derive(StructOpt, StructOptToml, Deserialize, Debug, Clone)]
pub struct ArgsInput {
    #[structopt(subcommand)]
    pub subcommands: SubCommands,
    #[structopt(flatten)]
    pub globals: Globals,
}
