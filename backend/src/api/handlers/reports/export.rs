use axum::{
    extract::{Query, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use chrono::NaiveDate;
use serde::Deserialize;
use std::collections::HashMap;
use uuid::Uuid;

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::{Admin, RoleGuard};
use crate::repositories::{
    AbsenceRepository, AbsenceTypeRepository, ClockRepository, UserRepository,
};
use crate::utils::{end_of_day, start_of_day};

#[derive(Debug, Deserialize)]
pub struct ExportReportsQuery {
    /// Type of export: "clocks", "absences", or "users"
    #[serde(rename = "type")]
    pub export_type: ExportType,
    /// Filter from date (YYYY-MM-DD)
    pub start_date: Option<NaiveDate>,
    /// Filter to date (YYYY-MM-DD)
    pub end_date: Option<NaiveDate>,
    /// Filter by specific user
    pub user_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ExportType {
    Clocks,
    Absences,
    Users,
}

/// GET /api/v1/reports/export
///
/// Export data as CSV.
/// Admin+ access only.
/// Returns CSV file for download.
pub async fn export_reports(
    State(state): State<AppState>,
    RoleGuard(user, _): RoleGuard<Admin>,
    Query(query): Query<ExportReportsQuery>,
) -> Result<Response, AppError> {
    let claims = user.0;
    let org_id = claims.org_id;

    let (csv_content, filename_prefix) = match query.export_type {
        ExportType::Clocks => {
            let csv = export_clocks(&state, org_id, &query).await?;
            (csv, "clocks")
        }
        ExportType::Absences => {
            let csv = export_absences(&state, org_id, &query).await?;
            (csv, "absences")
        }
        ExportType::Users => {
            let csv = export_users(&state, org_id).await?;
            (csv, "users")
        }
    };

    // Log export
    tracing::info!(
        user_id = %claims.sub,
        org_id = %org_id,
        export_type = filename_prefix,
        "Admin exported {} to CSV",
        filename_prefix
    );

    // Generate filename with current timestamp
    let filename = format!(
        "{}_{}.csv",
        filename_prefix,
        chrono::Utc::now().format("%Y%m%d_%H%M%S")
    );

    // Build response with proper headers
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/csv; charset=utf-8"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{}\"", filename))
            .unwrap_or_else(|_| HeaderValue::from_static("attachment; filename=\"export.csv\"")),
    );

    Ok((StatusCode::OK, headers, csv_content).into_response())
}

/// Export clock entries to CSV
async fn export_clocks(
    state: &AppState,
    org_id: Uuid,
    query: &ExportReportsQuery,
) -> Result<String, AppError> {
    use crate::models::{ClockFilter, Pagination};

    let clock_repo = ClockRepository::new(state.db_pool.clone());
    let user_repo = UserRepository::new(state.db_pool.clone());

    let filter = ClockFilter {
        user_id: query.user_id,
        start_date: query.start_date.map(start_of_day),
        end_date: query.end_date.map(end_of_day),
        status: None,
    };

    // Get all clock entries (max 10000)
    let pagination = Pagination {
        page: 1,
        per_page: 10000,
    };

    let (entries, _) = clock_repo.list_all(org_id, &filter, &pagination).await?;

    // Build user cache
    let mut user_cache: HashMap<Uuid, (String, String)> = HashMap::new();

    // Build CSV
    let mut csv = String::from(
        "Date,User Email,User Name,Clock In,Clock Out,Duration (hours),Status,Notes\n",
    );

    for entry in entries {
        // Get user info from cache or fetch
        let (email, name) = if let Some(info) = user_cache.get(&entry.user_id) {
            info.clone()
        } else {
            let info = match user_repo.find_by_id(entry.user_id).await {
                Ok(u) => (u.email.clone(), format!("{} {}", u.first_name, u.last_name)),
                Err(_) => ("Unknown".to_string(), "Unknown".to_string()),
            };
            user_cache.insert(entry.user_id, info.clone());
            info
        };

        let date = entry.clock_in.format("%Y-%m-%d").to_string();
        let clock_in = entry.clock_in.format("%H:%M:%S").to_string();
        let clock_out = entry
            .clock_out
            .map(|t| t.format("%H:%M:%S").to_string())
            .unwrap_or_default();

        // Calculate duration in hours
        let duration = entry
            .clock_out
            .map(|out| {
                let duration = out - entry.clock_in;
                let hours = duration.num_minutes() as f64 / 60.0;
                format!("{:.2}", hours)
            })
            .unwrap_or_default();

        let status = format!("{:?}", entry.status);
        let notes = entry
            .notes
            .unwrap_or_default()
            .replace(',', ";")
            .replace('\n', " ");

        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{}\n",
            date,
            escape_csv(&email),
            escape_csv(&name),
            clock_in,
            clock_out,
            duration,
            status,
            escape_csv(&notes)
        ));
    }

    Ok(csv)
}

/// Export absences to CSV
async fn export_absences(
    state: &AppState,
    org_id: Uuid,
    query: &ExportReportsQuery,
) -> Result<String, AppError> {
    use crate::models::{AbsenceFilter, Pagination};

    let absence_repo = AbsenceRepository::new(state.db_pool.clone());
    let absence_type_repo = AbsenceTypeRepository::new(state.db_pool.clone());
    let user_repo = UserRepository::new(state.db_pool.clone());

    let filter = AbsenceFilter {
        user_id: query.user_id,
        start_date: query.start_date,
        end_date: query.end_date,
        status: None,
        type_id: None,
        team_id: None,
    };

    // Get all absences (max 10000)
    let pagination = Pagination {
        page: 1,
        per_page: 10000,
    };

    let (absences, _) = absence_repo.list(org_id, &filter, &pagination).await?;

    // Build user cache
    let mut user_cache: HashMap<Uuid, (String, String)> = HashMap::new();

    // Build type cache - preload all absence types
    let mut type_cache: HashMap<Uuid, String> = HashMap::new();
    let types = absence_type_repo.list(org_id).await?;
    for t in types {
        type_cache.insert(t.id, t.name.clone());
    }

    // Build CSV
    let mut csv =
        String::from("Start Date,End Date,User Email,User Name,Type,Days,Status,Reason\n");

    for absence in absences {
        // Get user info from cache or fetch
        let (email, name) = if let Some(info) = user_cache.get(&absence.user_id) {
            info.clone()
        } else {
            let info = match user_repo.find_by_id(absence.user_id).await {
                Ok(u) => (u.email.clone(), format!("{} {}", u.first_name, u.last_name)),
                Err(_) => ("Unknown".to_string(), "Unknown".to_string()),
            };
            user_cache.insert(absence.user_id, info.clone());
            info
        };

        // Get type name from cache
        let type_name = type_cache
            .get(&absence.type_id)
            .cloned()
            .unwrap_or_else(|| "Unknown".to_string());

        let start_date = absence.start_date.format("%Y-%m-%d").to_string();
        let end_date = absence.end_date.format("%Y-%m-%d").to_string();
        let days = absence.days_count.to_string();
        let status = format!("{:?}", absence.status);
        let reason = absence
            .reason
            .unwrap_or_default()
            .replace(',', ";")
            .replace('\n', " ");

        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{}\n",
            start_date,
            end_date,
            escape_csv(&email),
            escape_csv(&name),
            escape_csv(&type_name),
            days,
            status,
            escape_csv(&reason)
        ));
    }

    Ok(csv)
}

/// Export users to CSV
async fn export_users(state: &AppState, org_id: Uuid) -> Result<String, AppError> {
    use crate::models::{Pagination, UserFilter};

    let user_repo = UserRepository::new(state.db_pool.clone());

    let filter = UserFilter::default();
    let pagination = Pagination {
        page: 1,
        per_page: 10000,
    };

    let (users, _) = user_repo.list(org_id, &filter, &pagination).await?;

    // Build CSV
    let mut csv = String::from("Email,First Name,Last Name,Role,Phone,Created At,Status\n");

    for user in users {
        let status = if user.deleted_at.is_some() {
            "Deleted"
        } else {
            "Active"
        };

        let role_str = format!("{:?}", user.role);
        csv.push_str(&format!(
            "{},{},{},{},{},{},{}\n",
            escape_csv(&user.email),
            escape_csv(&user.first_name),
            escape_csv(&user.last_name),
            role_str,
            escape_csv(&user.phone.unwrap_or_default()),
            user.created_at.format("%Y-%m-%d %H:%M:%S"),
            status
        ));
    }

    Ok(csv)
}

/// Escape CSV field - wrap in quotes if contains comma, quote, or newline
fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}
