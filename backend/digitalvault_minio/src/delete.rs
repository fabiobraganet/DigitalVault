use rusoto_core::RusotoError;
use rusoto_s3::{DeleteObjectRequest, S3Client, S3};
use std::sync::Arc;
use std::error::Error;

pub struct MinioDelete {
    client: Arc<S3Client>,
}

impl MinioDelete {
    pub fn new(client: Arc<S3Client>) -> Self {
        MinioDelete { client }
    }

    pub async fn delete_object(&self, bucket: &str, key: &str) -> Result<(), Box<dyn Error>> {
        let delete_req = DeleteObjectRequest {
            bucket: bucket.to_string(),
            key: key.to_string(),
            bypass_governance_retention: None,
            expected_bucket_owner: None,
            mfa: None,
            request_payer: None,
            version_id: None,
        };

        match self.client.delete_object(delete_req).await {
            Ok(_) => Ok(()),
            Err(RusotoError::Service(e)) => Err(Box::new(e)),
            Err(e) => Err(Box::new(e)),
        }
    }
}
