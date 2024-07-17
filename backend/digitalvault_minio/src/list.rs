use rusoto_s3::{S3Client, ListObjectsV2Request, ListObjectsV2Output, ListBucketsOutput, S3};
use std::error::Error;
use std::sync::Arc;

pub struct MinioList {
    client: Arc<S3Client>,
}

impl MinioList {
    pub fn new(client: Arc<S3Client>) -> Self {
        MinioList { client }
    }

    pub async fn list_objects(&self, bucket: &str) -> Result<ListObjectsV2Output, Box<dyn Error + Send + Sync>> {
        let request = ListObjectsV2Request {
            bucket: bucket.to_string(),
            ..Default::default()
        };

        let result = self.client.list_objects_v2(request).await;
        match result {
            Ok(output) => Ok(output),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn list_buckets(&self) -> Result<ListBucketsOutput, Box<dyn Error + Send + Sync>> {
        let result = self.client.list_buckets().await;
        match result {
            Ok(output) => Ok(output),
            Err(e) => Err(Box::new(e)),
        }
    }
}
