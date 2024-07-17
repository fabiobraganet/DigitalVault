// src/management_user.rs
use crate::client::MinioClient;
use reqwest::Error;

/// UserManager é responsável por gerenciar usuários no MinIO.
pub struct UserManager {
    client: MinioClient,
}

impl UserManager {
    pub fn new(client: MinioClient) -> Self {
        UserManager { client }
    }

    pub async fn create_user(&self, user_name: &str) -> Result<(), Error> {
        let url = format!("{}/minio/admin/v3/add-user?user={}", self.client.endpoint, user_name);
        let response = self.client.client.lock().await.post(&url)
            .send()
            .await?;

        response.error_for_status_ref()?;
        Ok(())
    }

    pub async fn delete_user(&self, user_name: &str) -> Result<(), Error> {
        let url = format!("{}/minio/admin/v3/remove-user?user={}", self.client.endpoint, user_name);
        let response = self.client.client.lock().await.post(&url)
            .send()
            .await?;

        response.error_for_status_ref()?;
        Ok(())
    }

    pub async fn list_users(&self) -> Result<Vec<String>, Error> {
        let url = format!("{}/minio/admin/v3/list-users", self.client.endpoint);
        let response = self.client.client.lock().await.get(&url)
            .send()
            .await?;

        response.error_for_status_ref()?;
        let users: Vec<String> = response.json().await?;
        Ok(users)
    }

    pub async fn update_user(&self, user_name: &str, new_access_key: Option<&str>, new_secret_key: Option<&str>) -> Result<(), Error> {
        let url = format!("{}/minio/admin/v3/update-user?user={}", self.client.endpoint, user_name);
        let mut body = serde_json::json!({});
        if let Some(access_key) = new_access_key {
            body["accessKey"] = serde_json::json!(access_key);
        }
        if let Some(secret_key) = new_secret_key {
            body["secretKey"] = serde_json::json!(secret_key);
        }

        let response = self.client.client.lock().await.post(&url)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?;

        response.error_for_status_ref()?;
        Ok(())
    }
}
