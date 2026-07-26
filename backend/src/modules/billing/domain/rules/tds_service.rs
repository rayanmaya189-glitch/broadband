//! Indian TDS (Tax Deducted at Source) Service
//! Implements TDS calculation per Income Tax Act sections:
//! - 194C: Contractor/sub-contractor payments (>₹30,000 single / ₹1,00,000 aggregate)
//! - 194J: Professional/technical fees (>₹30,000)
//! - 194H: Commission/brokerage (>₹15,000)
//! - 194A: Interest payments (>₹40,000 / ₹50,000 for seniors)

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// TDS sections applicable to ISP operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TdsSection {
    /// Section 194C - Contractor/sub-contractor
    Contractor,
    /// Section 194J - Professional/technical fees
    Professional,
    /// Section 194H - Commission/brokerage
    Commission,
    /// Section 194A - Interest
    Interest,
}

impl TdsSection {
    /// Minimum threshold for TDS deduction (single payment)
    pub fn threshold_single(&self) -> Decimal {
        match self {
            Self::Contractor => dec!(30000),
            Self::Professional => dec!(30000),
            Self::Commission => dec!(15000),
            Self::Interest => dec!(40000),
        }
    }

    /// Aggregate threshold per financial year
    pub fn threshold_aggregate(&self) -> Decimal {
        match self {
            Self::Contractor => dec!(100000),
            Self::Professional => dec!(30000),
            Self::Commission => dec!(15000),
            Self::Interest => dec!(50000),
        }
    }

    /// TDS rate (for non-corporate deductees without PAN: higher rate applies)
    pub fn rate_without_pan(&self) -> Decimal {
        match self {
            Self::Contractor => dec!(0.05),    // 5% (double of 2%)
            Self::Professional => dec!(0.20),  // 20% (double of 10%)
            Self::Commission => dec!(0.10),    // 10% (double of 5%)
            Self::Interest => dec!(0.20),      // 20% (double of 10%)
        }
    }
}

/// TDS deduction result
#[derive(Debug, Clone)]
pub struct TdsDeduction {
    pub section: &'static str,
    pub gross_amount: Decimal,
    pub threshold: Decimal,
    pub tds_rate: Decimal,
    pub tds_amount: Decimal,
    pub net_payable: Decimal,
    pub pan_available: bool,
    pub deductee_pan: Option<String>,
    pub requires_filing: bool,
}

/// Calculate TDS on a vendor payment
pub fn calculate_tds(
    gross_amount: Decimal,
    section: TdsSection,
    deductee_pan: Option<&str>,
    aggregate_ytd: Decimal,
) -> TdsDeduction {
    let pan_available = deductee_pan.is_some() && !deductee_pan.unwrap_or("").is_empty()
        && deductee_pan.unwrap_or("").len() == 10;

    // Check if aggregate threshold is exceeded
    let effective_threshold = if aggregate_ytd > Decimal::ZERO {
        // Already crossed aggregate threshold from previous payments
        dec!(0) // No threshold; deduct on full amount
    } else {
        section.threshold_single()
    };

    let requires_filing = gross_amount >= effective_threshold;

    let tds_rate = if pan_available {
        match section {
            TdsSection::Contractor => dec!(0.02),     // 2%
            TdsSection::Professional => dec!(0.10),    // 10%
            TdsSection::Commission => dec!(0.05),      // 5%
            TdsSection::Interest => dec!(0.10),        // 10%
        }
    } else {
        section.rate_without_pan()
    };

    let tds_amount = if requires_filing {
        (gross_amount * tds_rate).round_dp(2)
    } else {
        Decimal::ZERO
    };

    let net_payable = gross_amount - tds_amount;

    TdsDeduction {
        section: match section {
            TdsSection::Contractor => "194C",
            TdsSection::Professional => "194J",
            TdsSection::Commission => "194H",
            TdsSection::Interest => "194A",
        },
        gross_amount,
        threshold: effective_threshold,
        tds_rate,
        tds_amount,
        net_payable,
        pan_available,
        deductee_pan: deductee_pan.map(|s| s.to_string()),
        requires_filing,
    }
}

/// TDS return data for quarterly filing (Form 26Q/27Q)
#[derive(Debug, Clone, serde::Serialize)]
pub struct TdsReturnData {
    pub quarter: String,
    pub financial_year: String,
    pub deductor_pan: String,
    pub total_deducted: Decimal,
    pub total_paid: Decimal,
    pub entries: Vec<TdsReturnEntry>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TdsReturnEntry {
    pub deductee_name: String,
    pub deductee_pan: String,
    pub section: String,
    pub payment_date: String,
    pub gross_amount: Decimal,
    pub tds_rate: Decimal,
    pub tds_amount: Decimal,
    pub tds_deposited: Decimal,
    pub status: String,
}

/// Determine the Indian financial quarter (Q1=Apr-Jun, Q2=Jul-Sep, Q3=Oct-Dec, Q4=Jan-Mar)
pub fn get_financial_quarter(month: u32) -> &'static str {
    match month {
        4..=6 => "Q1",
        7..=9 => "Q2",
        10..=12 => "Q3",
        1..=3 => "Q4",
        _ => "Q1",
    }
}

/// Determine financial year start year from a calendar month
pub fn get_financial_year_start(month: u32, year: i32) -> i32 {
    if month >= 4 { year } else { year - 1 }
}

/// Generate a quarterly TDS return summary (Form 26Q format)
pub fn generate_quarterly_tds_return(
    entries: Vec<TdsReturnEntry>,
    quarter: &str,
    financial_year: &str,
    deductor_pan: &str,
) -> TdsReturnData {
    let total_deducted: Decimal = entries.iter().map(|e| e.tds_amount).sum();
    let total_paid: Decimal = entries.iter().map(|e| e.tds_deposited).sum();

    TdsReturnData {
        quarter: quarter.to_string(),
        financial_year: financial_year.to_string(),
        deductor_pan: deductor_pan.to_string(),
        total_deducted,
        total_paid,
        entries,
    }
}

/// Convert TDS return data to CSV for quarterly filing
pub fn tds_return_to_csv(data: &TdsReturnData) -> String {
    let mut csv = String::with_capacity(1024);
    csv.push_str("Deductee Name,Deductee PAN,Section,Payment Date,Gross Amount,TDS Rate,TDS Amount,TDS Deposited,Status\n");
    for entry in &data.entries {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{}\n",
            entry.deductee_name,
            entry.deductee_pan,
            entry.section,
            entry.payment_date,
            entry.gross_amount,
            entry.tds_rate,
            entry.tds_amount,
            entry.tds_deposited,
            entry.status,
        ));
    }
    csv.push_str(&format!(
        "\nTotal,,,{},{},{},,,\n",
        data.total_deducted, "", data.total_paid,
    ));
    csv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contractor_tds_with_pan() {
        let result = calculate_tds(dec!(100000), TdsSection::Contractor, Some("ABCDE1234F"), Decimal::ZERO);
        assert_eq!(result.tds_rate, dec!(0.02));
        assert_eq!(result.tds_amount, dec!(2000));
        assert_eq!(result.net_payable, dec!(98000));
        assert!(result.pan_available);
    }

    #[test]
    fn test_contractor_tds_without_pan() {
        let result = calculate_tds(dec!(100000), TdsSection::Contractor, None, Decimal::ZERO);
        assert_eq!(result.tds_rate, dec!(0.05));
        assert_eq!(result.tds_amount, dec!(5000));
        assert_eq!(result.net_payable, dec!(95000));
        assert!(!result.pan_available);
    }

    #[test]
    fn test_professional_tds_below_threshold() {
        let result = calculate_tds(dec!(20000), TdsSection::Professional, Some("ABCDE1234F"), Decimal::ZERO);
        assert_eq!(result.tds_amount, Decimal::ZERO);
        assert!(!result.requires_filing);
    }

    #[test]
    fn test_professional_tds_above_threshold() {
        let result = calculate_tds(dec!(50000), TdsSection::Professional, Some("ABCDE1234F"), Decimal::ZERO);
        assert_eq!(result.tds_rate, dec!(0.10));
        assert_eq!(result.tds_amount, dec!(5000));
        assert_eq!(result.net_payable, dec!(45000));
    }

    #[test]
    fn test_aggregate_threshold_crossed() {
        // Previous YTD is ₹90,000, now paying ₹25,000 → already past ₹100,000 aggregate
        let result = calculate_tds(dec!(25000), TdsSection::Contractor, Some("ABCDE1234F"), dec!(90000));
        assert_eq!(result.tds_amount, dec!(500)); // 2% of ₹25,000
        assert!(result.requires_filing);
    }
}
