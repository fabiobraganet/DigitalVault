// tests/configtest.rs
use digitalvault_minio_manager::manager_config::config::Config;

#[test]
fn test_config() {
    let config = Config::new();
    assert!(!config.endpoint.is_empty());
    assert!(!config.access_key.is_empty());
    assert!(!config.secret_key.is_empty());
}
