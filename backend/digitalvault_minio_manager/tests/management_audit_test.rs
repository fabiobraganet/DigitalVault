// tests/management_audit_test.rs
use digitalvault_minio_manager::management_audit::AuditManager;
use digitalvault_minio_manager::client::MinioClient;
use digitalvault_minio_manager::manager_config::config::Config;
use tokio;

#[tokio::test]
async fn test_log_and_get_audit_logs() {
    let config = Config::new();
    let client = MinioClient::new(&config.endpoint, &config.access_key, &config.secret_key);
    let audit_manager = AuditManager::new(client);

    // Registra um evento
    let result = audit_manager.log_event("test_event").await;
    assert!(result.is_ok(), "Failed to log event: {:?}", result.err());

    // Obtém os logs de auditoria
    let result = audit_manager.get_audit_logs(None).await;
    assert!(result.is_ok(), "Failed to get audit logs: {:?}", result.err());

    // Verifica se o evento registrado está nos logs
    let logs = result.unwrap();
    assert!(logs.contains(&"test_event".to_string()));
}
