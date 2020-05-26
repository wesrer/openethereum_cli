use crate::args::ArgsError;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "config"]
struct Config;

pub fn get_config(config_name: &str) -> Result<String, ArgsError> {
    match Config::get(config_name) {
        Some(x) => Ok(std::str::from_utf8(x.as_ref()).unwrap().to_owned()),
        None => Err(ArgsError::ConfigReadError(format!(
            "Failure to read config file: {}",
            config_name
        ))),
    }
}
