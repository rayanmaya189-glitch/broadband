//! GST (Goods and Services Tax) utility module for Indian tax calculations.
//!
//! Determines whether a transaction is interstate or intrastate based on
//! the supplier (branch) state and the recipient (customer) state, then
//! calculates the appropriate CGST/SGST or IGST amounts.
//!
//! GST Rules:
//! - Intrastate (same state): CGST + SGST (split equally, e.g., 9% + 9% = 18%)
//! - Interstate (different state): IGST (full rate, e.g., 18%)

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// Standard GST rates for ISP/internet services in India.
/// SAC code 998421: Telecommunications, broadcasting and information supply services
pub mod rates {
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    /// Standard GST rate for internet services (18%)
    pub const STANDARD_GST_RATE: Decimal = dec!(18.0);

    /// CGST rate for intrastate (half of total, 9%)
    pub const CGST_RATE: Decimal = dec!(9.0);

    /// SGST rate for intrastate (half of total, 9%)
    pub const SGST_RATE: Decimal = dec!(9.0);

    /// IGST rate for interstate (full rate, 18%)
    pub const IGST_RATE: Decimal = dec!(18.0);
}

/// Result of GST calculation for a line item or invoice.
#[derive(Debug, Clone)]
pub struct GstCalculation {
    pub taxable_amount: Decimal,
    pub cgst_amount: Decimal,
    pub sgst_amount: Decimal,
    pub igst_amount: Decimal,
    pub total_tax: Decimal,
    pub total_amount: Decimal,
    pub is_interstate: bool,
}

/// Determine if a transaction is interstate based on supplier and recipient states.
///
/// # Arguments
/// * `supplier_state` - The state of the branch/company (GSTIN state code or name)
/// * `recipient_state` - The state of the customer
///
/// # Returns
/// `true` if interstate (different states), `false` if intrastate (same state)
pub fn is_interstate(supplier_state: &str, recipient_state: &str) -> bool {
    normalize_state(supplier_state) != normalize_state(recipient_state)
}

/// Normalize state names/codes for comparison.
/// Handles both state names (e.g., "Maharashtra") and state codes (e.g., "27").
fn normalize_state(state: &str) -> String {
    let state = state.trim().to_lowercase();

    // Map state codes to names for consistent comparison
    match state.as_str() {
        "01" => "jammu & kashmir".to_string(),
        "02" => "himachal pradesh".to_string(),
        "03" => "punjab".to_string(),
        "04" => "chandigarh".to_string(),
        "05" => "uttarakhand".to_string(),
        "06" => "haryana".to_string(),
        "07" => "delhi".to_string(),
        "08" => "rajasthan".to_string(),
        "09" => "uttar pradesh".to_string(),
        "10" => "bihar".to_string(),
        "11" => "sikkim".to_string(),
        "12" => "arunachal pradesh".to_string(),
        "13" => "nagaland".to_string(),
        "14" => "manipur".to_string(),
        "15" => "mizoram".to_string(),
        "16" => "tripura".to_string(),
        "17" => "meghalaya".to_string(),
        "18" => "assam".to_string(),
        "19" => "west bengal".to_string(),
        "20" => "jharkhand".to_string(),
        "21" => "odisha".to_string(),
        "22" => "chhattisgarh".to_string(),
        "23" => "madhya pradesh".to_string(),
        "24" => "gujarat".to_string(),
        "25" => "daman & diu".to_string(),
        "26" => "dadra & nagar haveli".to_string(),
        "27" => "maharashtra".to_string(),
        "28" => "andhra pradesh (old)".to_string(),
        "29" => "karnataka".to_string(),
        "30" => "goa".to_string(),
        "31" => "lakshadweep".to_string(),
        "32" => "kerala".to_string(),
        "33" => "tamil nadu".to_string(),
        "34" => "puducherry".to_string(),
        "35" => "andaman & nicobar islands".to_string(),
        "36" => "telangana".to_string(),
        "37" => "andhra pradesh".to_string(),
        "38" => "ladakh".to_string(),
        _ => state, // Already a name or unknown code
    }
}

/// Extract state code from a GSTIN (first 2 digits after the state code position).
///
/// GSTIN format: 2-digit state code + 10-char PAN + 1 digit entity number + Z + check digit
/// Example: 27AAPFU0939F1ZV → state code 27 (Maharashtra)
pub fn extract_state_from_gstin(gstin: &str) -> Option<String> {
    let gstin = gstin.trim().to_uppercase();
    if gstin.len() != 15 {
        return None;
    }
    let state_code = &gstin[0..2];
    // Validate it's a numeric state code
    if state_code.parse::<u8>().is_ok() {
        Some(state_code.to_string())
    } else {
        None
    }
}

/// Calculate GST for a given taxable amount based on supplier and recipient states.
///
/// # Arguments
/// * `taxable_amount` - The amount on which GST is calculated (before tax)
/// * `supplier_state` - The state of the branch/company
/// * `recipient_state` - The state of the customer
/// * `gst_rate` - Optional custom GST rate (defaults to 18%)
///
/// # Returns
/// `GstCalculation` with detailed breakdown
pub fn calculate_gst(
    taxable_amount: Decimal,
    supplier_state: &str,
    recipient_state: &str,
    gst_rate: Option<Decimal>,
) -> GstCalculation {
    let rate = gst_rate.unwrap_or(rates::STANDARD_GST_RATE);
    let total_tax = taxable_amount * rate / dec!(100);

    let interstate = is_interstate(supplier_state, recipient_state);

    let (cgst, sgst, igst) = if interstate {
        // Interstate: IGST = full rate
        (Decimal::ZERO, Decimal::ZERO, total_tax)
    } else {
        // Intrastate: CGST = half, SGST = half
        let half = total_tax / dec!(2);
        (half, half, Decimal::ZERO)
    };

    GstCalculation {
        taxable_amount,
        cgst_amount: cgst,
        sgst_amount: sgst,
        igst_amount: igst,
        total_tax,
        total_amount: taxable_amount + total_tax,
        is_interstate: interstate,
    }
}

/// Calculate GST for a line item quantity.
///
/// # Arguments
/// * `quantity` - Number of units
/// * `unit_price` - Price per unit
/// * `supplier_state` - The state of the branch/company
/// * `recipient_state` - The state of the customer
/// * `gst_rate` - Optional custom GST rate
pub fn calculate_line_item_gst(
    quantity: Decimal,
    unit_price: Decimal,
    supplier_state: &str,
    recipient_state: &str,
    gst_rate: Option<Decimal>,
) -> GstCalculation {
    let taxable_amount = quantity * unit_price;
    calculate_gst(taxable_amount, supplier_state, recipient_state, gst_rate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_intrastate_same_state() {
        let result = calculate_gst(dec!(1000), "Maharashtra", "Maharashtra", None);
        assert!(!result.is_interstate);
        assert_eq!(result.taxable_amount, dec!(1000));
        assert_eq!(result.cgst_amount, dec!(90)); // 9% of 1000
        assert_eq!(result.sgst_amount, dec!(90)); // 9% of 1000
        assert_eq!(result.igst_amount, dec!(0));
        assert_eq!(result.total_tax, dec!(180));
        assert_eq!(result.total_amount, dec!(1180));
    }

    #[test]
    fn test_interstate_different_state() {
        let result = calculate_gst(dec!(1000), "Maharashtra", "Karnataka", None);
        assert!(result.is_interstate);
        assert_eq!(result.taxable_amount, dec!(1000));
        assert_eq!(result.cgst_amount, dec!(0));
        assert_eq!(result.sgst_amount, dec!(0));
        assert_eq!(result.igst_amount, dec!(180)); // 18% of 1000
        assert_eq!(result.total_tax, dec!(180));
        assert_eq!(result.total_amount, dec!(1180));
    }

    #[test]
    fn test_custom_gst_rate() {
        let result = calculate_gst(dec!(1000), "Maharashtra", "Maharashtra", Some(dec!(12)));
        assert!(!result.is_interstate);
        assert_eq!(result.cgst_amount, dec!(60)); // 6% of 1000
        assert_eq!(result.sgst_amount, dec!(60)); // 6% of 1000
        assert_eq!(result.total_tax, dec!(120));
    }

    #[test]
    fn test_state_code_normalization() {
        // State code 27 = Maharashtra
        let result = calculate_gst(dec!(1000), "27", "27", None);
        assert!(!result.is_interstate);

        // State code 29 = Karnataka
        let result = calculate_gst(dec!(1000), "27", "29", None);
        assert!(result.is_interstate);
    }

    #[test]
    fn test_gstin_state_extraction() {
        assert_eq!(extract_state_from_gstin("27AAPFU0939F1ZV"), Some("27".to_string()));
        assert_eq!(extract_state_from_gstin("29BBBGU0939F1ZV"), Some("29".to_string()));
        assert_eq!(extract_state_from_gstin("INVALID"), None);
    }

    #[test]
    fn test_line_item_gst() {
        let result = calculate_line_item_gst(
            dec!(2), dec!(500), "Maharashtra", "Maharashtra", None,
        );
        assert_eq!(result.taxable_amount, dec!(1000));
        assert_eq!(result.cgst_amount, dec!(90));
        assert_eq!(result.sgst_amount, dec!(90));
        assert_eq!(result.total_amount, dec!(1180));
    }

    #[test]
    fn test_zero_amount() {
        let result = calculate_gst(Decimal::ZERO, "Maharashtra", "Karnataka", None);
        assert!(result.is_interstate);
        assert_eq!(result.total_tax, Decimal::ZERO);
        assert_eq!(result.total_amount, Decimal::ZERO);
    }

    #[test]
    fn test_case_insensitive_state_names() {
        let result = calculate_gst(dec!(1000), "maharashtra", "MAHARASHTRA", None);
        assert!(!result.is_interstate);

        let result = calculate_gst(dec!(1000), "MAHARASHTRA", "karnataka", None);
        assert!(result.is_interstate);
    }
}
