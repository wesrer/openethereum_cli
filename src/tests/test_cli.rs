use crate::args::Args;
use crate::parse_cli::ArgsInput;
use crate::args::ArgsError;
use crate::globals::Globals;

#[test]
fn test_reading_invalid_configurations() {
    let config_invalid_1 = Args::generate_default_configuration(
        "tests/invalid_config_1.toml",
        "config/config_default.toml",
    );

    let expected_1 = Err(ArgsError::ConfigReadError(
        "Failure to read config file: tests/invalid_config_1.toml".to_owned(),
    ));

    let config_invalid_2 = Args::generate_default_configuration(
        "tests/invalid_config_2.toml",
        "config/config_default.toml",
    );

    let expected_2 = Err(ArgsError::ConfigReadError(
        "Failure to read config file: tests/invalid_config_2.toml".to_owned(),
    ));

    let config_invalid_3 = Args::generate_default_configuration(
        "tests/invalid_config_3.toml",
        "config/config_default.toml",
    );

    let expected_3 = Err(ArgsError::ConfigReadError(
        "Failure to read config file: tests/invalid_config_3.toml".to_owned(),
    ));

    let config_invalid_4 = Args::generate_default_configuration(
        "tests/invalid_config_4.toml",
        "config/config_default.toml",
    );

    let expected_4: Result<(Globals, Globals), ArgsError> = Err(ArgsError::ConfigReadError(
        "Failure to read config file: tests/invalid_config_4.toml".to_owned(),
    ));

    assert_eq!(config_invalid_1, expected_1);
    assert_eq!(config_invalid_2, expected_2);
    assert_eq!(config_invalid_3, expected_3);
    assert_eq!(config_invalid_4, expected_4);
}

#[test]
fn test_override_defaults_with_custom_config() {
    let stratum_enabled = Args::generate_default_configuration(
        "src/tests/stratum_enabled_full.toml",
        "config/config_default.toml",
    ).unwrap();

    assert_eq!(stratum_enabled.0.sealing_mining.stratum, Some(true));
    assert_eq!(stratum_enabled.0.sealing_mining.stratum_interface, Some("some interface".to_owned()));
    assert_eq!(stratum_enabled.0.sealing_mining.stratum_port, Some(8007));
    assert_eq!(stratum_enabled.0.sealing_mining.stratum_secret, Some("Yellow".to_owned()));
}

#[test]
fn test_overwrite_custom_config_with_raw_flags() {
    let mut raw: ArgsInput = Default::default();
    let mut resolved: Args = Default::default();

    raw.globals.sealing_mining.stratum_secret = Some("Changed".to_owned());

    let (user_defaults, fallback) = Args::generate_default_configuration(
        "src/tests/stratum_enabled_full.toml",
        "config/config_default.toml",
    ).unwrap();

    resolved.from_cli(raw, user_defaults, fallback);

    assert_eq!(resolved.arg_stratum_secret, Some("Changed".to_owned()));
}
