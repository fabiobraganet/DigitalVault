use rusoto_s3::{CopyObjectRequest, S3Client, S3};
use std::error::Error;
use std::sync::Arc;

pub struct MinioCopy {
    client: Arc<S3Client>,
}

impl MinioCopy {
    pub fn new(client: Arc<S3Client>) -> Self {
        MinioCopy { client }
    }

    pub async fn copy_object(&self, source_bucket: &str, source_key: &str, destination_bucket: &str, destination_key: &str) -> Result<(), Box<dyn Error>> {
        let copy_source = format!("{}/{}", source_bucket, source_key);
        let copy_request = CopyObjectRequest {
            bucket: destination_bucket.to_string(),
            key: destination_key.to_string(),
            copy_source,
            ..Default::default()
        };

        match self.client.copy_object(copy_request).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
