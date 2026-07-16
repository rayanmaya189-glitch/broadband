use crate::modules::branches::domain::value_objects::{BranchId, BranchStatus};

/// Branch aggregate root - represents a geographic service location
#[derive(Debug, Clone)]
pub struct Branch {
    pub id: BranchId,
    pub name: String,
    pub slug: String,
    pub code: String,
    pub city: String,
    pub state: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub timezone: String,
    pub is_active: bool,
    pub status: BranchStatus,
}

/// Domain errors for Branch aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum BranchDomainError {
    InvalidSlug,
    InvalidCode,
    BranchNotFound(i64),
    SlugAlreadyExists(String),
    CodeAlreadyExists(String),
    CannotDeactivateActiveBranch,
}

impl std::fmt::Display for BranchDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSlug => write!(f, "Branch slug must be lowercase alphanumeric with hyphens"),
            Self::InvalidCode => write!(f, "Branch code must be 2-20 uppercase alphanumeric characters"),
            Self::BranchNotFound(id) => write!(f, "Branch {} not found", id),
            Self::SlugAlreadyExists(slug) => write!(f, "Branch slug '{}' already exists", slug),
            Self::CodeAlreadyExists(code) => write!(f, "Branch code '{}' already exists", code),
            Self::CannotDeactivateActiveBranch => {
                write!(f, "Cannot deactivate branch with active customers")
            }
        }
    }
}

impl std::error::Error for BranchDomainError {}

impl Branch {
    pub fn new(
        name: String,
        slug: String,
        code: String,
        city: String,
        state: String,
    ) -> Result<Self, BranchDomainError> {
        if !Self::is_valid_slug(&slug) {
            return Err(BranchDomainError::InvalidSlug);
        }
        if !Self::is_valid_code(&code) {
            return Err(BranchDomainError::InvalidCode);
        }
        Ok(Self {
            id: BranchId::new(0),
            name,
            slug,
            code,
            city,
            state,
            address: None,
            phone: None,
            email: None,
            timezone: "Asia/Kolkata".to_string(),
            is_active: true,
            status: BranchStatus::Active,
        })
    }

    fn is_valid_slug(slug: &str) -> bool {
        slug.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
            && !slug.is_empty()
            && slug.len() <= 100
    }

    fn is_valid_code(code: &str) -> bool {
        code.chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
            && code.len() >= 2
            && code.len() <= 20
    }

    pub fn deactivate(&mut self) -> Result<(), BranchDomainError> {
        if self.status == BranchStatus::Active {
            return Err(BranchDomainError::CannotDeactivateActiveBranch);
        }
        self.status = BranchStatus::Inactive;
        self.is_active = false;
        Ok(())
    }

    pub fn reactivate(&mut self) {
        self.status = BranchStatus::Active;
        self.is_active = true;
    }

    pub fn is_available(&self) -> bool {
        self.is_active && self.status == BranchStatus::Active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_branch() {
        let branch = Branch::new(
            "Jalgaon Main".to_string(),
            "jalgaon-main".to_string(),
            "JLG".to_string(),
            "Jalgaon".to_string(),
            "Maharashtra".to_string(),
        );
        assert!(branch.is_ok());
        let branch = branch.unwrap();
        assert_eq!(branch.status, BranchStatus::Active);
        assert!(branch.is_available());
    }

    #[test]
    fn test_invalid_slug() {
        let branch = Branch::new(
            "Test".to_string(),
            "Invalid Slug!".to_string(),
            "TST".to_string(),
            "City".to_string(),
            "State".to_string(),
        );
        assert_eq!(branch, Err(BranchDomainError::InvalidSlug));
    }

    #[test]
    fn test_invalid_code() {
        let branch = Branch::new(
            "Test".to_string(),
            "test".to_string(),
            "x".to_string(), // Too short
            "City".to_string(),
            "State".to_string(),
        );
        assert_eq!(branch, Err(BranchDomainError::InvalidCode));
    }
}
