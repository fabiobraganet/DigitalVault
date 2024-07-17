use rusoto_core::RusotoError;
use rusoto_s3::{GetObjectRequest, S3Client, S3};
use std::error::Error;
use std::sync::Arc;
use tokio::io::AsyncReadExt;

pub struct MinioDownload {
    client: Arc<S3Client>,
}

impl MinioDownload {
    pub fn new(client: Arc<S3Client>) -> Self {
        MinioDownload { client }
    }

    pub async fn download(&self, bucket: &str, key: &str) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        let get_req = GetObjectRequest {
            bucket: bucket.to_string(),
            key: key.to_string(),
            ..Default::default()
        };

        let result = self.client.get_object(get_req).await;
        match result {
            Ok(output) => {
                let mut body = output.body.unwrap().into_async_read();
                let mut data = Vec::new();
                body.read_to_end(&mut data).await?;
                Ok(data)
            }
            Err(RusotoError::Service(e)) => Err(Box::new(e)),
            Err(e) => Err(Box::new(e)),
        }
    }
}
