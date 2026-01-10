//! In-memory caching service for read-heavy data
//!
//! Provides time-based caching for:
//! - Absence types (per organization)
//! - Work schedules (per organization)
//! - Closed days (per organization)

use cached::{Cached, TimedCache};
use std::sync::Mutex;
use uuid::Uuid;

use crate::models::{AbsenceTypeResponse, ClosedDayResponse, WorkScheduleWithDays};

/// Cache TTL in seconds (5 minutes)
const CACHE_TTL_SECONDS: u64 = 300;

lazy_static::lazy_static! {
    static ref ABSENCE_TYPES_CACHE: Mutex<TimedCache<Uuid, Vec<AbsenceTypeResponse>>> =
        Mutex::new(TimedCache::with_lifespan(CACHE_TTL_SECONDS));

    static ref SCHEDULES_CACHE: Mutex<TimedCache<Uuid, Vec<WorkScheduleWithDays>>> =
        Mutex::new(TimedCache::with_lifespan(CACHE_TTL_SECONDS));

    static ref CLOSED_DAYS_CACHE: Mutex<TimedCache<String, Vec<ClosedDayResponse>>> =
        Mutex::new(TimedCache::with_lifespan(CACHE_TTL_SECONDS));
}

/// Cache service for read-heavy data
pub struct CacheService;

impl CacheService {
    // === Absence Types Cache ===

    /// Get cached absence types for an organization
    pub fn get_absence_types(org_id: Uuid) -> Option<Vec<AbsenceTypeResponse>> {
        let mut cache = ABSENCE_TYPES_CACHE.lock().ok()?;
        cache.cache_get(&org_id).cloned()
    }

    /// Set cached absence types for an organization
    pub fn set_absence_types(org_id: Uuid, types: Vec<AbsenceTypeResponse>) {
        if let Ok(mut cache) = ABSENCE_TYPES_CACHE.lock() {
            cache.cache_set(org_id, types);
        }
    }

    /// Invalidate absence types cache for an organization
    pub fn invalidate_absence_types(org_id: Uuid) {
        if let Ok(mut cache) = ABSENCE_TYPES_CACHE.lock() {
            cache.cache_remove(&org_id);
        }
    }

    // === Work Schedules Cache ===

    /// Get cached work schedules for an organization
    pub fn get_schedules(org_id: Uuid) -> Option<Vec<WorkScheduleWithDays>> {
        let mut cache = SCHEDULES_CACHE.lock().ok()?;
        cache.cache_get(&org_id).cloned()
    }

    /// Set cached work schedules for an organization
    pub fn set_schedules(org_id: Uuid, schedules: Vec<WorkScheduleWithDays>) {
        if let Ok(mut cache) = SCHEDULES_CACHE.lock() {
            cache.cache_set(org_id, schedules);
        }
    }

    /// Invalidate schedules cache for an organization
    pub fn invalidate_schedules(org_id: Uuid) {
        if let Ok(mut cache) = SCHEDULES_CACHE.lock() {
            cache.cache_remove(&org_id);
        }
    }

    // === Closed Days Cache ===

    /// Build cache key for closed days (org_id + filter params)
    fn closed_days_key(org_id: Uuid, start: Option<&str>, end: Option<&str>, recurring: Option<bool>) -> String {
        format!(
            "{}:{}:{}:{}",
            org_id,
            start.unwrap_or(""),
            end.unwrap_or(""),
            recurring.map(|r| r.to_string()).unwrap_or_default()
        )
    }

    /// Get cached closed days for an organization with filters
    pub fn get_closed_days(
        org_id: Uuid,
        start_date: Option<&str>,
        end_date: Option<&str>,
        is_recurring: Option<bool>,
    ) -> Option<Vec<ClosedDayResponse>> {
        let key = Self::closed_days_key(org_id, start_date, end_date, is_recurring);
        let mut cache = CLOSED_DAYS_CACHE.lock().ok()?;
        cache.cache_get(&key).cloned()
    }

    /// Set cached closed days for an organization with filters
    pub fn set_closed_days(
        org_id: Uuid,
        start_date: Option<&str>,
        end_date: Option<&str>,
        is_recurring: Option<bool>,
        days: Vec<ClosedDayResponse>,
    ) {
        let key = Self::closed_days_key(org_id, start_date, end_date, is_recurring);
        if let Ok(mut cache) = CLOSED_DAYS_CACHE.lock() {
            cache.cache_set(key, days);
        }
    }

    /// Invalidate all closed days cache entries for an organization
    /// Note: This clears the entire cache since we use composite keys
    pub fn invalidate_closed_days() {
        if let Ok(mut cache) = CLOSED_DAYS_CACHE.lock() {
            cache.cache_clear();
        }
    }

    // === Global Operations ===

    /// Clear all caches (useful for testing or admin operations)
    pub fn clear_all() {
        if let Ok(mut cache) = ABSENCE_TYPES_CACHE.lock() {
            cache.cache_clear();
        }
        if let Ok(mut cache) = SCHEDULES_CACHE.lock() {
            cache.cache_clear();
        }
        if let Ok(mut cache) = CLOSED_DAYS_CACHE.lock() {
            cache.cache_clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closed_days_key_generation() {
        let org_id = Uuid::new_v4();

        let key1 = CacheService::closed_days_key(org_id, Some("2024-01-01"), Some("2024-12-31"), Some(true));
        let key2 = CacheService::closed_days_key(org_id, None, None, None);

        assert!(key1.contains(&org_id.to_string()));
        assert!(key1.contains("2024-01-01"));
        assert!(key1.contains("true"));

        assert!(key2.contains(&org_id.to_string()));
    }

    #[test]
    fn test_cache_invalidation() {
        // Test that clear_all doesn't panic
        CacheService::clear_all();
    }
}
