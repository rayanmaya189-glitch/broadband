use chrono::{DateTime, Datelike, Duration, TimeZone, Timelike, Utc};

/// Parsed schedule that can calculate the next run time.
#[derive(Debug, Clone)]
pub enum Schedule {
    /// Standard 5-field cron: minute hour day_of_month month day_of_week
    Cron {
        minutes: Vec<u32>,
        hours: Vec<u32>,
        days_of_month: Vec<u32>,
        months: Vec<u32>,
        days_of_week: Vec<u32>,
        /// Whether day_of_month was wildcard `*`
        dom_is_wildcard: bool,
        /// Whether day_of_week was wildcard `*`
        dow_is_wildcard: bool,
    },
    /// Fixed interval in seconds
    Interval { seconds: u64 },
    /// One-time job (no next run)
    OneTime,
    /// Never runs
    Disabled,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScheduleParseError {
    InvalidCronFormat(String),
    InvalidInterval(String),
    UnknownJobType(String),
}

impl std::fmt::Display for ScheduleParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCronFormat(msg) => write!(f, "Invalid cron format: {}", msg),
            Self::InvalidInterval(msg) => write!(f, "Invalid interval: {}", msg),
            Self::UnknownJobType(msg) => write!(f, "Unknown job type: {}", msg),
        }
    }
}

impl std::error::Error for ScheduleParseError {}

impl Schedule {
    pub fn parse(job_type: &str, schedule: &str) -> Result<Self, ScheduleParseError> {
        match job_type {
            "cron" => Self::parse_cron(schedule),
            "interval" => Self::parse_interval(schedule),
            "one_time" => Ok(Schedule::OneTime),
            _ => Err(ScheduleParseError::UnknownJobType(job_type.to_string())),
        }
    }

    pub fn parse_cron(expr: &str) -> Result<Self, ScheduleParseError> {
        let parts: Vec<&str> = expr.split_whitespace().collect();
        if parts.len() != 5 {
            return Err(ScheduleParseError::InvalidCronFormat(format!(
                "Expected 5 fields, got {}",
                parts.len()
            )));
        }
        let minutes = parse_cron_field(parts[0], 0, 59, "minute")?;
        let hours = parse_cron_field(parts[1], 0, 23, "hour")?;
        let (days_of_month, dom_is_wildcard) =
            parse_cron_field_with_wildcard(parts[2], 1, 31, "day_of_month")?;
        let months = parse_cron_field(parts[3], 1, 12, "month")?;
        let (days_of_week, dow_is_wildcard) =
            parse_cron_field_with_wildcard(parts[4], 0, 6, "day_of_week")?;

        Ok(Schedule::Cron {
            minutes,
            hours,
            days_of_month,
            months,
            days_of_week,
            dom_is_wildcard,
            dow_is_wildcard,
        })
    }

    pub fn parse_interval(expr: &str) -> Result<Self, ScheduleParseError> {
        let expr = expr.trim();
        if let Some(s) = expr.strip_suffix('h') {
            let hours: u64 = s.trim().parse().map_err(|_| {
                ScheduleParseError::InvalidInterval(format!("Invalid hours: {}", expr))
            })?;
            return Ok(Schedule::Interval {
                seconds: hours * 3600,
            });
        }
        if let Some(s) = expr.strip_suffix('m') {
            let minutes: u64 = s.trim().parse().map_err(|_| {
                ScheduleParseError::InvalidInterval(format!("Invalid minutes: {}", expr))
            })?;
            return Ok(Schedule::Interval {
                seconds: minutes * 60,
            });
        }
        if let Some(s) = expr.strip_suffix('s') {
            let seconds: u64 = s.trim().parse().map_err(|_| {
                ScheduleParseError::InvalidInterval(format!("Invalid seconds: {}", expr))
            })?;
            return Ok(Schedule::Interval { seconds });
        }
        let seconds: u64 = expr.parse().map_err(|_| {
            ScheduleParseError::InvalidInterval(format!("Invalid interval: {}", expr))
        })?;
        Ok(Schedule::Interval { seconds })
    }

    pub fn next_run_after(&self, after: DateTime<Utc>) -> Option<DateTime<Utc>> {
        match self {
            Schedule::Cron {
                minutes,
                hours,
                days_of_month,
                months,
                days_of_week,
                dom_is_wildcard,
                dow_is_wildcard,
            } => next_cron_run(
                after,
                minutes,
                hours,
                days_of_month,
                months,
                days_of_week,
                *dom_is_wildcard,
                *dow_is_wildcard,
            ),
            Schedule::Interval { seconds } => {
                if *seconds == 0 {
                    return None;
                }
                Some(after + Duration::seconds(*seconds as i64))
            }
            Schedule::OneTime | Schedule::Disabled => None,
        }
    }
}

// ── Cron field parsing ──

fn parse_cron_field(
    field: &str,
    min: u32,
    max: u32,
    name: &str,
) -> Result<Vec<u32>, ScheduleParseError> {
    let (_values, _is_wildcard) = parse_cron_field_inner(field, min, max, name)?;
    Ok(_values)
}

fn parse_cron_field_with_wildcard(
    field: &str,
    min: u32,
    max: u32,
    name: &str,
) -> Result<(Vec<u32>, bool), ScheduleParseError> {
    parse_cron_field_inner(field, min, max, name)
}

fn parse_cron_field_inner(
    field: &str,
    min: u32,
    max: u32,
    name: &str,
) -> Result<(Vec<u32>, bool), ScheduleParseError> {
    let mut values = Vec::new();
    let is_wildcard = field.trim() == "*";

    for part in field.split(',') {
        let part = part.trim();
        if part == "*" {
            for i in min..=max {
                values.push(i);
            }
        } else if let Some((start_str, end_str)) = part.split_once('-') {
            let start: u32 = start_str.trim().parse().map_err(|_| {
                ScheduleParseError::InvalidCronFormat(format!(
                    "Invalid {} range start: {}",
                    name, start_str
                ))
            })?;
            let end: u32 = end_str.trim().parse().map_err(|_| {
                ScheduleParseError::InvalidCronFormat(format!(
                    "Invalid {} range end: {}",
                    name, end_str
                ))
            })?;
            if start < min || end > max || start > end {
                return Err(ScheduleParseError::InvalidCronFormat(format!(
                    "{} range {}-{} out of bounds [{}/{}]",
                    name, start, end, min, max
                )));
            }
            for i in start..=end {
                values.push(i);
            }
        } else if let Some((base_str, step_str)) = part.split_once('/') {
            let step: u32 = step_str.trim().parse().map_err(|_| {
                ScheduleParseError::InvalidCronFormat(format!(
                    "Invalid {} step: {}",
                    name, step_str
                ))
            })?;
            if step == 0 {
                return Err(ScheduleParseError::InvalidCronFormat(format!(
                    "{} step cannot be 0",
                    name
                )));
            }
            if base_str == "*" {
                let mut i = min;
                while i <= max {
                    values.push(i);
                    i += step;
                }
            } else {
                let start: u32 = base_str.trim().parse().map_err(|_| {
                    ScheduleParseError::InvalidCronFormat(format!(
                        "Invalid {} step base: {}",
                        name, base_str
                    ))
                })?;
                let mut i = start;
                while i <= max {
                    values.push(i);
                    i += step;
                }
            }
        } else {
            let value: u32 = part.parse().map_err(|_| {
                ScheduleParseError::InvalidCronFormat(format!("Invalid {} value: {}", name, part))
            })?;
            if value < min || value > max {
                return Err(ScheduleParseError::InvalidCronFormat(format!(
                    "{} value {} out of bounds [{}/{}]",
                    name, value, min, max
                )));
            }
            values.push(value);
        }
    }
    values.sort_unstable();
    values.dedup();
    Ok((values, is_wildcard))
}

// ── Next cron run calculation ──

fn next_cron_run(
    after: DateTime<Utc>,
    minutes: &[u32],
    hours: &[u32],
    days_of_month: &[u32],
    months: &[u32],
    days_of_week: &[u32],
    dom_is_wildcard: bool,
    dow_is_wildcard: bool,
) -> Option<DateTime<Utc>> {
    let mut candidate = after + Duration::minutes(1);
    candidate = candidate.with_second(0)?.with_nanosecond(0)?;

    let limit = after + Duration::days(366 * 4);

    while candidate < limit {
        // Check month
        if !months.contains(&candidate.month()) {
            candidate = advance_to_next_month(candidate, months)?;
            continue;
        }

        // Check day (standard cron: AND when both non-wildcard, OR otherwise)
        let dom_match = dom_is_wildcard || days_of_month.contains(&candidate.day());
        // Cron convention: 0=Sun,1=Mon,...,6=Sat. number_from_monday() gives 1=Mon,...,7=Sun.
        // Convert: cron 0 (Sun) → 7, cron 1-6 stay same.
        let candidate_dow = {
            let n = candidate.weekday().number_from_monday();
            if n == 7 {
                0
            } else {
                n
            }
        };
        let dow_match = dow_is_wildcard || days_of_week.contains(&candidate_dow);
        let day_matches = if !dom_is_wildcard && !dow_is_wildcard {
            dom_match && dow_match
        } else if !dom_is_wildcard {
            dom_match
        } else {
            dow_match
        };

        if !day_matches {
            candidate = (candidate + Duration::days(1))
                .with_hour(0)?
                .with_minute(0)?;
            continue;
        }

        // Check hour
        if !hours.contains(&candidate.hour()) {
            candidate = advance_to_next_hour(candidate, hours)?;
            continue;
        }

        // Check minute
        if !minutes.contains(&candidate.minute()) {
            candidate = advance_to_next_minute(candidate, minutes)?;
            continue;
        }

        return Some(candidate);
    }
    None
}

// ── Advance helpers ──

fn advance_to_next_month(current: DateTime<Utc>, months: &[u32]) -> Option<DateTime<Utc>> {
    let mut year = current.year();
    let mut month = current.month();
    for _ in 0..13 {
        month += 1;
        if month > 12 {
            month = 1;
            year += 1;
        }
        if months.contains(&month) {
            return Utc.with_ymd_and_hms(year, month, 1, 0, 0, 0).single();
        }
    }
    Utc.with_ymd_and_hms(year, month, 1, 0, 0, 0).single()
}

fn advance_to_next_hour(current: DateTime<Utc>, hours: &[u32]) -> Option<DateTime<Utc>> {
    let candidate = current.with_minute(0)?.with_second(0)?;
    let cur_hour = candidate.hour();
    for &h in hours {
        if h > cur_hour {
            return candidate.with_hour(h);
        }
    }
    let next_day = (candidate + Duration::days(1))
        .with_hour(0)?
        .with_minute(0)?;
    next_day.with_hour(*hours.first()?)
}

fn advance_to_next_minute(current: DateTime<Utc>, minutes: &[u32]) -> Option<DateTime<Utc>> {
    let candidate = current.with_second(0)?;
    let cur_min = candidate.minute();
    for &m in minutes {
        if m > cur_min {
            return candidate.with_minute(m);
        }
    }
    let next_hour = candidate + Duration::hours(1);
    next_hour.with_minute(*minutes.first()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_parse_cron_hourly() {
        let schedule = Schedule::parse_cron("0 * * * *").unwrap();
        let after = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
        let next = schedule.next_run_after(after).unwrap();
        assert_eq!(next.hour(), 11);
        assert_eq!(next.minute(), 0);
    }

    #[test]
    fn test_parse_cron_daily_1am() {
        let schedule = Schedule::parse_cron("0 1 * * *").unwrap();
        let after = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
        let next = schedule.next_run_after(after).unwrap();
        assert_eq!(next.hour(), 1);
        assert_eq!(next.minute(), 0);
        assert_eq!(next.day(), 16);
    }

    #[test]
    fn test_parse_cron_every_5_minutes() {
        let schedule = Schedule::parse_cron("*/5 * * * *").unwrap();
        let after = Utc.with_ymd_and_hms(2024, 1, 15, 10, 32, 0).unwrap();
        let next = schedule.next_run_after(after).unwrap();
        assert_eq!(next.hour(), 10);
        assert_eq!(next.minute(), 35);
    }

    #[test]
    fn test_parse_cron_invalid() {
        assert!(Schedule::parse_cron("invalid").is_err());
        assert!(Schedule::parse_cron("* * *").is_err());
    }

    #[test]
    fn test_parse_interval_seconds() {
        let schedule = Schedule::parse_interval("300").unwrap();
        let after = Utc.with_ymd_and_hms(2024, 1, 15, 10, 0, 0).unwrap();
        let next = schedule.next_run_after(after).unwrap();
        assert_eq!(next, Utc.with_ymd_and_hms(2024, 1, 15, 10, 5, 0).unwrap());
    }

    #[test]
    fn test_parse_interval_minutes() {
        let schedule = Schedule::parse_interval("5m").unwrap();
        let after = Utc.with_ymd_and_hms(2024, 1, 15, 10, 0, 0).unwrap();
        let next = schedule.next_run_after(after).unwrap();
        assert_eq!(next, Utc.with_ymd_and_hms(2024, 1, 15, 10, 5, 0).unwrap());
    }

    #[test]
    fn test_parse_interval_hours() {
        let schedule = Schedule::parse_interval("2h").unwrap();
        let after = Utc.with_ymd_and_hms(2024, 1, 15, 10, 0, 0).unwrap();
        let next = schedule.next_run_after(after).unwrap();
        assert_eq!(next, Utc.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap());
    }

    #[test]
    fn test_one_time() {
        let schedule = Schedule::OneTime;
        let after = Utc::now();
        assert!(schedule.next_run_after(after).is_none());
    }

    #[test]
    fn test_parse_dispatch() {
        let s = Schedule::parse("cron", "0 * * * *").unwrap();
        assert!(matches!(s, Schedule::Cron { .. }));
        let s = Schedule::parse("interval", "300").unwrap();
        assert!(matches!(s, Schedule::Interval { .. }));
        let s = Schedule::parse("one_time", "now").unwrap();
        assert!(matches!(s, Schedule::OneTime));
    }

    #[test]
    fn test_cron_monthly_first_day() {
        let schedule = Schedule::parse_cron("0 0 1 * *").unwrap();
        let after = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
        let next = schedule.next_run_after(after).unwrap();
        // After Jan 15, next 1st-of-month is Feb 1
        assert_eq!(next.day(), 1);
        assert!(
            next.month() > 1,
            "Expected month > January, got {}",
            next.month()
        );
    }

    #[test]
    fn test_cron_specific_day_and_weekday_and() {
        // "0 2 15 * 1" = 2:00 AM on 15th AND Monday
        let schedule = Schedule::parse_cron("0 2 15 * 1").unwrap();
        let after = Utc.with_ymd_and_hms(2024, 1, 8, 0, 0, 0).unwrap(); // Mon Jan 8
        let next = schedule.next_run_after(after).unwrap();
        // Jan 15 is a Monday, so should match
        assert_eq!(next.day(), 15);
        assert_eq!(next.month(), 1);
        assert_eq!(next.hour(), 2);
    }

    #[test]
    fn test_cron_specific_day_and_weekday_no_match() {
        // "0 2 15 * 3" = 2:00 AM on 15th AND Wednesday
        let schedule = Schedule::parse_cron("0 2 15 * 3").unwrap();
        let after = Utc.with_ymd_and_hms(2024, 1, 8, 0, 0, 0).unwrap(); // Mon Jan 8
        let next = schedule.next_run_after(after).unwrap();
        // Jan 15 is Monday (not Wed), so should skip to Feb 14 (Wed)
        // Actually Feb 14 2024 is a Wednesday
        assert_eq!(next.month(), 2);
        assert_eq!(next.day(), 14);
    }
}
