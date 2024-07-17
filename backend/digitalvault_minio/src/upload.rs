use rusoto_s3::{PutObjectRequest, S3Client, S3};
use std::sync::Arc;
use std::error::Error;

pub struct MinioUpload {
    client: Arc<S3Client>,
}

impl MinioUpload {
    pub fn new(client: Arc<S3Client>) -> Self {
        MinioUpload { client }
    }

    pub async fn upload(&self, bucket: &str, key: &str, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let put_request = PutObjectRequest {
            bucket: bucket.to_string(),
            key: key.to_string(),
            body: Some(data.to_vec().into()),
            ..Default::default()
        };

        match self.client.put_object(put_request).await {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Failed to upload object: {:?}", e);
                Err(Box::new(e))
            }
        }
    }
}
