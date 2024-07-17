// src/management_role.rs
use crate::client::MinioClient;
use reqwest::Error;

pub struct RoleManager {
    client: MinioClient,
}

impl RoleManager {
    pub fn new(client: MinioClient) -> Self {
        RoleManager { client }
    }

    pub async fn create_role(&self, role_name: &str, assume_role_policy_document: &str) -> Result<(), Error> {
        let url = format!("{}/minio/admin/v3/add-role?name={}", self.client.endpoint, role_name);
        let response = self.client.client.lock().await.post(&url)
            .header("Content-Type", "application/json")
            .body(assume_role_policy_document.to_string())
            .send()
            .await?;

        response.error_for_status_ref()?;
        Ok(())
    }

    pub async fn delete_role(&self, role_name: &str) -> Result<(), Error> {
        let url = format!("{}/minio/admin/v3/remove-role?name={}", self.client.endpoint, role_name);
        let response = self.client.client.lock().await.post(&url)
            .send()
            .await?;

        response.error_for_status_ref()?;
        Ok(())
    }

    pub async fn attach_role_policy(&self, role_name: &str, policy_name: &str) -> Result<(), Error> {
        let url = format!("{}/minio/admin/v3/set-role-policy?role={}&policy={}", self.client.endpoint, role_name, policy_name);
        let response = self.client.client.lock().await.post(&url)
            .send()
            .await?;

        response.error_for_status_ref()?;
        Ok(())
    }

    pub async fn detach_role_policy(&self, role_name: &str, policy_name: &str) -> Result<(), Error> {
        let url = format!("{}/minio/admin/v3/unset-role-policy?role={}&policy={}", self.client.endpoint, role_name, policy_name);
        let response = self.client.client.lock().await.post(&url)
            .send()
            .await?;

        response.error_for_status_ref()?;
        Ok(())
    }

    pub async fn assume_role(&self, role_name: &str, session_name: &str) -> Result<(), Error> {
        let url = format!("{}/minio/admin/v3/assume-role?role={}&session={}", self.client.endpoint, role_name, session_name);
        let response = self.client.client.lock().await.post(&url)
            .send()
            .await?;

        response.error_for_status_ref()?;
        Ok(())
    }
}
