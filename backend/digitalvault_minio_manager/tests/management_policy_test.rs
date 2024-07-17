// tests/management_policy_test.rs
use digitalvault_minio_manager::management_policy::PolicyManager;
use digitalvault_minio_manager::client::MinioClient;
use digitalvault_minio_manager::manager_config::config::Config;
use tokio;

#[tokio::test]
async fn test_create_and_list_policies() {
    let config = Config::new();
    let client = MinioClient::new(&config.endpoint, &config.access_key, &config.secret_key);
    let policy_manager = PolicyManager::new(client);

    // Cria uma policy
    let policy_document = r#"{"Version": "2012-10-17", "Statement": [{"Effect": "Allow", "Action": "s3:GetObject", "Resource": "*"}]}"#;
    let result = policy_manager.create_policy("test_policy", policy_document).await;
    assert!(result.is_ok(), "Failed to create policy: {:?}", result.err());

    // Lista as policies
    let result = policy_manager.list_policies().await;
    assert!(result.is_ok(), "Failed to list policies: {:?}", result.err());

    // Verifica se a policy criada está na lista
    let policies = result.unwrap();
    assert!(policies.contains(&"test_policy".to_string()));
}
