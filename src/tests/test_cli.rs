use crate::args::Args;
use crate::args::ArgsError;
use crate::globals::Globals;
use crate::parse_cli::ArgsInput;

#[test]
fn test_override_defaults_with_custom_config() {
    let stratum_enabled = Args::generate_default_configuration(
        "src/tests/stratum_enabled_full.toml",
        "config/config_default.toml",
    )
    .unwrap();

    assert_eq!(stratum_enabled.0.sealing_mining.stratum, true);
    assert_eq!(
        stratum_enabled.0.sealing_mining.stratum_interface,
        Some("some interface".to_owned())
    );
    assert_eq!(stratum_enabled.0.sealing_mining.stratum_port, Some(8007));
    assert_eq!(
        stratum_enabled.0.sealing_mining.stratum_secret,
        Some("Yellow".to_owned())
    );
}

#[test]
fn test_overwrite_custom_config_with_raw_flags() {
    let mut raw: ArgsInput = Default::default();
    let mut resolved: Args = Default::default();

    // These are equivalent to the raw arguments that are going to be accepted
    raw.globals.sealing_mining.stratum_secret = Some("Changed".to_owned());

    // In the default config file, there is a config value "Yellow" for the
    // same field, which it should ignore because of the presence of the raw
    // argument
    let (user_defaults, fallback) = Args::generate_default_configuration(
        "src/tests/stratum_enabled_full.toml",
        "config/config_default.toml",
    )
    .unwrap();

    resolved.from_cli(raw, user_defaults, fallback);

    assert_eq!(resolved.arg_stratum_secret, Some("Changed".to_owned()));
}

#[test]
fn test_not_accepting_min_peers_bigger_than_max_peers() {
    assert!(false);
}
