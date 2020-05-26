use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "config"]
struct Config;

pub fn get_config() {
    let default = Config::get("config_default.toml").unwrap();
    let config_string = std::str::from_utf8(default.as_ref());
    println!("{:?}", config_string);
}
