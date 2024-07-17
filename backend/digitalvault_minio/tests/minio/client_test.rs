#[cfg(test)]
mod client_test {
    use std::env;
    use std::sync::Arc;
    use digitalvault_minio::client::MinioClient;

    #[test]
    fn test_minio_client_new() {
        dotenv::from_filename(".env.test").ok();

        let endpoint = env::var("MINIO_ENDPOINT").expect("MINIO_ENDPOINT not set");
        let access_key = env::var("MINIO_ACCESS_KEY").expect("MINIO_ACCESS_KEY not set");
        let secret_key = env::var("MINIO_SECRET_KEY").expect("MINIO_SECRET_KEY not set");

        let client = MinioClient::new(&endpoint, &access_key, &secret_key);
        assert!(Arc::strong_count(&client.s3_client) > 0);
    }
}

