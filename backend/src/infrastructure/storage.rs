use aws_config::BehaviorVersion;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::Client;
use tracing::{debug, info};

use crate::shared::errors::AppError;

/// MinIO/S3 compatible storage service
#[derive(Clone)]
pub struct StorageService {
    client: Client,
    default_bucket: String,
}

impl StorageService {
    /// Create a new storage service from environment variables
    pub async fn from_env() -> Result<Self, AppError> {
        let endpoint =
            std::env::var("MINIO_ENDPOINT").unwrap_or_else(|_| "http://localhost:9000".to_string());
        let access_key = std::env::var("MINIO_ACCESS_KEY")
            .map_err(|_| AppError::Internal(anyhow::anyhow!("MINIO_ACCESS_KEY not set")))?;
        let secret_key = std::env::var("MINIO_SECRET_KEY")
            .map_err(|_| AppError::Internal(anyhow::anyhow!("MINIO_SECRET_KEY not set")))?;
        let region = std::env::var("MINIO_REGION").unwrap_or_else(|_| "us-east-1".to_string());
        let default_bucket = std::env::var("MINIO_BUCKET_DOCUMENTS")
            .unwrap_or_else(|_| "aeroxe-documents".to_string());

        let cred =
            aws_sdk_s3::config::Credentials::new(&access_key, &secret_key, None, None, "minio");

        let config = aws_sdk_s3::config::Builder::new()
            .behavior_version(BehaviorVersion::latest())
            .region(aws_sdk_s3::config::Region::new(region))
            .endpoint_url(&endpoint)
            .credentials_provider(cred)
            .force_path_style(true) // Required for MinIO
            .build();

        let client = Client::from_conf(config);

        Ok(Self {
            client,
            default_bucket,
        })
    }

    /// Generate a presigned URL for uploading a file
    pub async fn presign_upload(
        &self,
        bucket: Option<&str>,
        key: &str,
        content_type: &str,
        expires_in_secs: u64,
    ) -> Result<String, AppError> {
        let bucket = bucket.unwrap_or(&self.default_bucket);

        let presign_config = PresigningConfig::expires_in(std::time::Duration::from_secs(
            expires_in_secs,
        ))
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Presigning config error: {}", e)))?;

        let presigned = self
            .client
            .put_object()
            .bucket(bucket)
            .key(key)
            .content_type(content_type)
            .presigned(presign_config)
            .await
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to generate presigned URL: {}", e))
            })?;

        let url = presigned.uri().to_string();
        debug!(bucket = %bucket, key = %key, "Generated presigned upload URL");

        Ok(url)
    }

    /// Generate a presigned URL for downloading a file
    pub async fn presign_download(
        &self,
        bucket: Option<&str>,
        key: &str,
        expires_in_secs: u64,
    ) -> Result<String, AppError> {
        let bucket = bucket.unwrap_or(&self.default_bucket);

        let presign_config = PresigningConfig::expires_in(std::time::Duration::from_secs(
            expires_in_secs,
        ))
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Presigning config error: {}", e)))?;

        let presigned = self
            .client
            .get_object()
            .bucket(bucket)
            .key(key)
            .presigned(presign_config)
            .await
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to generate presigned URL: {}", e))
            })?;

        let url = presigned.uri().to_string();
        debug!(bucket = %bucket, key = %key, "Generated presigned download URL");

        Ok(url)
    }

    /// Upload a file directly to storage
    pub async fn upload_object(
        &self,
        bucket: Option<&str>,
        key: &str,
        content_type: &str,
        body: Vec<u8>,
    ) -> Result<String, AppError> {
        let bucket = bucket.unwrap_or(&self.default_bucket);

        let result = self
            .client
            .put_object()
            .bucket(bucket)
            .key(key)
            .content_type(content_type)
            .body(body.into())
            .send()
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to upload object: {}", e)))?;

        let location = format!("{}/{}", bucket, key);
        info!(bucket = %bucket, key = %key, etag = ?result.e_tag(), "Uploaded object to storage");

        Ok(location)
    }

    /// Download a file from storage
    pub async fn download_object(
        &self,
        bucket: Option<&str>,
        key: &str,
    ) -> Result<Vec<u8>, AppError> {
        let bucket = bucket.unwrap_or(&self.default_bucket);

        let result = self
            .client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to download object: {}", e)))?;

        let bytes = result.body.collect().await.map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Failed to read object body: {}", e))
        })?;
        let data = bytes.into_bytes();

        debug!(bucket = %bucket, key = %key, size = data.len(), "Downloaded object from storage");

        Ok(data.to_vec())
    }

    /// Delete a file from storage
    pub async fn delete_object(&self, bucket: Option<&str>, key: &str) -> Result<(), AppError> {
        let bucket = bucket.unwrap_or(&self.default_bucket);

        self.client
            .delete_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to delete object: {}", e)))?;

        info!(bucket = %bucket, key = %key, "Deleted object from storage");

        Ok(())
    }

    /// Check if an object exists in storage
    pub async fn object_exists(&self, bucket: Option<&str>, key: &str) -> Result<bool, AppError> {
        let bucket = bucket.unwrap_or(&self.default_bucket);

        match self
            .client
            .head_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                let err_str = e.to_string();
                if err_str.contains("NoSuchKey") || err_str.contains("404") {
                    Ok(false)
                } else {
                    Err(AppError::Internal(anyhow::anyhow!(
                        "Failed to check object existence: {}",
                        e
                    )))
                }
            }
        }
    }

    /// Ensure a bucket exists, create if not
    pub async fn ensure_bucket(&self, bucket: &str) -> Result<(), AppError> {
        let exists = self
            .client
            .list_buckets()
            .send()
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to list buckets: {}", e)))?
            .buckets()
            .iter()
            .any(|b| b.name() == Some(bucket));

        if !exists {
            self.client
                .create_bucket()
                .bucket(bucket)
                .send()
                .await
                .map_err(|e| {
                    AppError::Internal(anyhow::anyhow!("Failed to create bucket: {}", e))
                })?;

            info!(bucket = %bucket, "Created new storage bucket");
        }

        Ok(())
    }
}
