#[cfg(test)]
mod ensure_test {
    use digitalvault_minio::client::MinioClient;
    use digitalvault_minio::ensure::MinioEnsure;
    use std::env;
    use tokio;

    #[tokio::test]
    async fn test_minio_ensure_bucket_exists() {
        dotenv::from_filename(".env.test").ok();

        let endpoint = env::var("MINIO_ENDPOINT").expect("MINIO_ENDPOINT not set");
        let access_key = env::var("MINIO_ACCESS_KEY").expect("MINIO_ACCESS_KEY not set");
        let secret_key = env::var("MINIO_SECRET_KEY").expect("MINIO_SECRET_KEY not set");
        let buckets = env::var("MINIO_BUCKETS").expect("MINIO_BUCKETS must be set");
        let mut bucket_iter = buckets.split(',');
        let bucket = bucket_iter.next().unwrap();
        let client = MinioClient::new(&endpoint, &access_key, &secret_key);
        let ensure = MinioEnsure::new(client.s3_client.clone());
        let result = ensure.ensure_bucket_exists(bucket).await;

        match &result {
            Ok(_) => println!("Bucket ensured or already exists."),
            Err(e) => eprintln!("Failed to ensure bucket exists: {:?}", e),
        }

        assert!(result.is_ok(), "Failed to ensure bucket exists: {:?}", result.err());
    }
}
