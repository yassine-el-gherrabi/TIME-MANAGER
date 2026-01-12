//! DateTime helper utilities for consistent date/time handling across the application.
//!
//! This module provides helper functions for common datetime operations,
//! eliminating scattered unwrap() calls and ensuring consistent handling
//! of date/time boundaries.

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

use crate::error::AppError;

/// Returns midnight (00:00:00) as a NaiveTime.
/// This is a statically valid time and will never fail.
#[inline]
pub fn midnight() -> NaiveTime {
    NaiveTime::from_hms_opt(0, 0, 0).expect("midnight (00:00:00) is always valid")
}

/// Returns end of day time (23:59:59) as a NaiveTime.
/// This is a statically valid time and will never fail.
#[inline]
pub fn end_of_day_time() -> NaiveTime {
    NaiveTime::from_hms_opt(23, 59, 59).expect("23:59:59 is always valid")
}

/// Convert a NaiveDate to DateTime<Utc> at the start of day (00:00:00 UTC).
///
/// # Example
/// ```
/// use chrono::NaiveDate;
/// use timemanager_backend::utils::datetime::start_of_day;
///
/// let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
/// let dt = start_of_day(date);
/// assert_eq!(dt.format("%H:%M:%S").to_string(), "00:00:00");
/// ```
#[inline]
pub fn start_of_day(date: NaiveDate) -> DateTime<Utc> {
    date.and_time(midnight()).and_utc()
}

/// Convert a NaiveDate to DateTime<Utc> at the end of day (23:59:59 UTC).
///
/// # Example
/// ```
/// use chrono::NaiveDate;
/// use timemanager_backend::utils::datetime::end_of_day;
///
/// let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
/// let dt = end_of_day(date);
/// assert_eq!(dt.format("%H:%M:%S").to_string(), "23:59:59");
/// ```
#[inline]
pub fn end_of_day(date: NaiveDate) -> DateTime<Utc> {
    date.and_time(end_of_day_time()).and_utc()
}

/// Convert a NaiveDate to NaiveDateTime at the start of day (00:00:00).
/// Used for Diesel queries that require NaiveDateTime.
///
/// # Example
/// ```
/// use chrono::NaiveDate;
/// use timemanager_backend::utils::datetime::start_of_day_naive;
///
/// let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
/// let dt = start_of_day_naive(date);
/// assert_eq!(dt.format("%H:%M:%S").to_string(), "00:00:00");
/// ```
#[inline]
pub fn start_of_day_naive(date: NaiveDate) -> NaiveDateTime {
    date.and_time(midnight())
}

/// Convert a NaiveDate to NaiveDateTime at the end of day (23:59:59).
/// Used for Diesel queries that require NaiveDateTime.
///
/// # Example
/// ```
/// use chrono::NaiveDate;
/// use timemanager_backend::utils::datetime::end_of_day_naive;
///
/// let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
/// let dt = end_of_day_naive(date);
/// assert_eq!(dt.format("%H:%M:%S").to_string(), "23:59:59");
/// ```
#[inline]
pub fn end_of_day_naive(date: NaiveDate) -> NaiveDateTime {
    date.and_time(end_of_day_time())
}

/// Convert a NaiveDate to DateTime<Utc> at the start of day using chrono's timezone.
/// This variant uses `Utc.from_utc_datetime` for compatibility with certain query patterns.
#[inline]
pub fn start_of_day_tz(date: NaiveDate) -> DateTime<Utc> {
    Utc.from_utc_datetime(&start_of_day_naive(date))
}

/// Convert a NaiveDate to DateTime<Utc> at the end of day using chrono's timezone.
/// This variant uses `Utc.from_utc_datetime` for compatibility with certain query patterns.
#[inline]
pub fn end_of_day_tz(date: NaiveDate) -> DateTime<Utc> {
    Utc.from_utc_datetime(&end_of_day_naive(date))
}

/// Get the first day of a year (January 1st).
///
/// Returns an error if the year is out of the valid range for NaiveDate.
///
/// # Example
/// ```
/// use timemanager_backend::utils::datetime::start_of_year;
///
/// let date = start_of_year(2024).unwrap();
/// assert_eq!(date.format("%Y-%m-%d").to_string(), "2024-01-01");
/// ```
pub fn start_of_year(year: i32) -> Result<NaiveDate, AppError> {
    NaiveDate::from_ymd_opt(year, 1, 1)
        .ok_or_else(|| AppError::ValidationError(format!("Invalid year: {}", year)))
}

/// Get the last day of a year (December 31st).
///
/// Returns an error if the year is out of the valid range for NaiveDate.
///
/// # Example
/// ```
/// use timemanager_backend::utils::datetime::end_of_year;
///
/// let date = end_of_year(2024).unwrap();
/// assert_eq!(date.format("%Y-%m-%d").to_string(), "2024-12-31");
/// ```
pub fn end_of_year(year: i32) -> Result<NaiveDate, AppError> {
    NaiveDate::from_ymd_opt(year, 12, 31)
        .ok_or_else(|| AppError::ValidationError(format!("Invalid year: {}", year)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Timelike};

    #[test]
    fn test_midnight() {
        let time = midnight();
        assert_eq!(time.hour(), 0);
        assert_eq!(time.minute(), 0);
        assert_eq!(time.second(), 0);
    }

    #[test]
    fn test_end_of_day_time() {
        let time = end_of_day_time();
        assert_eq!(time.hour(), 23);
        assert_eq!(time.minute(), 59);
        assert_eq!(time.second(), 59);
    }

    #[test]
    fn test_start_of_day() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let dt = start_of_day(date);

        assert_eq!(dt.date_naive(), date);
        assert_eq!(dt.time(), midnight());
    }

    #[test]
    fn test_end_of_day() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let dt = end_of_day(date);

        assert_eq!(dt.date_naive(), date);
        assert_eq!(dt.time(), end_of_day_time());
    }

    #[test]
    fn test_start_of_day_naive() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let dt = start_of_day_naive(date);

        assert_eq!(dt.date(), date);
        assert_eq!(dt.time(), midnight());
    }

    #[test]
    fn test_end_of_day_naive() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let dt = end_of_day_naive(date);

        assert_eq!(dt.date(), date);
        assert_eq!(dt.time(), end_of_day_time());
    }

    #[test]
    fn test_start_of_year() {
        let date = start_of_year(2024).unwrap();
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 1);
    }

    #[test]
    fn test_end_of_year() {
        let date = end_of_year(2024).unwrap();
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), 12);
        assert_eq!(date.day(), 31);
    }

    #[test]
    fn test_start_of_year_invalid() {
        // Test with an invalid year (chrono supports years -262143 to 262142)
        let result = start_of_year(300000);
        assert!(result.is_err());
    }

    #[test]
    fn test_end_of_year_invalid() {
        let result = end_of_year(300000);
        assert!(result.is_err());
    }

    #[test]
    fn test_tz_variants_match_regular() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();

        assert_eq!(start_of_day(date), start_of_day_tz(date));
        assert_eq!(end_of_day(date), end_of_day_tz(date));
    }

    #[test]
    fn test_leap_year_handling() {
        // Feb 29 on a leap year
        let leap_date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        let start = start_of_day(leap_date);
        let end = end_of_day(leap_date);

        assert_eq!(start.date_naive(), leap_date);
        assert_eq!(end.date_naive(), leap_date);
    }
}
