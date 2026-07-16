/// Accounting business rules and invariants
pub struct AccountingRules;

impl AccountingRules {
    /// Valid account types
    pub const VALID_ACCOUNT_TYPES: &[&str] = &["asset", "liability", "equity", "revenue", "expense"];

    /// Account code ranges by type
    pub const ASSET_RANGE: (i32, i32) = (1000, 1999);
    pub const LIABILITY_RANGE: (i32, i32) = (2000, 2999);
    pub const EQUITY_RANGE: (i32, i32) = (3000, 3999);
    pub const REVENUE_RANGE: (i32, i32) = (4000, 4999);
    pub const EXPENSE_RANGE: (i32, i32) = (5000, 5999);

    /// Check if account code is valid for its type
    pub fn is_valid_code_for_type(code: i32, account_type: &str) -> bool {
        let range = match account_type {
            "asset" => Self::ASSET_RANGE,
            "liability" => Self::LIABILITY_RANGE,
            "equity" => Self::EQUITY_RANGE,
            "revenue" => Self::REVENUE_RANGE,
            "expense" => Self::EXPENSE_RANGE,
            _ => return false,
        };
        code >= range.0 && code <= range.1
    }

    /// GST rate for Maharashtra (CGST + SGST)
    pub const CGST_RATE: f64 = 9.0;
    pub const SGST_RATE: f64 = 9.0;
    pub const IGST_RATE: f64 = 18.0;
}
