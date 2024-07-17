#[cfg(test)]
mod download_test {
    use digitalvault_minio::config::test::ConfigTest;
    use tokio;


    #[tokio::test]
    async fn test_minio_download_object_success() {
        let config = ConfigTest::new();

        let upload_service = config.upload_service();
        let download_service = config.download_service();
        let delete_service = config.delete_service();
        let key = "testfile.txt";
        let data = b"Hello, world!";

        // Upload 
        let upload_result = upload_service.upload(&config.source_bucket, key, data).await;
        assert!(upload_result.is_ok(), "Upload failed: {:?}", upload_result.err());

        // Download 
        let download_result = download_service.download(&config.source_bucket, key).await;
        match &download_result {
            Ok(downloaded_data) => assert_eq!(downloaded_data, data),
            Err(e) => panic!("Download failed: {:?}", e),
        }

        // Clean
        let delete_result = delete_service.delete_object(&config.source_bucket, key).await;
        assert!(delete_result.is_ok(), "Delete failed: {:?}", delete_result.err());
    }    

    #[tokio::test]
    async fn test_minio_download_nonexistent_object() {
        let config = ConfigTest::new();

        let download_service = config.download_service();
        let key = "nonexistentfile.txt";


        let download_result = download_service.download(&config.source_bucket, key).await;

        assert!(download_result.is_err(), "Expected download to fail for nonexistent file");
    }

    // #[tokio::test]
    // async fn test_minio_download_restricted_permissions() {
    //     let config = ConfigTest::new();

    //     let upload_service = config.upload_service();
    //     let download_service = config.download_service();
    //     let delete_service = config.delete_service();
    //     let key = "restrictedfile.txt";
    //     let data = b"Hello, restricted world!";

    //     // Upload the file with restricted permissions
    //     // Assumption: upload_service has a method to set permissions
    //     // let upload_result = upload_service.upload_with_permissions(&config.source_bucket, key, data, "restricted").await;
    //     // assert!(upload_result.is_ok(), "Upload failed: {:?}", upload_result.err());

    //     // Attempt to download the file with restricted permissions
    //     let download_result = download_service.download(&config.source_bucket, key).await;
    //     assert!(download_result.is_err(), "Expected download to fail due to restricted permissions");

    //     // Clean up
    //     let delete_result = delete_service.delete_object(&config.source_bucket, key).await;
    //     assert!(delete_result.is_ok(), "Delete failed: {:?}", delete_result.err());
    // }

    #[tokio::test]
    async fn test_minio_download_empty_file() {
        let config = ConfigTest::new();

        let upload_service = config.upload_service();
        let download_service = config.download_service();
        let delete_service = config.delete_service();
        let key = "emptyfile.txt";
        let data = b"";

        // Upload the empty file
        let upload_result = upload_service.upload(&config.source_bucket, key, data).await;
        assert!(upload_result.is_ok(), "Upload failed: {:?}", upload_result.err());

        // Download the empty file
        let download_result = download_service.download(&config.source_bucket, key).await;
        match &download_result {
            Ok(downloaded_data) => assert_eq!(downloaded_data, data),
            Err(e) => panic!("Download failed: {:?}", e),
        }

        // Clean up
        let delete_result = delete_service.delete_object(&config.source_bucket, key).await;
        assert!(delete_result.is_ok(), "Delete failed: {:?}", delete_result.err());
    }

    #[tokio::test]
    async fn test_minio_download_large_file() {
        let config = ConfigTest::new();

        let upload_service = config.upload_service();
        let download_service = config.download_service();
        let delete_service = config.delete_service();
        let key = "largefile.txt";
        let data = vec![0u8; 10 * 1024 * 1024]; // 10 MB file

        // Upload the large file
        let upload_result = upload_service.upload(&config.source_bucket, key, &data).await;
        assert!(upload_result.is_ok(), "Upload failed: {:?}", upload_result.err());

        // Download the large file
        let download_result = download_service.download(&config.source_bucket, key).await;
        match &download_result {
            Ok(downloaded_data) => assert_eq!(downloaded_data, &data[..]),
            Err(e) => panic!("Download failed: {:?}", e),
        }

        // Clean up
        let delete_result = delete_service.delete_object(&config.source_bucket, key).await;
        assert!(delete_result.is_ok(), "Delete failed: {:?}", delete_result.err());
    }

}
