//! Indian GST Tax Service
//! Implements CGST/SGST/IGST calculation, place-of-supply logic, and HSN/SAC codes.
//! Internet services SAC: 998421 (Telecommunications, broadcasting and information supply services)

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// HSN/SAC codes for ISP services
pub const SAC_INTERNET_ACCESS: &str = "998421";
pub const SAC_BROADBAND: &str = "998421";
pub const SAC_INSTALLATION: &str = "9954";
pub const SAC_EQUIPMENT_RENTAL: &str = "998421";

/// GST rates
pub const CGST_RATE: Decimal = dec!(9.0); // 9%
pub const SGST_RATE: Decimal = dec!(9.0); // 9%
pub const IGST_RATE: Decimal = dec!(18.0); // 18% (sum of CGST + SGST)

/// Indian state codes for GST (state code → state name)
pub fn state_code_from_name(state: &str) -> Option<&'static str> {
    match state.to_lowercase().as_str() {
        "andhra pradesh" => Some("37"),
        "arunachal pradesh" => Some("12"),
        "assam" => Some("18"),
        "bihar" => Some("10"),
        "chhattisgarh" => Some("22"),
        "goa" => Some("30"),
        "gujarat" => Some("24"),
        "haryana" => Some("06"),
        "himachal pradesh" => Some("02"),
        "jharkhand" => Some("20"),
        "karnataka" => Some("29"),
        "kerala" => Some("32"),
        "madhya pradesh" => Some("23"),
        "maharashtra" => Some("27"),
        "manipur" => Some("14"),
        "meghalaya" => Some("17"),
        "mizoram" => Some("15"),
        "nagaland" => Some("13"),
        "odisha" => Some("21"),
        "punjab" => Some("03"),
        "rajasthan" => Some("08"),
        "sikkim" => Some("11"),
        "tamil nadu" => Some("33"),
        "telangana" => Some("36"),
        "tripura" => Some("16"),
        "uttar pradesh" => Some("09"),
        "uttarakhand" => Some("05"),
        "west bengal" => Some("19"),
        "delhi" => Some("07"),
        "jammu and kashmir" => Some("01"),
        "ladakh" => Some("38"),
        "puducherry" => Some("34"),
        "chandigarh" => Some("04"),
        "andaman and nicobar islands" => Some("35"),
        "dadra and nagar haveli and daman and diu" => Some("26"),
        "lakshadweep" => Some("31"),
        _ => None,
    }
}

/// Determine if a transaction is intra-state or inter-state
pub fn is_intra_state(supplier_state: &str, place_of_supply: &str) -> bool {
    supplier_state.to_lowercase() == place_of_supply.to_lowercase()
}

/// Calculate GST on a taxable amount
pub fn calculate_gst_breakdown(taxable_amount: Decimal, is_intra_state: bool) -> GstBreakdown {
    let cgst_rate = CGST_RATE / dec!(100);
    let sgst_rate = SGST_RATE / dec!(100);
    let igst_rate = IGST_RATE / dec!(100);

    let cgst_amount = if is_intra_state {
        (taxable_amount * cgst_rate).round_dp(2)
    } else {
        Decimal::ZERO
    };
    let sgst_amount = if is_intra_state {
        (taxable_amount * sgst_rate).round_dp(2)
    } else {
        Decimal::ZERO
    };
    let igst_amount = if is_intra_state {
        Decimal::ZERO
    } else {
        (taxable_amount * igst_rate).round_dp(2)
    };
    let total_tax = cgst_amount + sgst_amount + igst_amount;

    GstBreakdown {
        taxable_amount,
        cgst_rate: CGST_RATE,
        sgst_rate: SGST_RATE,
        igst_rate: if is_intra_state {
            Decimal::ZERO
        } else {
            IGST_RATE
        },
        cgst_amount,
        sgst_amount,
        igst_amount,
        total_tax,
    }
}

/// Calculate late fee with GST
pub fn calculate_late_fee_with_gst(late_fee_base: Decimal, is_intra_state: bool) -> LateFeeGst {
    let gst = calculate_gst_breakdown(late_fee_base, is_intra_state);
    LateFeeGst {
        late_fee_subtotal: late_fee_base,
        gst,
    }
}

/// GST breakdown for a taxable amount
#[derive(Debug, Clone)]
pub struct GstBreakdown {
    pub taxable_amount: Decimal,
    pub cgst_rate: Decimal,
    pub sgst_rate: Decimal,
    pub igst_rate: Decimal,
    pub cgst_amount: Decimal,
    pub sgst_amount: Decimal,
    pub igst_amount: Decimal,
    pub total_tax: Decimal,
}

/// Late fee with GST breakdown
#[derive(Debug, Clone)]
pub struct LateFeeGst {
    pub late_fee_subtotal: Decimal,
    pub gst: GstBreakdown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intra_state_gst() {
        let result = calculate_gst_breakdown(dec!(1000), true);
        assert_eq!(result.cgst_amount, dec!(90));
        assert_eq!(result.sgst_amount, dec!(90));
        assert_eq!(result.igst_amount, Decimal::ZERO);
        assert_eq!(result.total_tax, dec!(180));
    }

    #[test]
    fn test_inter_state_gst() {
        let result = calculate_gst_breakdown(dec!(1000), false);
        assert_eq!(result.cgst_amount, Decimal::ZERO);
        assert_eq!(result.sgst_amount, Decimal::ZERO);
        assert_eq!(result.igst_amount, dec!(180));
        assert_eq!(result.total_tax, dec!(180));
    }

    #[test]
    fn test_place_of_supply() {
        assert!(is_intra_state("Maharashtra", "Maharashtra"));
        assert!(!is_intra_state("Maharashtra", "Karnataka"));
    }

    #[test]
    fn test_late_fee_with_gst() {
        let result = calculate_late_fee_with_gst(dec!(20), true);
        assert_eq!(result.late_fee_subtotal, dec!(20));
        assert_eq!(result.gst.cgst_amount, dec!(1.80));
        assert_eq!(result.gst.sgst_amount, dec!(1.80));
        assert_eq!(result.gst.total_tax, dec!(3.60));
    }
}
