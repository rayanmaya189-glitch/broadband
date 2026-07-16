use crate::modules::notification::domain::value_objects::{NotificationChannel, NotificationId, NotificationStatus};

/// Notification aggregate root - represents a multi-channel notification
#[derive(Debug, Clone)]
pub struct Notification {
    pub id: NotificationId,
    pub template_id: Option<i64>,
    pub channel: NotificationChannel,
    pub recipient_type: String,
    pub recipient_id: i64,
    pub recipient_address: String,
    pub subject: Option<String>,
    pub body: String,
    pub status: NotificationStatus,
    pub retry_count: i32,
    pub max_retries: i32,
    pub last_error: Option<String>,
    pub sent_at: Option<chrono::DateTime<chrono::Utc>>,
    pub delivered_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Domain errors for Notification aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum NotificationDomainError {
    NotificationNotFound(i64),
    MaxRetriesExceeded,
    InvalidRecipient,
    InvalidChannel,
    AlreadyDelivered,
}

impl std::fmt::Display for NotificationDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotificationNotFound(id) => write!(f, "Notification {} not found", id),
            Self::MaxRetriesExceeded => write!(f, "Maximum retry attempts exceeded"),
            Self::InvalidRecipient => write!(f, "Invalid recipient address"),
            Self::InvalidChannel => write!(f, "Invalid notification channel"),
            Self::AlreadyDelivered => write!(f, "Notification already delivered"),
        }
    }
}

impl std::error::Error for NotificationDomainError {}

impl Notification {
    pub fn new(
        channel: NotificationChannel,
        recipient_type: String,
        recipient_id: i64,
        recipient_address: String,
        subject: Option<String>,
        body: String,
    ) -> Result<Self, NotificationDomainError> {
        if recipient_address.is_empty() {
            return Err(NotificationDomainError::InvalidRecipient);
        }
        Ok(Self {
            id: NotificationId::new(0),
            template_id: None,
            channel,
            recipient_type,
            recipient_id,
            recipient_address,
            subject,
            body,
            status: NotificationStatus::Queued,
            retry_count: 0,
            max_retries: 3,
            last_error: None,
            sent_at: None,
            delivered_at: None,
        })
    }

    pub fn mark_sent(&mut self) {
        self.status = NotificationStatus::Sent;
        self.sent_at = Some(chrono::Utc::now());
    }

    pub fn mark_delivered(&mut self) -> Result<(), NotificationDomainError> {
        if self.status == NotificationStatus::Delivered {
            return Err(NotificationDomainError::AlreadyDelivered);
        }
        self.status = NotificationStatus::Delivered;
        self.delivered_at = Some(chrono::Utc::now());
        Ok(())
    }

    pub fn mark_failed(&mut self, error: String) -> Result<(), NotificationDomainError> {
        self.retry_count += 1;
        if self.retry_count >= self.max_retries {
            self.status = NotificationStatus::Failed;
        } else {
            self.status = NotificationStatus::Retrying;
        }
        self.last_error = Some(error);
        Ok(())
    }

    pub fn can_retry(&self) -> bool {
        self.status == NotificationStatus::Failed && self.retry_count < self.max_retries
    }

    pub fn retry_delay_seconds(&self) -> u64 {
        10u64.pow(self.retry_count as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_notification() {
        let notif = Notification::new(
            NotificationChannel::Email,
            "customer".to_string(),
            1,
            "user@example.com".to_string(),
            Some("Welcome".to_string()),
            "Hello!".to_string(),
        );
        assert!(notif.is_ok());
        let notif = notif.unwrap();
        assert_eq!(notif.status, NotificationStatus::Queued);
    }

    #[test]
    fn test_delivery_flow() {
        let mut notif = Notification::new(
            NotificationChannel::Sms,
            "customer".to_string(),
            1,
            "+919876543210".to_string(),
            None,
            "Your OTP is 123456".to_string(),
        ).unwrap();
        notif.mark_sent();
        assert_eq!(notif.status, NotificationStatus::Sent);
        notif.mark_delivered().unwrap();
        assert_eq!(notif.status, NotificationStatus::Delivered);
    }

    #[test]
    fn test_retry_flow() {
        let mut notif = Notification::new(
            NotificationChannel::Email,
            "customer".to_string(),
            1,
            "user@example.com".to_string(),
            None,
            "Body".to_string(),
        ).unwrap();
        notif.mark_failed("SMTP error".to_string()).unwrap();
        assert!(notif.can_retry());
        assert_eq!(notif.retry_count, 1);
    }
}
