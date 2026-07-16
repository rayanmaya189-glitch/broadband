use crate::modules::identity::domain::value_objects::{UserId, UserStatus};

/// User aggregate root - represents a system user (staff or customer-facing)
#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub email: String,
    pub phone: String,
    pub name: String,
    pub password_hash: Option<String>,
    pub branch_id: Option<i64>,
    pub status: UserStatus,
    pub two_factor_enabled: bool,
    pub phone_verified: bool,
    pub email_verified: bool,
    pub failed_login_attempts: i32,
    pub locked_until: Option<chrono::DateTime<chrono::Utc>>,
}

/// Domain errors for User aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum UserDomainError {
    InvalidEmail,
    InvalidPhone,
    PasswordTooWeak,
    AccountLocked,
    AlreadyVerified,
    MaxLoginAttemptsExceeded,
    UserNotFound(i64),
    TwoFactorAlreadyEnabled,
}

impl std::fmt::Display for UserDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidEmail => write!(f, "Invalid email format"),
            Self::InvalidPhone => write!(f, "Invalid phone format"),
            Self::PasswordTooWeak => write!(f, "Password must be at least 8 characters"),
            Self::AccountLocked => write!(f, "Account is locked due to too many failed attempts"),
            Self::AlreadyVerified => write!(f, "Already verified"),
            Self::MaxLoginAttemptsExceeded => write!(f, "Maximum login attempts exceeded"),
            Self::UserNotFound(id) => write!(f, "User {} not found", id),
            Self::TwoFactorAlreadyEnabled => write!(f, "Two-factor authentication is already enabled"),
        }
    }
}

impl std::error::Error for UserDomainError {}

impl User {
    pub fn new(
        email: String,
        phone: String,
        name: String,
        branch_id: Option<i64>,
    ) -> Result<Self, UserDomainError> {
        if !Self::is_valid_email(&email) {
            return Err(UserDomainError::InvalidEmail);
        }
        if !Self::is_valid_phone(&phone) {
            return Err(UserDomainError::InvalidPhone);
        }
        Ok(Self {
            id: UserId::new(0),
            email,
            phone,
            name,
            password_hash: None,
            branch_id,
            status: UserStatus::Active,
            two_factor_enabled: false,
            phone_verified: false,
            email_verified: false,
            failed_login_attempts: 0,
            locked_until: None,
        })
    }

    fn is_valid_email(email: &str) -> bool {
        !email.is_empty() && email.len() <= 255 && email.contains('@') && email.contains('.')
    }

    fn is_valid_phone(phone: &str) -> bool {
        let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
        digits.len() >= 7 && digits.len() <= 15
    }

    pub fn record_failed_login(&mut self) {
        self.failed_login_attempts += 1;
        if self.failed_login_attempts >= 5 {
            self.status = UserStatus::Locked;
            self.locked_until =
                Some(chrono::Utc::now() + chrono::Duration::minutes(30));
        }
    }

    pub fn reset_failed_login(&mut self) {
        self.failed_login_attempts = 0;
        self.locked_until = None;
        if self.status == UserStatus::Locked {
            self.status = UserStatus::Active;
        }
    }

    pub fn is_locked(&self) -> bool {
        self.status == UserStatus::Locked
    }

    pub fn can_login(&self) -> bool {
        self.status == UserStatus::Active && self.locked_until.map_or(true, |t| t < chrono::Utc::now())
    }

    pub fn enable_two_factor(&mut self) -> Result<(), UserDomainError> {
        if self.two_factor_enabled {
            return Err(UserDomainError::TwoFactorAlreadyEnabled);
        }
        self.two_factor_enabled = true;
        Ok(())
    }

    pub fn suspend(&mut self) {
        self.status = UserStatus::Suspended;
    }

    pub fn reactivate(&mut self) {
        self.status = UserStatus::Active;
        self.reset_failed_login();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_user() {
        let user = User::new(
            "admin@aeroxe.com".to_string(),
            "+919876543210".to_string(),
            "Admin".to_string(),
            Some(1),
        );
        assert!(user.is_ok());
        let user = user.unwrap();
        assert_eq!(user.status, UserStatus::Active);
        assert!(!user.is_locked());
    }

    #[test]
    fn test_failed_login_locks_account() {
        let mut user = User::new(
            "admin@aeroxe.com".to_string(),
            "+919876543210".to_string(),
            "Admin".to_string(),
            None,
        )
        .unwrap();

        for _ in 0..5 {
            user.record_failed_login();
        }
        assert!(user.is_locked());
        assert!(!user.can_login());
    }

    #[test]
    fn test_enable_two_factor() {
        let mut user = User::new(
            "admin@aeroxe.com".to_string(),
            "+919876543210".to_string(),
            "Admin".to_string(),
            None,
        )
        .unwrap();

        assert!(user.enable_two_factor().is_ok());
        assert!(user.two_factor_enabled);
        assert_eq!(
            user.enable_two_factor(),
            Err(UserDomainError::TwoFactorAlreadyEnabled)
        );
    }
}
