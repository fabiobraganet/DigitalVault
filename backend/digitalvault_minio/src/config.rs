// src/config.rs
pub mod test {
    use crate::client::MinioClient;
    use crate::copy::MinioCopy;
    use crate::download::MinioDownload;
    use crate::upload::MinioUpload;
    use crate::delete::MinioDelete;
    use crate::list::MinioList;
    use rusoto_iam::IamClient;
    use rusoto_s3::S3Client;
    use std::env;
    use std::sync::Arc;

    pub struct ConfigTest {
        pub s3_client: Arc<S3Client>,
        pub iam_client: Arc<IamClient>,
        pub source_bucket: String,
        pub destination_bucket: String,
    }

    impl ConfigTest {
        pub fn new() -> Self {
            dotenv::from_filename(".env.test").ok();

            let endpoint = env::var("MINIO_ENDPOINT").expect("MINIO_ENDPOINT not set");
            let access_key = env::var("MINIO_ACCESS_KEY").expect("MINIO_ACCESS_KEY not set");
            let secret_key = env::var("MINIO_SECRET_KEY").expect("MINIO_SECRET_KEY not set");
            let buckets = env::var("MINIO_BUCKETS").expect("MINIO_BUCKETS must be set");
            let mut bucket_iter = buckets.split(',');
            let source_bucket = bucket_iter.next().unwrap().to_string();
            let destination_bucket = bucket_iter.next().unwrap().to_string();

            let minioclient = MinioClient::new(&endpoint, &access_key, &secret_key);

            ConfigTest {
                s3_client: minioclient.s3_client,
                iam_client: minioclient.iam_client,
                source_bucket,
                destination_bucket,
            }
        }

        pub fn list_service(&self) -> MinioList {
            MinioList::new(self.s3_client.clone())
        }

        pub fn download_service(&self) -> MinioDownload {
            MinioDownload::new(self.s3_client.clone())
        }

        pub fn upload_service(&self) -> MinioUpload {
            MinioUpload::new(self.s3_client.clone())
        }

        pub fn delete_service(&self) -> MinioDelete {
            MinioDelete::new(self.s3_client.clone())
        }

        pub fn copy_service(&self) -> MinioCopy {
            MinioCopy::new(self.s3_client.clone())
        }
    }
}
