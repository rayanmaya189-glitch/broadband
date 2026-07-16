use crate::modules::coverage::domain::value_objects::{CoverageAreaId, CoverageStatus};

/// CoverageArea aggregate root - represents a geographic service area
#[derive(Debug, Clone)]
pub struct CoverageArea {
    pub id: CoverageAreaId,
    pub branch_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub area_type: String,
    pub fiber_available: bool,
    pub estimated_installation_days: i32,
    pub max_customers: Option<i32>,
    pub current_customers: i32,
    pub is_active: bool,
    pub status: CoverageStatus,
}

/// Domain errors for CoverageArea aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum CoverageDomainError {
    CoverageAreaNotFound(i64),
    CapacityExceeded,
    InvalidAreaType,
}

impl std::fmt::Display for CoverageDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CoverageAreaNotFound(id) => write!(f, "Coverage area {} not found", id),
            Self::CapacityExceeded => write!(f, "Coverage area has reached maximum customer capacity"),
            Self::InvalidAreaType => write!(f, "Invalid area type"),
        }
    }
}

impl std::error::Error for CoverageDomainError {}

impl CoverageArea {
    pub fn new(branch_id: i64, name: String, area_type: String) -> Result<Self, CoverageDomainError> {
        let valid_types = ["polygon", "circle", "pincode"];
        if !valid_types.contains(&area_type.as_str()) {
            return Err(CoverageDomainError::InvalidAreaType);
        }
        Ok(Self {
            id: CoverageAreaId::new(0),
            branch_id,
            name,
            description: None,
            area_type,
            fiber_available: true,
            estimated_installation_days: 3,
            max_customers: None,
            current_customers: 0,
            is_active: true,
            status: CoverageStatus::Active,
        })
    }

    pub fn can_accept_customer(&self) -> bool {
        if !self.is_active { return false; }
        match self.max_customers {
            Some(max) => self.current_customers < max,
            None => true,
        }
    }

    pub fn add_customer(&mut self) -> Result<(), CoverageDomainError> {
        if !self.can_accept_customer() {
            return Err(CoverageDomainError::CapacityExceeded);
        }
        self.current_customers += 1;
        Ok(())
    }

    pub fn remove_customer(&mut self) {
        self.current_customers = (self.current_customers - 1).max(0);
    }

    pub fn utilization_percent(&self) -> f64 {
        match self.max_customers {
            Some(max) if max > 0 => (self.current_customers as f64 / max as f64) * 100.0,
            _ => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_coverage_area() {
        let area = CoverageArea::new(1, "City Center".to_string(), "polygon".to_string());
        assert!(area.is_ok());
        let area = area.unwrap();
        assert!(area.can_accept_customer());
    }

    #[test]
    fn test_capacity_check() {
        let mut area = CoverageArea::new(1, "Area".to_string(), "circle".to_string()).unwrap();
        area.max_customers = Some(2);
        assert!(area.can_accept_customer());
        area.add_customer().unwrap();
        area.add_customer().unwrap();
        assert!(!area.can_accept_customer());
        assert_eq!(area.add_customer(), Err(CoverageDomainError::CapacityExceeded));
    }
}
