use rusoto_core::{HttpClient, Region};
use rusoto_credential::StaticProvider;
use rusoto_s3::{S3Client, S3, ListObjectVersionsRequest, ListObjectVersionsOutput, PutBucketVersioningRequest, VersioningConfiguration};
use std::sync::Arc;
use std::error::Error;

pub struct MinioVersioning {
    client: Arc<S3Client>,
}

impl MinioVersioning {
    pub fn new(endpoint: &str, access_key: &str, secret_key: &str) -> Self {
        let region = Region::Custom {
            name: "minio".to_string(),
            endpoint: endpoint.to_string(),
        };

        let provider = StaticProvider::new_minimal(access_key.to_string(), secret_key.to_string());

        let client = S3Client::new_with(
            HttpClient::new().expect("Failed to create HTTP client"),
            provider,
            region,
        );

        MinioVersioning {
            client: Arc::new(client),
        }
    }

    pub async fn enable_versioning(&self, bucket: &str) -> Result<(), Box<dyn Error>> {
        let versioning_configuration = VersioningConfiguration {
            status: Some("Enabled".to_string()),
            ..Default::default()
        };

        let put_versioning_request = PutBucketVersioningRequest {
            bucket: bucket.to_string(),
            versioning_configuration,
            ..Default::default()
        };

        self.client.put_bucket_versioning(put_versioning_request).await?;
        Ok(())
    }

    pub async fn list_object_versions(&self, bucket: &str, prefix: &str) -> Result<ListObjectVersionsOutput, Box<dyn Error>> {
        let list_versions_request = ListObjectVersionsRequest {
            bucket: bucket.to_string(),
            prefix: Some(prefix.to_string()),
            ..Default::default()
        };

        let result = self.client.list_object_versions(list_versions_request).await?;
        Ok(result)
    }
}
