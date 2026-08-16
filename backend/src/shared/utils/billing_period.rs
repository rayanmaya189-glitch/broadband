use chrono::Months;

/// Compute the calendar-month end of a billing period: `start + n` calendar
/// months, clamping to the last day of the resulting month. This is what
/// billing periods actually mean — `start + 30*n days` drifts across months
/// (e.g. Jan 15 + 1 month = Feb 14, not Feb 15) and short months (Jan 31)
/// would roll past the intended date.
pub fn period_end_months(start: chrono::NaiveDate, months: u32) -> chrono::NaiveDate {
    start
        .checked_add_months(Months::new(months))
        .unwrap_or_else(|| start + chrono::Duration::days(30 * months as i64))
}

/// Compute the calendar-month start of a billing period: `end - n` calendar
/// months, clamping to the last day of the resulting month.
pub fn period_start_months(end: chrono::NaiveDate, months: u32) -> chrono::NaiveDate {
    end.checked_sub_months(Months::new(months))
        .unwrap_or_else(|| end - chrono::Duration::days(30 * months as i64))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn same_day_after_one_month() {
        let start = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        assert_eq!(
            period_end_months(start, 1),
            NaiveDate::from_ymd_opt(2026, 2, 15).unwrap()
        );
    }

    #[test]
    fn end_of_month_clamps() {
        let start = NaiveDate::from_ymd_opt(2026, 1, 31).unwrap();
        assert_eq!(
            period_end_months(start, 1),
            NaiveDate::from_ymd_opt(2026, 2, 28).unwrap()
        );
    }

    #[test]
    fn leap_year_february() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
        assert_eq!(
            period_end_months(start, 1),
            NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()
        );
    }

    #[test]
    fn round_trip_start_equals_end() {
        let end = NaiveDate::from_ymd_opt(2026, 3, 15).unwrap();
        assert_eq!(
            period_start_months(end, 1),
            NaiveDate::from_ymd_opt(2026, 2, 15).unwrap()
        );
    }

    #[test]
    fn differs_from_fixed_30_day_arithmetic() {
        let start = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        let calendar = period_end_months(start, 1);
        let fixed30 = start + chrono::Duration::days(30);
        assert_ne!(
            calendar, fixed30,
            "calendar months must not be 30-day chunks"
        );
    }
}
