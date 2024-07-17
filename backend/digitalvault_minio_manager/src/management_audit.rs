// src/management_audit.rs
use crate::client::MinioClient;
use reqwest::Error;

pub struct AuditManager {
    client: MinioClient,
}

impl AuditManager {
    pub fn new(client: MinioClient) -> Self {
        AuditManager { client }
    }

    pub async fn log_event(&self, event: &str) -> Result<(), Error> {
        let url = format!("{}/minio/admin/v3/log-event?event={}", self.client.endpoint, event);
        let response = self.client.client.lock().await.post(&url)
            .send()
            .await?;

        response.error_for_status()?;

        Ok(())
    }

    pub async fn get_audit_logs(&self, filter: Option<&str>) -> Result<Vec<String>, Error> {
        let url = match filter {
            Some(f) => format!("{}/minio/admin/v3/get-audit-logs?filter={}", self.client.endpoint, f),
            None => format!("{}/minio/admin/v3/get-audit-logs", self.client.endpoint),
        };
        let response = self.client.client.lock().await.get(&url)
            .send()
            .await?;

        response.error_for_status_ref()?;

        let logs: Vec<String> = response.json().await?;

        Ok(logs)
    }

    pub async fn monitor_access(&self) -> Result<(), Error> {
        let url = format!("{}/minio/admin/v3/monitor-access", self.client.endpoint);
        let response = self.client.client.lock().await.get(&url)
            .send()
            .await?;

        response.error_for_status()?;
        
        Ok(())
    }
}