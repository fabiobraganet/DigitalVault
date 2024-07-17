use rusoto_core::RusotoError;
use rusoto_s3::{HeadObjectRequest, S3Client, S3};
use std::error::Error;
use std::sync::Arc;

pub struct MinioMetadata {
    client: Arc<S3Client>,
}

impl MinioMetadata {
    pub fn new(client: Arc<S3Client>) -> Self {
        MinioMetadata { client }
    }

    pub async fn get_metadata(&self, bucket: &str, key: &str) -> Result<Option<std::collections::HashMap<String, String>>, Box<dyn Error>> {
        let head_request = HeadObjectRequest {
            bucket: bucket.to_string(),
            key: key.to_string(),
            ..Default::default()
        };

        match self.client.head_object(head_request).await {
            Ok(output) => Ok(output.metadata),
            Err(RusotoError::Service(e)) if e.to_string().contains("NotFound") => Ok(None),
            Err(e) => {
                eprintln!("Failed to get metadata: {:?}", e);
                Err(Box::new(e))
            }
        }
    }
}
