// src/management_policy.rs
use crate::client::MinioClient;
use reqwest::Error;

pub struct PolicyManager {
    client: MinioClient,
}

impl PolicyManager {
    pub fn new(client: MinioClient) -> Self {
        PolicyManager { client }
    }

    pub async fn create_policy(&self, policy_name: &str, policy_document: &str) -> Result<(), Error> {
        let url = format!("{}/minio/admin/v3/add-canned-policy?name={}", self.client.endpoint, policy_name);
        let response = self.client.client.lock().await.post(&url)
            .header("Content-Type", "application/json")
            .body(policy_document.to_string())
            .send()
            .await?;

        response.error_for_status_ref()?;
        Ok(())
    }

    pub async fn delete_policy(&self, policy_name: &str) -> Result<(), Error> {
        let url = format!("{}/minio/admin/v3/remove-canned-policy?name={}", self.client.endpoint, policy_name);
        let response = self.client.client.lock().await.post(&url)
            .send()
            .await?;

        response.error_for_status_ref()?;
        Ok(())
    }

    pub async fn attach_policy_to_user(&self, policy_name: &str, user_name: &str) -> Result<(), Error> {
        let url = format!("{}/minio/admin/v3/set-user-policy?policy={}&user={}", self.client.endpoint, policy_name, user_name);
        let response = self.client.client.lock().await.post(&url)
            .send()
            .await?;

        response.error_for_status_ref()?;
        Ok(())
    }

    pub async fn attach_policy_to_group(&self, policy_name: &str, group_name: &str) -> Result<(), Error> {
        let url = format!("{}/minio/admin/v3/set-group-policy?policy={}&group={}", self.client.endpoint, policy_name, group_name);
        let response = self.client.client.lock().await.post(&url)
            .send()
            .await?;

        response.error_for_status_ref()?;
        Ok(())
    }

    pub async fn detach_policy_from_user(&self, policy_name: &str, user_name: &str) -> Result<(), Error> {
        let url = format!("{}/minio/admin/v3/unset-user-policy?policy={}&user={}", self.client.endpoint, policy_name, user_name);
        let response = self.client.client.lock().await.post(&url)
            .send()
            .await?;

        response.error_for_status_ref()?;
        Ok(())
    }

    pub async fn detach_policy_from_group(&self, policy_name: &str, group_name: &str) -> Result<(), Error> {
        let url = format!("{}/minio/admin/v3/unset-group-policy?policy={}&group={}", self.client.endpoint, policy_name, group_name);
        let response = self.client.client.lock().await.post(&url)
            .send()
            .await?;

        response.error_for_status_ref()?;
        Ok(())
    }

    pub async fn list_policies(&self) -> Result<Vec<String>, Error> {
        let url = format!("{}/minio/admin/v3/list-canned-policies", self.client.endpoint);
        let response = self.client.client.lock().await.get(&url)
            .send()
            .await?;

        response.error_for_status_ref()?;
        let policies: Vec<String> = response.json().await?;
        Ok(policies)
    }
}
