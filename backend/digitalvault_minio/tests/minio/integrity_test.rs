#[cfg(test)]
mod integrity_test {
    use digitalvault_minio::client::MinioClient;
    use digitalvault_minio::integrity::MinioIntegrity;
    use digitalvault_minio::upload::MinioUpload;
    use std::env;
    use tokio;

    #[tokio::test]
    async fn test_minio_integrity_check() {
        dotenv::from_filename(".env.test").ok();

        let endpoint = env::var("MINIO_ENDPOINT").expect("MINIO_ENDPOINT must be set");
        let access_key = env::var("MINIO_ACCESS_KEY").expect("MINIO_ACCESS_KEY must be set");
        let secret_key = env::var("MINIO_SECRET_KEY").expect("MINIO_SECRET_KEY must be set");
        let buckets = env::var("MINIO_BUCKETS").expect("MINIO_BUCKETS must be set");
        let bucket = buckets.split(',').next().unwrap();
        let minioclient = MinioClient::new(&endpoint, &access_key, &secret_key);
        let upload_client = MinioUpload::new(minioclient.s3_client.clone());
        let upload_result = upload_client.upload(bucket, "test_integrity", b"test data for integrity check").await;
        assert!(upload_result.is_ok());

        let integrity_client = MinioIntegrity::new(minioclient.s3_client.clone());
        let etag_result = integrity_client.get_etag(bucket, "test_integrity").await;
        assert!(etag_result.is_ok());

        let etag = etag_result.unwrap();
        let integrity_result = integrity_client.check_integrity(bucket, "test_integrity", &etag).await;
        assert!(integrity_result.is_ok());
        assert!(integrity_result.unwrap());
    }
}
