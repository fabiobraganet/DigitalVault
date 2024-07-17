#[cfg(test)]
mod list_test {
    use digitalvault_minio::config::test::ConfigTest;
    use tokio;

    #[tokio::test]
    async fn test_minio_list_buckets_and_objects() {
        let config = ConfigTest::new();
        let list_client = config.list_service();

        // Test listing buckets
        let bucket_list_result = list_client.list_buckets().await;
        assert!(bucket_list_result.is_ok(), "Failed to list buckets: {:?}", bucket_list_result.err());
        let bucket_list = bucket_list_result.unwrap();
        println!("Buckets: {:?}", bucket_list.buckets);

        // Test listing objects in the source bucket
        let object_list_result = list_client.list_objects(&config.source_bucket).await;
        assert!(object_list_result.is_ok(), "Failed to list objects: {:?}", object_list_result.err());
        let object_list = object_list_result.unwrap();
        println!("Objects in bucket {}: {:?}", config.source_bucket, object_list.contents);
    }
}
