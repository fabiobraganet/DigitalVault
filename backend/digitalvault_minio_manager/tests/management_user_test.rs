// tests/management_user_test.rs
#[cfg(test)]
mod management_user_test {
    use digitalvault_minio_manager::manager_config::config::Config;

    #[tokio::test]
    async fn test_create_and_list_users() {
        let config = Config::new();
        let user_service = config.user_service();

        let create_result = user_service.create_user("test_user").await;
        assert!(create_result.is_ok(), "Failed to create user: {:?}", create_result.err());

        let list_result = user_service.list_users().await;
        assert!(list_result.is_ok(), "Failed to list users: {:?}", list_result.err());
    }
}
