//! Verify KYC command handler.
//!
//! Handles KYC verification requests for compliance.

use crate::common::errors::app_error::AppError;
use crate::modules::compliance::domain::aggregates::kyc_verification::kyc_verification::{
    KycEvent, KycType, KycVerification,
};
use crate::modules::compliance::domain::rules::compliance_rules;

/// Command to submit KYC verification.
#[derive(Debug, Clone)]
pub struct SubmitKycCommand {
    pub customer_id: i64,
    pub verification_type: String,
    pub id_proof_type: String,
    pub id_proof_number: String,
    pub document_url: Option<String>,
}

/// Command to verify KYC.
#[derive(Debug, Clone)]
pub struct VerifyKycCommand {
    pub kyc_id: i64,
    pub verified_by: i64,
}

/// Command to reject KYC.
#[derive(Debug, Clone)]
pub struct RejectKycCommand {
    pub kyc_id: i64,
    pub reason: String,
}

/// Command handler for KYC submission.
pub struct SubmitKycHandler;

impl SubmitKycHandler {
    /// Handle the submit KYC command.
    pub fn handle(
        id: i64,
        command: SubmitKycCommand,
    ) -> Result<KycVerification, AppError> {
        // Validate input
        compliance_rules::validate_kyc_submission(
            &command.id_proof_type,
            &command.id_proof_number,
        )
        .map_err(|e| AppError::Validation(e.to_string()))?;

        let verification_type = KycType::from_str(&command.verification_type)
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let kyc = KycVerification::submit(
            id,
            command.customer_id,
            verification_type,
            Some(command.id_proof_type),
            Some(command.id_proof_number),
            command.document_url,
        )
        .map_err(|e| AppError::Validation(e.to_string()))?;

        Ok(kyc)
    }
}

/// Command handler for KYC verification.
pub struct VerifyKycHandler;

impl VerifyKycHandler {
    /// Handle the verify KYC command.
    pub fn handle(
        mut kyc: KycVerification,
        command: VerifyKycCommand,
    ) -> Result<KycEvent, AppError> {
        kyc.verify(command.verified_by)
            .map_err(|e| AppError::Validation(e.to_string()))
    }
}

/// Command handler for KYC rejection.
pub struct RejectKycHandler;

impl RejectKycHandler {
    /// Handle the reject KYC command.
    pub fn handle(
        mut kyc: KycVerification,
        command: RejectKycCommand,
    ) -> Result<KycEvent, AppError> {
        kyc.reject(&command.reason)
            .map_err(|e| AppError::Validation(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_submit_kyc_handler() {
        let command = SubmitKycCommand {
            customer_id: 100,
            verification_type: "identity".to_string(),
            id_proof_type: "aadhaar".to_string(),
            id_proof_number: "1234-5678-9012".to_string(),
            document_url: None,
        };

        let kyc = SubmitKycHandler::handle(1, command).unwrap();
        assert_eq!(kyc.customer_id, 100);
        assert_eq!(kyc.status.as_str(), "pending");
    }

    #[test]
    fn test_verify_kyc_handler() {
        let mut kyc = KycVerification::submit(
            1,
            100,
            KycType::Identity,
            None,
            None,
            None,
        ).unwrap();
        kyc.start_review().unwrap();

        let command = VerifyKycCommand {
            kyc_id: 1,
            verified_by: 1,
        };

        let event = VerifyKycHandler::handle(kyc, command).unwrap();
        match event {
            KycEvent::Verified { verified_by, .. } => {
                assert_eq!(verified_by, 1);
            }
            _ => panic!("Expected Verified event"),
        }
    }

    #[test]
    fn test_reject_kyc_handler() {
        let mut kyc = KycVerification::submit(
            1,
            100,
            KycType::Identity,
            None,
            None,
            None,
        ).unwrap();
        kyc.start_review().unwrap();

        let command = RejectKycCommand {
            kyc_id: 1,
            reason: "Invalid document".to_string(),
        };

        let event = RejectKycHandler::handle(kyc, command).unwrap();
        match event {
            KycEvent::Rejected { reason, .. } => {
                assert_eq!(reason, "Invalid document");
            }
            _ => panic!("Expected Rejected event"),
        }
    }
}
