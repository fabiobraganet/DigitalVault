use rusoto_s3::{HeadObjectRequest, S3Client, S3};
use std::error::Error;
use std::sync::Arc;
use actix_web::http::StatusCode;
use rusoto_core::RusotoError;

pub struct MinioIntegrity {
    client: Arc<S3Client>,
}

impl MinioIntegrity {
    pub fn new(client: Arc<S3Client>) -> Self {
        MinioIntegrity { client }
    }

    pub async fn get_etag(&self, bucket: &str, key: &str) -> Result<String, Box<dyn Error>> {
        let head_request = HeadObjectRequest {
            bucket: bucket.to_string(),
            key: key.to_string(),
            ..Default::default()
        };

        match self.client.head_object(head_request).await {
            Ok(output) => {
                if let Some(etag) = output.e_tag {
                    return Ok(etag);
                }
                Err(Box::new(RusotoError::<rusoto_s3::HeadObjectError>::Unknown(
                    rusoto_core::request::BufferedHttpResponse {
                        status: StatusCode::NOT_FOUND,
                        body: "".into(),
                        headers: Default::default(),
                    },
                )))
            },
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn check_integrity(&self, bucket: &str, key: &str, expected_etag: &str) -> Result<bool, Box<dyn Error>> {
        let etag = self.get_etag(bucket, key).await?;
        Ok(etag == expected_etag)
    }
}
