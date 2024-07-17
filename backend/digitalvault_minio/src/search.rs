use rusoto_s3::{ListObjectsV2Request, S3Client, S3};
use std::error::Error;
use std::sync::Arc;

pub struct MinioSearch {
    client: Arc<S3Client>,
}

impl MinioSearch {
    pub fn new(client: Arc<S3Client>) -> Self {
        MinioSearch { client }
    }

    pub async fn search_objects(&self, bucket: &str, prefix: Option<&str>) -> Result<Vec<String>, Box<dyn Error>> {
        let list_request = ListObjectsV2Request {
            bucket: bucket.to_string(),
            prefix: prefix.map(|p| p.to_string()),
            ..Default::default()
        };

        match self.client.list_objects_v2(list_request).await {
            Ok(output) => {
                let keys = output.contents.unwrap_or_default()
                    .into_iter()
                    .filter_map(|obj| obj.key)
                    .collect();
                Ok(keys)
            },
            Err(e) => Err(Box::new(e)),
        }
    }
}
