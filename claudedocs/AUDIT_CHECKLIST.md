# Time Manager - Audit & Requirements Checklist

> **Document Purpose**: Track all user-facing improvements and features identified during the application audit.
> **Date**: 2026-01-11

---

## 1. Application - Global Features

### 1.1 First User / Production Flow
- [x] **Define production initialization flow**: Setup wizard shown when DB is empty
- [x] **Implement superadmin bootstrap mechanism**: `/setup` wizard with `/api/v1/auth/bootstrap` endpoint

### 1.2 Onboarding
- [x] **Create frontend onboarding experience**: WelcomeModal with role-specific tips shown on first login, persisted via localStorage

### 1.3 Development/Testing
- [x] **Add task/command to launch app without seed**: `task up:noseed` with NO_SEED env var

### 1.4 Internationalization & Localization
- [ ] **Implement language selector**: Allow users to choose their BO language
- [x] **Add timezone indicator**: Shows organization timezone in Dashboard header with Globe icon

### 1.5 UI/UX Global
- [x] **Move toast notifications**: Display at top-center instead of top-right
- [ ] **Fix responsive design (mobile-first)**: Currently unusable on mobile, items overflow containers

### 1.6 Clock Notes Feature
- [x] **Add clock note input in frontend**: Optional textarea in clock-out confirmation dialog
- [x] **Display notes in Pending Approvals**: Already implemented, verified working

### 1.7 User Role & Team Flexibility
- [x] **Allow user to be both manager AND employee**: Confirmed - Managers have all Employee capabilities (clocking) plus management features. Role hierarchy: SuperAdmin > Admin > Manager > Employee
- [x] **Evaluate multi-team membership**: Confirmed - Schema supports multi-team via team_members junction table. API: POST/DELETE /teams/:id/members. Use case: employee works on multiple projects/departments

### 1.8 Clock Restrictions Configuration
- [ ] **Implement clock-in/out restrictions at Organization level**: Define when users can clock
- [ ] **Allow Team-level override of clock restrictions**
- [ ] **Allow User-level override of clock restrictions**

### 1.9 Break System Redesign
- [ ] **Implement mandatory break windows**: Organization can define time ranges where clock doesn't count
- [ ] **Design break configuration at Organization/Team/User level**
- [ ] **Clearly document break system behavior**

### 1.10 CSV Export System
- [ ] **Design CSV export feature**: Determine what data can be exported
- [ ] **Define role-based export permissions**: What can each role export?
- [ ] **Implement CSV export at various points**: Users, Teams, Clock entries, Absences, etc.

---

## 2. Dashboard Page

### 2.1 Clock In/Out UX
- [x] **Prevent clock button spam**: 2s debounce after each clock action
- [x] **Add confirmation dialog on clock-out**: ConfirmDialog with "Are you sure?" message
- [x] **Define clock behavior per persona**: All roles can clock in/out. Employee: basic clocking. Manager: clocking + approve team clocks. Admin: clocking + manage users/teams. SuperAdmin: clocking + manage organizations. ClockWidget shown universally on Dashboard.

### 2.2 KPIs Clarity
- [x] **Add date precision to KPIs**: "Hours in January" format with dynamic month name
- [x] **Clarify time periods**: Month name shown in KPI titles and descriptions

### 2.3 Hours Worked Graph
- [x] **Fix week mode first week**: Weeks now align to ISO week boundaries (Monday). Backend aligns period.start to the Monday of the containing week.
- [x] **Fix week mode last week**: Hours clipped to period.end via `.min(period.end)` to avoid overflow values
- [x] **Put date ranges on separate lines**: CustomXAxisTick with multi-line labels (W2 + date range)

### 2.4 Hours Trend
- [x] **Verify Hours Trend is correct**: Fixed ISO date format in backend, chart rendering fixed

### 2.5 Quick Actions Section
- [x] **Evaluate "Your Profile" section**: Kept profile section for settings links. Moved Quick Actions to be visible to ALL users.
- [x] **Differentiate quick actions by role**: Employee: Clock History, Request Absence, Team Calendar. Manager adds: Pending Approvals. Admin adds: Manage Users/Teams.

---

## 3. Clock History Page

### 3.1 Filters Validation
- [x] **Add frontend date validation**: FROM must be before TO date (red border + error message)

### 3.2 Entries Counter
- [x] **Show total entries count**: Shows "X entries" instead of "X of Y"

---

## 4. My Absences Page

### 4.1 Request Counter
- [x] **Show total absence requests**: Shows "X requests" instead of "X of Y"

---

## 5. Clock Approval Page

### 5.1 User Context
- [x] **Display user's organization**: Added org_name in ClockEntryCard
- [x] **Display user's team**: Added team_name in ClockEntryCard

### 5.2 Time Comparison
- [x] **Show expected vs actual time worked**: Added theoretical_hours field to ClockEntryResponse. Frontend displays variance with color coding (green=met/exceeded, orange=under). Backend TODO: wire WorkScheduleRepository to ClockService to calculate actual theoretical hours.

---

## 6. Pending Absences Page

### 6.1 User Context
- [x] **Display user's organization**: Added org_name in AbsenceRequestCard
- [x] **Display user's team**: Added team_name in AbsenceRequestCard

---

## 7. Team Calendar Page

### 7.1 Multi-Team Manager Support
- [x] **Add team filter for managers**: Already implemented via OrgTeamFilter component. Managers can filter calendar by team using the team dropdown.

### 7.2 Empty State
- [x] **Show calendar even with no absences**: Calendar grid always visible with message in tbody

### 7.3 Additional Filters/Actions
- [x] **Evaluate need for more filters**: Evaluated - Current filters (Org/Team) are sufficient. Date range not needed (month navigation exists). User filter would add complexity. Absence type filter is nice-to-have but not essential.

---

## 8. Users Page

### 8.1 User Counter
- [x] **Show total users count**: Shows "X users" instead of "X of Y"

### 8.2 Show Deleted Feature
- [x] **Fix "Show deleted" toggle**: Backend now uses include_deleted query param

### 8.3 User Creation Form
- [ ] **Add Organization field**: Mandatory, not editable for admin role
- [ ] **Add Team field**: Optional
- [ ] **Add Schedule assignment**: Optional, defaults to organization/team schedule

### 8.4 Superadmin View
- [x] **Add organization filter for superadmin**: OrgTeamFilter component added to UsersPage
- [x] **Show organization column in user list**: showOrganization prop, visible only for SuperAdmin

### 8.5 Deletion Permissions
- [x] **Prevent self-deletion**: User cannot delete themselves (backend check already existed)
- [x] **Fix role hierarchy for deletion**: Admin cannot delete Superadmin (frontend + backend)

---

## 9. Teams Page

### 9.1 General Fixes
- [x] **Apply same fixes as Users page**: Counter shows "X teams" format

### 9.2 Superadmin Support
- [x] **Add organization filter for superadmin**: Already implemented via OrgTeamFilter component. SuperAdmin can filter teams by organization.
- [x] **Handle multi-organization context**: Handled via selectedOrgId passed to API

---

## 10. Profile Page

### 10.1 Role Display
- [x] **Style role consistently**: Uses same colored badge design as Users page

---

## 11. Grafana - Time Manager Dashboard

### 11.1 General Evaluation
- [ ] **Review dashboard usefulness**: What metrics are valuable?
- [ ] **Propose improvements**: What else should be monitored?

### 11.2 Latency Panel
- [ ] **Fix Latency (P50, P95, P99)**: Currently displays nothing

### 11.3 Infrastructure Panel
- [ ] **Fix Container CPU metrics**: Currently displays nothing
- [ ] **Fix Container Memory metrics**: Currently displays nothing

### 11.4 Backend Logs Panel
- [ ] **Fix Loki integration**: Backend Logs panel shows nothing
- [ ] **Ensure logs are properly collected**: Verify Promtail → Loki → Grafana pipeline

---

## Priority Legend

| Priority | Description |
|----------|-------------|
| 🔴 Critical | Blocks core functionality or security issue |
| 🟡 Important | Significant UX/functionality improvement |
| 🟢 Nice-to-have | Enhancement, polish |

---

## Notes

- **Responsive**: The mobile-first rework is extensive and should be planned as a dedicated sprint
- **i18n**: Language system needs architecture decision (library choice, translation workflow)
- **CSV Export**: Requires security review for data access permissions
- **Clock Restrictions**: Complex feature requiring DB schema changes and comprehensive testing
