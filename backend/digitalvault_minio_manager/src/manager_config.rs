// src/config.rs
pub mod config {
    use crate::client::MinioClient;
    use crate::management_user::UserManager;
    use crate::management_policy::PolicyManager;
    use crate::management_role::RoleManager;
    use crate::management_audit::AuditManager;
    use crate::management_group::GroupManager;
    use std::env;

    pub struct Config {
        pub minio_client: MinioClient,
        pub endpoint: String,
        pub access_key: String,
        pub secret_key: String,
    }

    impl Config {
        pub fn new() -> Self {
            dotenv::from_filename(".env.test").ok();

            let endpoint = env::var("MINIO_ENDPOINT").expect("MINIO_ENDPOINT not set");
            let access_key = env::var("MINIO_ACCESS_KEY").expect("MINIO_ACCESS_KEY not set");
            let secret_key = env::var("MINIO_SECRET_KEY").expect("MINIO_SECRET_KEY not set");

            let minio_client = MinioClient::new(&endpoint, &access_key, &secret_key);

            Config {
                minio_client,
                endpoint,
                access_key,
                secret_key,
            }
        }

        pub fn user_service(&self) -> UserManager {
            UserManager::new(self.minio_client.clone())
        }

        pub fn policy_service(&self) -> PolicyManager {
            PolicyManager::new(self.minio_client.clone())
        }

        pub fn role_service(&self) -> RoleManager {
            RoleManager::new(self.minio_client.clone())
        }

        pub fn audit_service(&self) -> AuditManager {
            AuditManager::new(self.minio_client.clone())
        }

        pub fn group_service(&self) -> GroupManager {
            GroupManager::new(self.minio_client.clone())
        }
    }
}
