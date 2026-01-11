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
- [ ] **Add timezone indicator**: Show which timezone the organization operates in

### 1.5 UI/UX Global
- [ ] **Move toast notifications**: Display at top-center instead of top-right
- [ ] **Fix responsive design (mobile-first)**: Currently unusable on mobile, items overflow containers

### 1.6 Clock Notes Feature
- [x] **Add clock note input in frontend**: Optional textarea in clock-out confirmation dialog
- [x] **Display notes in Pending Approvals**: Already implemented, verified working

### 1.7 User Role & Team Flexibility
- [ ] **Allow user to be both manager AND employee**: Confirm this is supported
- [ ] **Evaluate multi-team membership**: Can a user belong to multiple teams? Define persona/use case

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
- [ ] **Define clock behavior per persona**: What makes sense for each user type?

### 2.2 KPIs Clarity
- [ ] **Add date precision to KPIs**: "Hours This Month" → specify which month
- [ ] **Clarify time periods**: Weekly? Monthly? Add clear labels

### 2.3 Hours Worked Graph
- [ ] **Fix week mode first week**: Start from 1st of month, not previous month's last week
- [ ] **Fix week mode last week**: Avoid false values from month overflow
- [ ] **Put date ranges on separate lines**: Improve readability

### 2.4 Hours Trend
- [ ] **Verify Hours Trend is correct**: Confirm functionality

### 2.5 Quick Actions Section
- [ ] **Evaluate "Your Profile" section**: Replace with quick actions relevant to user role?
- [ ] **Differentiate quick actions by role**: Employee vs Admin vs Superadmin have different frequent tasks

---

## 3. Clock History Page

### 3.1 Filters Validation
- [ ] **Add frontend date validation**: FROM must be before TO date

### 3.2 Entries Counter
- [ ] **Show total entries count**: Not "X of Y displayed" but total entries (infinite scroll context)

---

## 4. My Absences Page

### 4.1 Request Counter
- [ ] **Show total absence requests**: Not "2 of 2" but total count

---

## 5. Clock Approval Page

### 5.1 User Context
- [ ] **Display user's organization**
- [ ] **Display user's team**

### 5.2 Time Comparison
- [ ] **Show expected vs actual time worked**: If user has a schedule, show +/- difference

---

## 6. Pending Absences Page

### 6.1 User Context
- [ ] **Display user's organization**
- [ ] **Display user's team**

---

## 7. Team Calendar Page

### 7.1 Multi-Team Manager Support
- [ ] **Add team filter for managers**: Managers of multiple teams can filter calendar by team

### 7.2 Empty State
- [ ] **Show calendar even with no absences**: Don't hide calendar when no absences scheduled

### 7.3 Additional Filters/Actions
- [ ] **Evaluate need for more filters**: Date range? User? Absence type?

---

## 8. Users Page

### 8.1 User Counter
- [ ] **Show total users count**: Not "10 of 10" with infinite scroll

### 8.2 Show Deleted Feature
- [ ] **Fix "Show deleted" toggle**: Currently shows same list (not working)

### 8.3 User Creation Form
- [ ] **Add Organization field**: Mandatory, not editable for admin role
- [ ] **Add Team field**: Optional
- [ ] **Add Schedule assignment**: Optional, defaults to organization/team schedule

### 8.4 Superadmin View
- [ ] **Add organization filter for superadmin**
- [ ] **Show organization column in user list**

### 8.5 Deletion Permissions
- [ ] **Prevent self-deletion**: User cannot delete themselves
- [ ] **Fix role hierarchy for deletion**: Admin cannot delete Superadmin (frontend + backend)

---

## 9. Teams Page

### 9.1 General Fixes
- [ ] **Apply same fixes as Users page**: Counter, filters, etc.

### 9.2 Superadmin Support
- [ ] **Add organization filter for superadmin**
- [ ] **Handle multi-organization context**

---

## 10. Profile Page

### 10.1 Role Display
- [ ] **Style role consistently**: Use same badge/color design as Users page

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
