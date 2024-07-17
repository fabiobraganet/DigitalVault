#[cfg(test)]
mod copy_test {
    use digitalvault_minio::config::test::ConfigTest;
    use tokio;

    #[tokio::test]
    async fn test_minio_copy_object() {
        let config = ConfigTest::new();

        let upload_client = config.upload_service();
        let upload_result = upload_client.upload(&config.source_bucket, "test", b"test data").await;
        
        assert!(upload_result.is_ok(), "Upload failed: {:?}", upload_result.err());

        let copy_client = config.copy_service();
        let result = copy_client.copy_object(&config.source_bucket, "test", &config.destination_bucket, "test_copy").await;
        
        assert!(result.is_ok(), "Copy failed: {:?}", result.err());
    }
}
