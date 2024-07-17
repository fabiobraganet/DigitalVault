use rusoto_core::RusotoError;
use rusoto_s3::{CreateBucketRequest, HeadBucketRequest, S3, S3Client};
use std::error::Error;
use std::sync::Arc;

pub struct MinioEnsure {
    client: Arc<S3Client>,
}

impl MinioEnsure {
    pub fn new(client: Arc<S3Client>) -> Self {
        MinioEnsure { client }
    }

    pub async fn ensure_bucket_exists(&self, bucket: &str) -> Result<(), Box<dyn Error>> {
        let head_request = HeadBucketRequest {
            bucket: bucket.to_string(),
            expected_bucket_owner: None, // Adiciona o campo expected_bucket_owner com valor None
        };

        match self.client.head_bucket(head_request).await {
            Ok(_) => return Ok(()),
            Err(RusotoError::Service(e)) if e.to_string().contains("NotFound") => {
                let create_request = CreateBucketRequest {
                    bucket: bucket.to_string(),
                    ..Default::default()
                };

                match self.client.create_bucket(create_request).await {
                    Ok(_) => Ok(()),
                    Err(create_err) => {
                        if create_err.to_string().contains("BucketAlreadyOwnedByYou") {
                            return Ok(());
                        }
                        Err(Box::new(create_err))
                    }
                }
            },
            Err(e) => {
                eprintln!("Failed to check bucket existence: {:?}", e);
                Err(Box::new(e))
            }
        }
    }
}
