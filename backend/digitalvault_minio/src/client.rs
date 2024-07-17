use rusoto_core::{HttpClient, Region};
use rusoto_credential::StaticProvider;
use rusoto_s3::S3Client;
use rusoto_iam::IamClient;
use std::sync::Arc;

pub struct MinioClient {
    pub s3_client: Arc<S3Client>,
    pub iam_client: Arc<IamClient>,
}

impl MinioClient {
    pub fn new(endpoint: &str, access_key: &str, secret_key: &str) -> Self {
        let region = Region::Custom {
            name: "minio".to_string(),
            endpoint: endpoint.to_string(),
        };

        let provider = StaticProvider::new_minimal(access_key.to_string(), secret_key.to_string());

        let s3_client = S3Client::new_with(
            HttpClient::new().expect("Failed to create HTTP client"),
            provider.clone(),
            region.clone(),
        );

        let iam_client = IamClient::new_with(
            HttpClient::new().expect("Failed to create HTTP client"),
            provider,
            region,
        );

        MinioClient {
            s3_client: Arc::new(s3_client),
            iam_client: Arc::new(iam_client),
        }
    }
}
