use dm_parser_sqllog::LogConfig;
use tracing::Level;

#[test]
fn default_log_config_has_expected_values() {
    let cfg = LogConfig::new();
    assert_eq!(cfg.level, Level::DEBUG);
    assert_eq!(cfg.path, "logs".to_string());
}

#[test]
fn setters_update_values() {
    let cfg = LogConfig::new()
        .set_level(Level::TRACE)
        .set_path("/tmp/mylogs");

    assert_eq!(cfg.level, Level::TRACE);
    assert_eq!(cfg.path, "/tmp/mylogs".to_string());
}
