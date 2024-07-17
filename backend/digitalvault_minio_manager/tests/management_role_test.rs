// tests/management_role_test.rs
use digitalvault_minio_manager::management_role::RoleManager;
use digitalvault_minio_manager::client::MinioClient;
use digitalvault_minio_manager::manager_config::config::Config;
use tokio;

#[tokio::test]
async fn test_create_and_delete_role() {
    let config = Config::new();
    let client = MinioClient::new(&config.endpoint, &config.access_key, &config.secret_key);
    let role_manager = RoleManager::new(client);

    // Cria uma role
    let assume_role_policy_document = r#"{"Version": "2012-10-17", "Statement": [{"Effect": "Allow", "Action": "sts:AssumeRole", "Resource": "*"}]}"#;
    let result = role_manager.create_role("test_role", assume_role_policy_document).await;
    assert!(result.is_ok(), "Failed to create role: {:?}", result.err());

    // Deleta a role
    let result = role_manager.delete_role("test_role").await;
    assert!(result.is_ok(), "Failed to delete role: {:?}", result.err());
}
