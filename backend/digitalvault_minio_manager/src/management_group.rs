// src/management_group.rs
use crate::client::MinioClient;
use reqwest::Error;

pub struct GroupManager {
    client: MinioClient,
}

impl GroupManager {
    pub fn new(client: MinioClient) -> Self {
        GroupManager { client }
    }

    pub async fn create_group(&self, group_name: &str) -> Result<(), Error> {
        let url = format!("{}/minio/admin/v3/add-group?group={}", self.client.endpoint, group_name);
        let response = self.client.client.lock().await.post(&url)
            .send()
            .await?;

        response.error_for_status_ref()?;
        Ok(())
    }

    pub async fn delete_group(&self, group_name: &str) -> Result<(), Error> {
        let url = format!("{}/minio/admin/v3/remove-group?group={}", self.client.endpoint, group_name);
        let response = self.client.client.lock().await.post(&url)
            .send()
            .await?;

        response.error_for_status_ref()?;
        Ok(())
    }

    pub async fn add_user_to_group(&self, user_name: &str, group_name: &str) -> Result<(), Error> {
        let url = format!("{}/minio/admin/v3/update-group-members?group={}&members={}&isRemove=false", self.client.endpoint, group_name, user_name);
        let response = self.client.client.lock().await.post(&url)
            .send()
            .await?;

        response.error_for_status_ref()?;
        Ok(())
    }

    pub async fn remove_user_from_group(&self, user_name: &str, group_name: &str) -> Result<(), Error> {
        let url = format!("{}/minio/admin/v3/update-group-members?group={}&members={}&isRemove=true", self.client.endpoint, group_name, user_name);
        let response = self.client.client.lock().await.post(&url)
            .send()
            .await?;

        response.error_for_status_ref()?;
        Ok(())
    }

    pub async fn list_groups(&self) -> Result<Vec<String>, Error> {
        let url = format!("{}/minio/admin/v3/list-groups", self.client.endpoint);
        let response = self.client.client.lock().await.get(&url)
            .send()
            .await?;

        response.error_for_status_ref()?;
        let groups: Vec<String> = response.json().await?;
        Ok(groups)
    }

    pub async fn list_group_users(&self, group_name: &str) -> Result<Vec<String>, Error> {
        let url = format!("{}/minio/admin/v3/list-group-members?group={}", self.client.endpoint, group_name);
        let response = self.client.client.lock().await.get(&url)
            .send()
            .await?;

        response.error_for_status_ref()?;
        let users: Vec<String> = response.json().await?;
        Ok(users)
    }
}
