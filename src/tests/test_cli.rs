use crate::args::Args;
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
fn test_stratum_options() {
    let stratum_missing_field = Args::generate_default_configuration(
        "tests/stratum_missing_field.toml",
        "config/config_default.toml",
    );

    let expected_1: Result<(Globals, Globals), ArgsError> = Err(ArgsError::ConfigReadError(
        "Failure to read config file: tests/stratum_missing_field.toml".to_owned(),
    ));

    // assert_eq!(stratum_missing_field, expected_1);

    let stratum_enabled = Args::generate_default_configuration(
        "src/tests/stratum_enabled.toml",
        "config/config_default.toml",
    ).unwrap();

    assert_eq!(stratum_enabled.0.sealing_mining.stratum, Some(true));
    // assert!(false)
}
