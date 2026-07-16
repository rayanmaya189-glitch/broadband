use crate::modules::plans::domain::value_objects::{PlanId, PlanStatus};

/// Plan aggregate root - represents an ISP internet plan
#[derive(Debug, Clone)]
pub struct Plan {
    pub id: PlanId,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub speed_label: String,
    pub download_mbps: i32,
    pub upload_mbps: i32,
    pub burst_mbps: Option<i32>,
    pub data_quota: String,
    pub is_popular: bool,
    pub is_business: bool,
    pub is_active: bool,
    pub sort_order: i32,
    pub status: PlanStatus,
}

/// Domain errors for Plan aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum PlanDomainError {
    InvalidSpeed,
    InvalidSlug,
    PlanNotFound(i64),
    PlanNotActive,
    AlreadyPublished,
    AlreadyUnpublished,
    SlugAlreadyExists(String),
    InvalidPrice,
}

impl std::fmt::Display for PlanDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSpeed => write!(f, "Upload speed cannot exceed download speed"),
            Self::InvalidSlug => write!(f, "Plan slug must be lowercase alphanumeric with hyphens"),
            Self::PlanNotFound(id) => write!(f, "Plan {} not found", id),
            Self::PlanNotActive => write!(f, "Plan is not active"),
            Self::AlreadyPublished => write!(f, "Plan is already published"),
            Self::AlreadyUnpublished => write!(f, "Plan is already unpublished"),
            Self::SlugAlreadyExists(slug) => write!(f, "Plan slug '{}' already exists", slug),
            Self::InvalidPrice => write!(f, "Plan price must be greater than zero"),
        }
    }
}

impl std::error::Error for PlanDomainError {}

impl Plan {
    pub fn new(
        slug: String,
        name: String,
        description: Option<String>,
        speed_label: String,
        download_mbps: i32,
        upload_mbps: i32,
        burst_mbps: Option<i32>,
    ) -> Result<Self, PlanDomainError> {
        if upload_mbps > download_mbps {
            return Err(PlanDomainError::InvalidSpeed);
        }
        if !Self::is_valid_slug(&slug) {
            return Err(PlanDomainError::InvalidSlug);
        }
        Ok(Self {
            id: PlanId::new(0),
            slug,
            name,
            description,
            speed_label,
            download_mbps,
            upload_mbps,
            burst_mbps,
            data_quota: "unlimited".to_string(),
            is_popular: false,
            is_business: false,
            is_active: false,
            sort_order: 0,
            status: PlanStatus::Draft,
        })
    }

    fn is_valid_slug(slug: &str) -> bool {
        if slug.is_empty() || slug.len() > 100 {
            return false;
        }
        slug.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    }

    pub fn publish(&mut self) -> Result<(), PlanDomainError> {
        if self.status == PlanStatus::Published {
            return Err(PlanDomainError::AlreadyPublished);
        }
        self.status = PlanStatus::Published;
        self.is_active = true;
        Ok(())
    }

    pub fn unpublish(&mut self) -> Result<(), PlanDomainError> {
        if self.status == PlanStatus::Draft {
            return Err(PlanDomainError::AlreadyUnpublished);
        }
        self.status = PlanStatus::Draft;
        self.is_active = false;
        Ok(())
    }

    pub fn is_available_for_purchase(&self) -> bool {
        self.is_active && self.status == PlanStatus::Published
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_plan() {
        let plan = Plan::new(
            "basic-50".to_string(),
            "Basic".to_string(),
            Some("Basic internet plan".to_string()),
            "50 Mbps".to_string(),
            50,
            25,
            Some(75),
        );
        assert!(plan.is_ok());
        let plan = plan.unwrap();
        assert_eq!(plan.status, PlanStatus::Draft);
        assert!(!plan.is_active);
    }

    #[test]
    fn test_publish_plan() {
        let mut plan = Plan::new(
            "basic-50".to_string(),
            "Basic".to_string(),
            None,
            "50 Mbps".to_string(),
            50,
            25,
            None,
        )
        .unwrap();
        let result = plan.publish();
        assert!(result.is_ok());
        assert_eq!(plan.status, PlanStatus::Published);
        assert!(plan.is_active);
        assert!(plan.is_available_for_purchase());
    }

    #[test]
    fn test_invalid_upload_speed() {
        let plan = Plan::new(
            "invalid".to_string(),
            "Invalid".to_string(),
            None,
            "50 Mbps".to_string(),
            25,
            50, // upload > download
            None,
        );
        assert_eq!(plan, Err(PlanDomainError::InvalidSpeed));
    }
}
