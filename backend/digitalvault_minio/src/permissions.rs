use rusoto_s3::{S3Client, S3, PutBucketPolicyRequest};
use std::error::Error;
use std::sync::Arc;

pub struct MinioPermissions {
    client: Arc<S3Client>,
}

impl MinioPermissions {
    pub fn new(client: Arc<S3Client>) -> Self {
        MinioPermissions { client }
    }

    pub async fn set_bucket_policy(&self, bucket: &str, policy: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        let request = PutBucketPolicyRequest {
            bucket: bucket.to_string(),
            policy: policy.to_string(),
            ..Default::default()
        };

        match self.client.put_bucket_policy(request).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}