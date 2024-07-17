// tests/management_group_test.rs
use digitalvault_minio_manager::management_group::GroupManager;
use digitalvault_minio_manager::client::MinioClient;
use digitalvault_minio_manager::manager_config::config::Config;
use tokio;

#[tokio::test]
async fn test_create_and_list_groups() {
    let config = Config::new();
    let client = MinioClient::new(&config.endpoint, &config.access_key, &config.secret_key);
    let group_manager = GroupManager::new(client);

    // Cria um grupo
    let result = group_manager.create_group("test_group").await;
    assert!(result.is_ok(), "Failed to create group: {:?}", result.err());

    // Lista os grupos
    let result = group_manager.list_groups().await;
    assert!(result.is_ok(), "Failed to list groups: {:?}", result.err());

    // Verifica se o grupo criado está na lista
    let groups = result.unwrap();
    assert!(groups.contains(&"test_group".to_string()));
}

#[tokio::test]
async fn test_add_and_remove_user_from_group() {
    let config = Config::new();
    let client = MinioClient::new(&config.endpoint, &config.access_key, &config.secret_key);
    let group_manager = GroupManager::new(client);

    // Adiciona um usuário ao grupo
    let result = group_manager.add_user_to_group("test_user", "test_group").await;
    assert!(result.is_ok(), "Failed to add user to group: {:?}", result.err());

    // Remove o usuário do grupo
    let result = group_manager.remove_user_from_group("test_user", "test_group").await;
    assert!(result.is_ok(), "Failed to remove user from group: {:?}", result.err());
}

#[tokio::test]
async fn test_list_group_users() {
    let config = Config::new();
    let client = MinioClient::new(&config.endpoint, &config.access_key, &config.secret_key);
    let group_manager = GroupManager::new(client);

    // Lista os usuários do grupo
    let result = group_manager.list_group_users("test_group").await;
    assert!(result.is_ok(), "Failed to list group users: {:?}", result.err());

    // Verifica se o usuário está na lista
    let users = result.unwrap();
    assert!(users.contains(&"test_user".to_string()));
}
