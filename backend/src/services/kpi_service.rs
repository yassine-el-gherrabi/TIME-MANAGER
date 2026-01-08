use chrono::{DateTime, Datelike, Utc};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use serde::Serialize;
use uuid::Uuid;

use crate::domain::enums::ClockEntryStatus;
use crate::error::AppError;
use crate::repositories::{ClockRepository, TeamRepository, UserRepository, WorkScheduleRepository};

type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Individual user KPIs
#[derive(Debug, Serialize)]
pub struct UserKPIs {
    pub user_id: Uuid,
    pub user_name: String,
    pub total_hours_worked: f64,
    pub theoretical_hours: f64,
    pub hours_variance: f64,
    pub punctuality_rate: f64,
    pub days_worked: i32,
    pub days_late: i32,
    pub average_daily_hours: f64,
}

/// Team KPIs summary
#[derive(Debug, Serialize)]
pub struct TeamKPIs {
    pub team_id: Uuid,
    pub team_name: String,
    pub member_count: i64,
    pub total_hours: f64,
    pub average_punctuality: f64,
    pub currently_clocked_in: i32,
    pub members: Vec<MemberKPISummary>,
}

/// Summary KPIs for a team member
#[derive(Debug, Serialize)]
pub struct MemberKPISummary {
    pub user_id: Uuid,
    pub user_name: String,
    pub hours_worked: f64,
    pub punctuality_rate: f64,
    pub is_clocked_in: bool,
}

/// Organization-wide KPIs
#[derive(Debug, Serialize)]
pub struct OrgKPIs {
    pub total_employees: i64,
    pub total_hours: f64,
    pub average_punctuality: f64,
    pub currently_clocked_in: i32,
    pub attendance_rate: f64,
}

/// Real-time presence overview
#[derive(Debug, Serialize)]
pub struct PresenceOverview {
    pub total_employees: i64,
    pub currently_present: i32,
    pub present_users: Vec<PresentUser>,
}

/// User currently present
#[derive(Debug, Serialize)]
pub struct PresentUser {
    pub user_id: Uuid,
    pub user_name: String,
    pub clock_in_time: DateTime<Utc>,
    pub elapsed_minutes: i64,
}

/// Chart data point
#[derive(Debug, Serialize)]
pub struct ChartDataPoint {
    pub date: String,
    pub hours_worked: f64,
    pub theoretical_hours: f64,
}

/// Chart data response
#[derive(Debug, Serialize)]
pub struct ChartData {
    pub data: Vec<ChartDataPoint>,
    pub granularity: String,
}

/// Date range for KPI queries
#[derive(Debug, Clone)]
pub struct DateRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

/// Granularity for chart data
#[derive(Debug, Clone, Copy)]
pub enum Granularity {
    Day,
    Week,
    Month,
}

/// Service for KPI calculations
pub struct KPIService {
    clock_repo: ClockRepository,
    team_repo: TeamRepository,
    user_repo: UserRepository,
    schedule_repo: WorkScheduleRepository,
}

impl KPIService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            clock_repo: ClockRepository::new(pool.clone()),
            team_repo: TeamRepository::new(pool.clone()),
            user_repo: UserRepository::new(pool.clone()),
            schedule_repo: WorkScheduleRepository::new(pool),
        }
    }

    /// Get KPIs for a specific user
    pub async fn get_user_kpis(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        period: DateRange,
    ) -> Result<UserKPIs, AppError> {
        let user = self.user_repo.find_by_id(user_id).await?;
        let user_name = format!("{} {}", user.first_name, user.last_name);

        // Get clock entries for the period
        let entries = self
            .clock_repo
            .get_entries_for_period(org_id, user_id, period.start, period.end)
            .await?;

        // Calculate total hours worked
        let total_minutes: i64 = entries
            .iter()
            .filter(|e| e.status == ClockEntryStatus::Approved)
            .filter_map(|e| e.clock_out.map(|out| (out - e.clock_in).num_minutes()))
            .sum();
        let total_hours_worked = total_minutes as f64 / 60.0;

        // Calculate theoretical hours
        let theoretical_hours = self
            .schedule_repo
            .get_theoretical_hours(org_id, user_id, period.start, period.end)
            .await?;

        // Calculate punctuality
        let (days_worked, days_late) =
            self.calculate_punctuality(org_id, user_id, &entries).await?;

        let punctuality_rate = if days_worked > 0 {
            ((days_worked - days_late) as f64 / days_worked as f64) * 100.0
        } else {
            100.0
        };

        let average_daily_hours = if days_worked > 0 {
            total_hours_worked / days_worked as f64
        } else {
            0.0
        };

        Ok(UserKPIs {
            user_id,
            user_name,
            total_hours_worked,
            theoretical_hours,
            hours_variance: total_hours_worked - theoretical_hours,
            punctuality_rate,
            days_worked,
            days_late,
            average_daily_hours,
        })
    }

    /// Get KPIs for a team
    pub async fn get_team_kpis(
        &self,
        org_id: Uuid,
        team_id: Uuid,
        period: DateRange,
    ) -> Result<TeamKPIs, AppError> {
        let team = self.team_repo.find_by_id(org_id, team_id).await?;
        let members = self.team_repo.list_members(team_id).await?;

        let mut member_summaries = Vec::with_capacity(members.len());
        let mut total_hours = 0.0;
        let mut total_punctuality = 0.0;
        let mut currently_clocked_in = 0;

        for member in &members {
            let entries = self
                .clock_repo
                .get_entries_for_period(org_id, member.id, period.start, period.end)
                .await?;

            let hours_worked: f64 = entries
                .iter()
                .filter(|e| e.status == ClockEntryStatus::Approved)
                .filter_map(|e| e.clock_out.map(|out| (out - e.clock_in).num_minutes()))
                .sum::<i64>() as f64
                / 60.0;

            let (days_worked, days_late) =
                self.calculate_punctuality(org_id, member.id, &entries).await?;

            let punctuality_rate = if days_worked > 0 {
                ((days_worked - days_late) as f64 / days_worked as f64) * 100.0
            } else {
                100.0
            };

            let is_clocked_in = self
                .clock_repo
                .find_open_entry(org_id, member.id)
                .await?
                .is_some();

            if is_clocked_in {
                currently_clocked_in += 1;
            }

            total_hours += hours_worked;
            total_punctuality += punctuality_rate;

            member_summaries.push(MemberKPISummary {
                user_id: member.id,
                user_name: format!("{} {}", member.first_name, member.last_name),
                hours_worked,
                punctuality_rate,
                is_clocked_in,
            });
        }

        let member_count = members.len() as i64;
        let average_punctuality = if member_count > 0 {
            total_punctuality / member_count as f64
        } else {
            100.0
        };

        Ok(TeamKPIs {
            team_id,
            team_name: team.name,
            member_count,
            total_hours,
            average_punctuality,
            currently_clocked_in,
            members: member_summaries,
        })
    }

    /// Get organization-wide KPIs
    pub async fn get_org_kpis(
        &self,
        org_id: Uuid,
        period: DateRange,
    ) -> Result<OrgKPIs, AppError> {
        // This is a simplified version - in production you'd want proper pagination
        let filter = crate::models::UserFilter::default();
        let pagination = crate::models::Pagination {
            page: 1,
            per_page: 1000,
        };

        let (users, total_employees) = self.user_repo.list(org_id, &filter, &pagination).await?;

        let mut total_hours = 0.0;
        let mut total_punctuality = 0.0;
        let mut users_with_entries = 0;

        for user in &users {
            let entries = self
                .clock_repo
                .get_entries_for_period(org_id, user.id, period.start, period.end)
                .await?;

            if !entries.is_empty() {
                users_with_entries += 1;

                let hours: f64 = entries
                    .iter()
                    .filter(|e| e.status == ClockEntryStatus::Approved)
                    .filter_map(|e| e.clock_out.map(|out| (out - e.clock_in).num_minutes()))
                    .sum::<i64>() as f64
                    / 60.0;

                let (days_worked, days_late) =
                    self.calculate_punctuality(org_id, user.id, &entries).await?;

                let punctuality = if days_worked > 0 {
                    ((days_worked - days_late) as f64 / days_worked as f64) * 100.0
                } else {
                    100.0
                };

                total_hours += hours;
                total_punctuality += punctuality;
            }
        }

        let clocked_in = self.clock_repo.get_currently_clocked_in(org_id).await?.len() as i32;

        let average_punctuality = if users_with_entries > 0 {
            total_punctuality / users_with_entries as f64
        } else {
            100.0
        };

        let attendance_rate = if total_employees > 0 {
            (users_with_entries as f64 / total_employees as f64) * 100.0
        } else {
            0.0
        };

        Ok(OrgKPIs {
            total_employees,
            total_hours,
            average_punctuality,
            currently_clocked_in: clocked_in,
            attendance_rate,
        })
    }

    /// Get real-time presence overview
    pub async fn get_real_time_presence(&self, org_id: Uuid) -> Result<PresenceOverview, AppError> {
        let filter = crate::models::UserFilter::default();
        let pagination = crate::models::Pagination {
            page: 1,
            per_page: 1000,
        };

        let (_, total_employees) = self.user_repo.list(org_id, &filter, &pagination).await?;

        let clocked_in_entries = self.clock_repo.get_currently_clocked_in(org_id).await?;

        let mut present_users = Vec::with_capacity(clocked_in_entries.len());
        for entry in &clocked_in_entries {
            let (user_name, _) = self.clock_repo.get_user_info(entry.user_id).await?;
            let elapsed_minutes = (Utc::now() - entry.clock_in).num_minutes();

            present_users.push(PresentUser {
                user_id: entry.user_id,
                user_name,
                clock_in_time: entry.clock_in,
                elapsed_minutes,
            });
        }

        Ok(PresenceOverview {
            total_employees,
            currently_present: present_users.len() as i32,
            present_users,
        })
    }

    /// Get chart data for hours worked
    pub async fn get_chart_data(
        &self,
        org_id: Uuid,
        user_id: Option<Uuid>,
        period: DateRange,
        granularity: Granularity,
    ) -> Result<ChartData, AppError> {
        // Simplified - generates data points based on granularity
        let mut data = Vec::new();
        let mut current = period.start;

        while current < period.end {
            let (point_end, date_str) = match granularity {
                Granularity::Day => {
                    let next = current + chrono::Duration::days(1);
                    (next, current.format("%Y-%m-%d").to_string())
                }
                Granularity::Week => {
                    let next = current + chrono::Duration::weeks(1);
                    (next, format!("Week {}", current.iso_week().week()))
                }
                Granularity::Month => {
                    let next = current + chrono::Duration::days(30);
                    (next, current.format("%Y-%m").to_string())
                }
            };

            let hours_worked = if let Some(uid) = user_id {
                let entries = self
                    .clock_repo
                    .get_entries_for_period(org_id, uid, current, point_end.min(period.end))
                    .await?;

                entries
                    .iter()
                    .filter(|e| e.status == ClockEntryStatus::Approved)
                    .filter_map(|e| e.clock_out.map(|out| (out - e.clock_in).num_minutes()))
                    .sum::<i64>() as f64
                    / 60.0
            } else {
                // Aggregate for entire org - simplified
                0.0
            };

            let theoretical_hours = if let Some(uid) = user_id {
                self.schedule_repo
                    .get_theoretical_hours(org_id, uid, current, point_end.min(period.end))
                    .await?
            } else {
                0.0
            };

            data.push(ChartDataPoint {
                date: date_str,
                hours_worked,
                theoretical_hours,
            });

            current = point_end;
        }

        Ok(ChartData {
            data,
            granularity: match granularity {
                Granularity::Day => "day".to_string(),
                Granularity::Week => "week".to_string(),
                Granularity::Month => "month".to_string(),
            },
        })
    }

    /// Calculate punctuality (days on time vs late)
    async fn calculate_punctuality(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        entries: &[crate::models::ClockEntry],
    ) -> Result<(i32, i32), AppError> {
        // Get user's schedule
        let schedule = self.schedule_repo.get_user_schedule(org_id, user_id).await?;
        let schedule = match schedule {
            Some(s) => s,
            None => {
                // Try default schedule
                match self.schedule_repo.get_default(org_id).await? {
                    Some(s) => s,
                    None => return Ok((entries.len() as i32, 0)), // No schedule, consider all on time
                }
            }
        };

        let days = self.schedule_repo.get_days(schedule.id).await?;
        if days.is_empty() {
            return Ok((entries.len() as i32, 0));
        }

        let mut days_worked = 0;
        let mut days_late = 0;
        let grace_period_minutes = 5; // 5 minute grace period

        for entry in entries {
            if entry.status != ClockEntryStatus::Approved {
                continue;
            }

            days_worked += 1;

            let weekday = entry.clock_in.weekday().num_days_from_monday() as i16;
            if let Some(day_schedule) = days.iter().find(|d| d.day_of_week == weekday) {
                let clock_in_time = entry.clock_in.time();
                let expected_start = day_schedule.start_time
                    + chrono::Duration::minutes(grace_period_minutes);

                if clock_in_time > expected_start {
                    days_late += 1;
                }
            }
        }

        Ok((days_worked, days_late))
    }
}
