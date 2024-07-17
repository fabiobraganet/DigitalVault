#[cfg(test)]
mod delete_test {
    use digitalvault_minio::client::MinioClient;
    use digitalvault_minio::delete::MinioDelete;
    use digitalvault_minio::upload::MinioUpload;
    use std::env;

    #[tokio::test]
    async fn test_minio_delete_object() {
        dotenv::from_filename(".env.test").ok();
        
        let endpoint = env::var("MINIO_ENDPOINT").expect("MINIO_ENDPOINT must be set");
        let access_key = env::var("MINIO_ACCESS_KEY").expect("MINIO_ACCESS_KEY must be set");
        let secret_key = env::var("MINIO_SECRET_KEY").expect("MINIO_SECRET_KEY must be set");
        let buckets = env::var("MINIO_BUCKETS").expect("MINIO_BUCKETS must be set");
        let bucket = buckets.split(',').next().unwrap();
        let client = MinioClient::new(&endpoint, &access_key, &secret_key);
        let upload_client = MinioUpload::new(client.s3_client.clone());
        let delete_client = MinioDelete::new(client.s3_client);

        // Primeiro, fazer upload de um objeto para testar a remoção
        let upload_result =
            upload_client.upload(&bucket, "test_delete_key", b"Hello, world!").await;
        assert!(
            upload_result.is_ok(),
            "Failed to upload object: {:?}",
            upload_result.err()
        );

        // Em seguida, deletar o objeto
        let delete_result = delete_client.delete_object(&bucket, "test_delete_key").await;
        assert!(
            delete_result.is_ok(),
            "Failed to delete object: {:?}",
            delete_result.err()
        );
    }
}
