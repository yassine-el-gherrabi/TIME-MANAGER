# Database Design Documentation

> **Status**: Technology-agnostic design - Database system to be selected
> **Last Updated**: 2025-10-06
> **Version**: 1.0

---

## Table of Contents

1. [Database Requirements](#database-requirements)
2. [Entity-Relationship Model](#entity-relationship-model)
3. [Schema Design](#schema-design)
4. [Data Integrity Rules](#data-integrity-rules)
5. [Indexing Strategy](#indexing-strategy)
6. [Query Patterns](#query-patterns)
7. [Data Migration Strategy](#data-migration-strategy)
8. [Performance Considerations](#performance-considerations)
9. [Backup & Recovery](#backup--recovery)

---

## Database Requirements

### Functional Requirements

**Data Storage Needs:**
- User account information (employees and managers)
- Team organization structure
- Clock in/out timestamps (time-series data)
- Working hours calculations
- Audit trail for compliance

**Data Volume Estimates:**
- **Users**: 100-500 initially, growth to 5,000+
- **Teams**: 10-50 initially
- **Clock Entries**: ~200 per user per month (2 clocks/day × 22 working days)
- **Total Clock Entries**: 10,000-100,000 per month initially

**Data Retention:**
- Active data: All current and historical clocks
- User data: Retain after account deletion (soft delete for compliance)
- Minimum retention: 3 years (labor law compliance)

### Non-Functional Requirements

**Performance:**
- Read response time: < 100ms (p95)
- Write response time: < 50ms (p95)
- Concurrent users: 100+ simultaneous
- Report generation: < 5 seconds for 1 year of data

**Availability:**
- Target: 99.9% uptime
- Acceptable downtime: < 8 hours/year
- Backup frequency: Daily
- Recovery Time Objective (RTO): < 4 hours
- Recovery Point Objective (RPO): < 24 hours

**Data Integrity:**
- ACID compliance for financial accuracy
- Referential integrity enforcement
- Transaction support for multi-step operations
- Audit logging for sensitive operations

**Security:**
- Encrypted connections
- Password hashing (never plain text storage)
- Role-based access control
- Soft delete for GDPR compliance

---

## Entity-Relationship Model

### Conceptual Model

```
┌─────────────────────────────────────────────────────────────┐
│                        TIME MANAGER                          │
│                     Conceptual Data Model                    │
└─────────────────────────────────────────────────────────────┘

                        manages
        ┌────────────────────────────────────┐
        │                                    │
        │                                    ▼
    ┌───┴────┐                          ┌────────┐
    │  User  │◄─────── member_of ───────│  Team  │
    │        │                          │        │
    └───┬────┘                          └────────┘
        │
        │ owns
        │
        ▼
    ┌────────┐
    │ Clock  │
    │        │
    └────────┘

Relationships:
  - User manages Team (1:N)
  - User member_of Team (M:N)
  - User owns Clock (1:N)
```

### Entity Definitions

**User Entity:**
```
Entity: USER
Description: Represents employees and managers in the system
Attributes:
  - id: Unique identifier
  - email: Unique login identifier
  - password_hash: Encrypted password
  - first_name: User's first name
  - last_name: User's last name
  - phone_number: Contact number (optional)
  - role: Employee or Manager designation
  - created_at: Account creation timestamp
  - updated_at: Last modification timestamp
  - deleted_at: Soft delete timestamp (nullable)

Business Rules:
  - Email must be unique across all users
  - Password must be hashed (never plain text)
  - Role must be 'employee' or 'manager'
  - Deleted accounts retained for audit (soft delete)
```

**Team Entity:**
```
Entity: TEAM
Description: Organizational unit grouping users
Attributes:
  - id: Unique identifier
  - name: Team name (unique)
  - description: Team purpose/description
  - manager_id: User who manages this team (FK)
  - created_at: Team creation timestamp
  - updated_at: Last modification timestamp
  - deleted_at: Soft delete timestamp (nullable)

Business Rules:
  - Team name must be unique
  - Manager must be a user with 'manager' role
  - Team can have only one manager
  - Deleting manager doesn't delete team (set manager_id NULL)
```

**Clock Entity:**
```
Entity: CLOCK
Description: Individual clock in/out event
Attributes:
  - id: Unique identifier
  - user_id: User who clocked in/out (FK)
  - time: Timestamp of event (UTC)
  - status: 'arrival' or 'departure'
  - created_at: Record creation timestamp

Business Rules:
  - Time must be in UTC timezone
  - Status must be 'arrival' or 'departure'
  - User cannot have two consecutive 'arrival' events
  - User cannot have 'departure' without prior 'arrival'
  - Cannot delete clock entries (audit trail)
```

**Team_Member Entity:**
```
Entity: TEAM_MEMBER
Description: Many-to-many relationship between users and teams
Attributes:
  - id: Unique identifier
  - user_id: User in the team (FK)
  - team_id: Team containing the user (FK)
  - joined_at: Timestamp when user joined team

Business Rules:
  - User can be member of multiple teams
  - Same user cannot be added to same team twice
  - Deleting user removes all team memberships (cascade)
  - Deleting team removes all memberships (cascade)
```

---

## Schema Design

### Logical Schema (Technology-Agnostic)

**Users Table:**

```sql
TABLE: users
--------------------------------------------------
COLUMN NAME       | DATA TYPE         | CONSTRAINTS
--------------------------------------------------
id                | INTEGER/UUID      | PRIMARY KEY, AUTO_INCREMENT
email             | VARCHAR(255)      | UNIQUE, NOT NULL
password_hash     | VARCHAR(255)      | NOT NULL
first_name        | VARCHAR(100)      | NOT NULL
last_name         | VARCHAR(100)      | NOT NULL
phone_number      | VARCHAR(20)       | NULLABLE
role              | ENUM/VARCHAR(20)  | NOT NULL, CHECK IN ('employee', 'manager')
created_at        | TIMESTAMP         | NOT NULL, DEFAULT CURRENT_TIMESTAMP
updated_at        | TIMESTAMP         | NOT NULL, DEFAULT CURRENT_TIMESTAMP
deleted_at        | TIMESTAMP         | NULLABLE (soft delete)
--------------------------------------------------

INDEXES:
  - PRIMARY KEY on id
  - UNIQUE INDEX on email
  - INDEX on role WHERE deleted_at IS NULL
  - INDEX on created_at
```

**Teams Table:**

```sql
TABLE: teams
--------------------------------------------------
COLUMN NAME       | DATA TYPE         | CONSTRAINTS
--------------------------------------------------
id                | INTEGER/UUID      | PRIMARY KEY, AUTO_INCREMENT
name              | VARCHAR(255)      | UNIQUE, NOT NULL
description       | TEXT              | NULLABLE
manager_id        | INTEGER/UUID      | FOREIGN KEY → users(id), NULLABLE
created_at        | TIMESTAMP         | NOT NULL, DEFAULT CURRENT_TIMESTAMP
updated_at        | TIMESTAMP         | NOT NULL, DEFAULT CURRENT_TIMESTAMP
deleted_at        | TIMESTAMP         | NULLABLE (soft delete)
--------------------------------------------------

FOREIGN KEYS:
  - manager_id REFERENCES users(id) ON DELETE SET NULL

INDEXES:
  - PRIMARY KEY on id
  - UNIQUE INDEX on name WHERE deleted_at IS NULL
  - INDEX on manager_id
```

**Team_Members Table:**

```sql
TABLE: team_members
--------------------------------------------------
COLUMN NAME       | DATA TYPE         | CONSTRAINTS
--------------------------------------------------
id                | INTEGER/UUID      | PRIMARY KEY, AUTO_INCREMENT
user_id           | INTEGER/UUID      | FOREIGN KEY → users(id), NOT NULL
team_id           | INTEGER/UUID      | FOREIGN KEY → teams(id), NOT NULL
joined_at         | TIMESTAMP         | NOT NULL, DEFAULT CURRENT_TIMESTAMP
--------------------------------------------------

FOREIGN KEYS:
  - user_id REFERENCES users(id) ON DELETE CASCADE
  - team_id REFERENCES teams(id) ON DELETE CASCADE

CONSTRAINTS:
  - UNIQUE (user_id, team_id)  -- Prevent duplicate memberships

INDEXES:
  - PRIMARY KEY on id
  - INDEX on user_id
  - INDEX on team_id
  - UNIQUE INDEX on (user_id, team_id)
```

**Clocks Table:**

```sql
TABLE: clocks
--------------------------------------------------
COLUMN NAME       | DATA TYPE         | CONSTRAINTS
--------------------------------------------------
id                | INTEGER/UUID      | PRIMARY KEY, AUTO_INCREMENT
user_id           | INTEGER/UUID      | FOREIGN KEY → users(id), NOT NULL
time              | TIMESTAMP(TZ)     | NOT NULL (UTC timezone)
status            | ENUM/VARCHAR(20)  | NOT NULL, CHECK IN ('arrival', 'departure')
created_at        | TIMESTAMP         | NOT NULL, DEFAULT CURRENT_TIMESTAMP
--------------------------------------------------

FOREIGN KEYS:
  - user_id REFERENCES users(id) ON DELETE CASCADE

INDEXES:
  - PRIMARY KEY on id
  - INDEX on (user_id, time DESC)  -- Most common query pattern
  - INDEX on time  -- For date range queries
  - INDEX on (user_id, status, time)  -- For status-specific queries

BUSINESS LOGIC CONSTRAINTS (application-level):
  - Cannot have two consecutive 'arrival' statuses
  - Cannot have 'departure' without prior 'arrival'
  - Time cannot be in the future
```

**Refresh_Tokens Table (Optional):**

```sql
TABLE: refresh_tokens
--------------------------------------------------
COLUMN NAME       | DATA TYPE         | CONSTRAINTS
--------------------------------------------------
id                | INTEGER/UUID      | PRIMARY KEY, AUTO_INCREMENT
user_id           | INTEGER/UUID      | FOREIGN KEY → users(id), NOT NULL
token_hash        | VARCHAR(255)      | UNIQUE, NOT NULL
expires_at        | TIMESTAMP         | NOT NULL
created_at        | TIMESTAMP         | NOT NULL, DEFAULT CURRENT_TIMESTAMP
revoked_at        | TIMESTAMP         | NULLABLE
--------------------------------------------------

FOREIGN KEYS:
  - user_id REFERENCES users(id) ON DELETE CASCADE

INDEXES:
  - PRIMARY KEY on id
  - UNIQUE INDEX on token_hash
  - INDEX on user_id
  - INDEX on expires_at WHERE revoked_at IS NULL
```

### Derived/Computed Data

**Working Hours Summary (View or Materialized View):**

```sql
VIEW/MATERIALIZED VIEW: working_hours_daily
--------------------------------------------------
Calculates daily working hours per user

SELECT
  user_id,
  DATE(time) as work_date,
  MIN(CASE WHEN status = 'arrival' THEN time END) as first_arrival,
  MAX(CASE WHEN status = 'departure' THEN time END) as last_departure,
  -- Total hours worked (assuming paired arrival/departure)
  EXTRACT(HOURS FROM (
    MAX(CASE WHEN status = 'departure' THEN time END) -
    MIN(CASE WHEN status = 'arrival' THEN time END)
  )) as hours_worked
FROM clocks
GROUP BY user_id, DATE(time);

Purpose:
  - Pre-calculate daily working hours
  - Improve report query performance
  - Cache complex aggregations

Refresh Strategy:
  - Materialized View: Refresh daily at midnight
  - Regular View: Computed on-demand
```

---

## Data Integrity Rules

### Referential Integrity

**Cascade Rules:**

```
users → clocks
  ON DELETE CASCADE
  Reason: When user is deleted, all their clocks should be removed

users → teams (as manager)
  ON DELETE SET NULL
  Reason: Team continues to exist if manager leaves

users → team_members
  ON DELETE CASCADE
  Reason: Remove user from all teams when account deleted

teams → team_members
  ON DELETE CASCADE
  Reason: Remove all memberships when team deleted
```

### Business Rules (Application-Level Constraints)

**Clock Validation Rules:**

```
Rule 1: Arrival/Departure Alternation
  - User's clock sequence must alternate: arrival → departure → arrival
  - Implementation: Check last clock status before creating new clock

Rule 2: Future Time Prevention
  - Clock time cannot be in the future
  - Implementation: Compare clock time with current server time

Rule 3: No Retroactive Clocks (Optional)
  - Clock time cannot be more than 24 hours in the past
  - Implementation: Compare with current time - 24 hours

Rule 4: Working Hours Limit (Optional)
  - Single work session cannot exceed 16 hours
  - Implementation: Calculate duration between arrival and departure
```

**User Validation Rules:**

```
Rule 1: Email Uniqueness
  - Email must be unique across all users (including soft-deleted)
  - Implementation: Unique constraint on email column

Rule 2: Password Strength (Application Level)
  - Minimum 8 characters
  - Must contain letters and numbers (optional)
  - Implementation: Validate before hashing

Rule 3: Role Validity
  - Role must be 'employee' or 'manager'
  - Implementation: ENUM or CHECK constraint
```

**Team Validation Rules:**

```
Rule 1: Manager Role Requirement
  - User assigned as manager must have 'manager' role
  - Implementation: Application-level check before assignment

Rule 2: Team Name Uniqueness
  - Active teams must have unique names
  - Implementation: Partial unique index WHERE deleted_at IS NULL
```

### Data Consistency Rules

**Transaction Boundaries:**

```
Transaction 1: Create User with Team Assignment
  BEGIN TRANSACTION
    1. INSERT INTO users (...)
    2. INSERT INTO team_members (user_id, team_id)
  COMMIT

Transaction 2: Clock In/Out
  BEGIN TRANSACTION
    1. SELECT last clock status
    2. Validate next status
    3. INSERT INTO clocks (...)
  COMMIT

Transaction 3: Delete User (Soft Delete)
  BEGIN TRANSACTION
    1. UPDATE users SET deleted_at = NOW() WHERE id = ?
    2. DELETE FROM team_members WHERE user_id = ?
  COMMIT
```

---

## Indexing Strategy

### Primary Indexes

**Performance-Critical Indexes:**

```sql
-- Users table
CREATE INDEX idx_users_email ON users(email);
  -- Usage: Login queries (SELECT * FROM users WHERE email = ?)
  -- Frequency: Every login (~1000/day)
  -- Cardinality: High (unique emails)

CREATE INDEX idx_users_role ON users(role) WHERE deleted_at IS NULL;
  -- Usage: List all managers/employees
  -- Frequency: Medium (~100/day)
  -- Cardinality: Low (2 roles)
  -- Partial: Exclude soft-deleted users

-- Clocks table (MOST CRITICAL)
CREATE INDEX idx_clocks_user_time ON clocks(user_id, time DESC);
  -- Usage: Get user's clocks ordered by time
  -- Frequency: Very High (~10,000/day)
  -- Cardinality: High
  -- Composite: Supports WHERE user_id = ? ORDER BY time DESC

CREATE INDEX idx_clocks_time ON clocks(time);
  -- Usage: Date range queries for reports
  -- Frequency: Medium (~500/day)
  -- Cardinality: High

CREATE INDEX idx_clocks_user_status_time ON clocks(user_id, status, time);
  -- Usage: Get last arrival/departure for user
  -- Frequency: High (~5,000/day)
  -- Cardinality: Very High

-- Teams table
CREATE INDEX idx_teams_manager ON teams(manager_id);
  -- Usage: Get all teams managed by user
  -- Frequency: Medium (~200/day)
  -- Cardinality: Medium

-- Team_members table
CREATE INDEX idx_team_members_user ON team_members(user_id);
CREATE INDEX idx_team_members_team ON team_members(team_id);
  -- Usage: Get teams for user / users in team
  -- Frequency: High (~2,000/day)
  -- Cardinality: Medium
```

### Composite Index Rationale

**Why (user_id, time DESC) instead of separate indexes?**

```
Query: SELECT * FROM clocks WHERE user_id = 123 ORDER BY time DESC LIMIT 10

With separate indexes:
  1. Use user_id index → get all rows for user (could be 1000s)
  2. Sort result by time → expensive operation
  Total: Index seek + full result sort

With composite index:
  1. Seek directly to (user_id=123, latest time)
  2. Scan forward 10 rows
  Total: Index seek + 10 row reads

Performance improvement: 10-100x faster for this critical query
```

### Index Maintenance

**Monitoring:**
- Analyze index usage monthly
- Identify unused indexes (remove to save space)
- Check for index bloat (rebuild if needed)
- Update statistics regularly for query optimizer

**Considerations:**
- Indexes slow down writes (INSERT/UPDATE/DELETE)
- Balance read performance vs write performance
- Clocks table is append-only (only INSERT) → indexes are cheap

---

## Query Patterns

### Frequent Queries (Top 10)

**1. User Login (Critical):**
```sql
SELECT id, email, password_hash, role
FROM users
WHERE email = ? AND deleted_at IS NULL;

Index Used: idx_users_email
Frequency: Very High (every login)
Performance Target: < 10ms
```

**2. Get User's Last Clock (Critical):**
```sql
SELECT id, time, status
FROM clocks
WHERE user_id = ?
ORDER BY time DESC
LIMIT 1;

Index Used: idx_clocks_user_time
Frequency: Very High (every clock in/out)
Performance Target: < 20ms
```

**3. Get User's Clocks for Date Range:**
```sql
SELECT id, time, status
FROM clocks
WHERE user_id = ?
  AND time >= ?
  AND time <= ?
ORDER BY time DESC;

Index Used: idx_clocks_user_time
Frequency: High (dashboard loads)
Performance Target: < 100ms
```

**4. Calculate Daily Working Hours:**
```sql
SELECT
  DATE(time) as date,
  SUM(CASE
    WHEN status = 'departure' THEN EXTRACT(EPOCH FROM time)
    WHEN status = 'arrival' THEN -EXTRACT(EPOCH FROM time)
  END) / 3600 as hours
FROM clocks
WHERE user_id = ?
  AND time >= ?
  AND time <= ?
GROUP BY DATE(time);

Index Used: idx_clocks_user_time
Frequency: Medium (report generation)
Performance Target: < 500ms
Optimization: Pre-calculate in materialized view
```

**5. Get Team Members:**
```sql
SELECT u.id, u.first_name, u.last_name, u.email, u.role
FROM users u
JOIN team_members tm ON u.id = tm.user_id
WHERE tm.team_id = ?
  AND u.deleted_at IS NULL;

Index Used: idx_team_members_team + users PK
Frequency: High (manager dashboard)
Performance Target: < 100ms
```

**6. Get User's Teams:**
```sql
SELECT t.id, t.name, t.description
FROM teams t
JOIN team_members tm ON t.id = tm.team_id
WHERE tm.user_id = ?
  AND t.deleted_at IS NULL;

Index Used: idx_team_members_user + teams PK
Frequency: Medium (profile page)
Performance Target: < 50ms
```

**7. Get Teams Managed by User:**
```sql
SELECT id, name, description
FROM teams
WHERE manager_id = ?
  AND deleted_at IS NULL;

Index Used: idx_teams_manager
Frequency: High (manager dashboard)
Performance Target: < 50ms
```

**8. Count Active Users:**
```sql
SELECT COUNT(*)
FROM users
WHERE deleted_at IS NULL;

Index Used: idx_users_role (partial index covers this)
Frequency: Low (admin dashboard)
Performance Target: < 100ms
```

**9. Team Working Hours Summary:**
```sql
SELECT
  u.id,
  u.first_name,
  u.last_name,
  SUM(/* hours calculation */) as total_hours
FROM users u
JOIN team_members tm ON u.id = tm.user_id
JOIN clocks c ON u.id = c.user_id
WHERE tm.team_id = ?
  AND c.time >= ?
  AND c.time <= ?
GROUP BY u.id, u.first_name, u.last_name;

Index Used: Multiple (complex join)
Frequency: Medium (team reports)
Performance Target: < 2 seconds
Optimization: Materialized view recommended
```

**10. Lateness Rate KPI:**
```sql
-- Count arrivals after expected time (e.g., 9:00 AM)
SELECT
  COUNT(CASE WHEN EXTRACT(HOUR FROM time) > 9 THEN 1 END) * 100.0 /
  COUNT(*) as lateness_rate
FROM clocks
WHERE user_id = ?
  AND status = 'arrival'
  AND time >= ?
  AND time <= ?;

Index Used: idx_clocks_user_status_time
Frequency: Medium (KPI reports)
Performance Target: < 500ms
```

---

## Data Migration Strategy

### Migration Workflow

**Schema Migrations:**

```
Version Control for Database Schema:
  - Each schema change = new migration file
  - Migration files numbered sequentially (001, 002, ...)
  - Up and down migrations for rollback capability
  - Never modify existing migrations (create new ones)

Migration File Structure:
  migrations/
    001_create_users_table.sql
    002_create_teams_table.sql
    003_create_clocks_table.sql
    004_create_team_members_table.sql
    005_add_user_role_index.sql
    ...

Migration Metadata Table:
  CREATE TABLE schema_migrations (
    version INTEGER PRIMARY KEY,
    applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
  );
```

**Sample Migration Files:**

```sql
-- 001_create_users_table.sql (UP)
CREATE TABLE users (
  id INTEGER PRIMARY KEY AUTO_INCREMENT,
  email VARCHAR(255) UNIQUE NOT NULL,
  password_hash VARCHAR(255) NOT NULL,
  first_name VARCHAR(100) NOT NULL,
  last_name VARCHAR(100) NOT NULL,
  phone_number VARCHAR(20),
  role VARCHAR(20) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMP NULL
);

CREATE INDEX idx_users_email ON users(email);

-- 001_create_users_table.sql (DOWN)
DROP INDEX idx_users_email;
DROP TABLE users;
```

**Migration Execution:**

```bash
# Run all pending migrations
./migrate up

# Rollback last migration
./migrate down

# Check migration status
./migrate status

# Rollback to specific version
./migrate down-to 003
```

### Data Seeding

**Development Seed Data:**

```sql
-- Seed managers
INSERT INTO users (email, password_hash, first_name, last_name, role) VALUES
('manager1@company.com', '$2b$12$...', 'John', 'Manager', 'manager'),
('manager2@company.com', '$2b$12$...', 'Jane', 'Boss', 'manager');

-- Seed employees
INSERT INTO users (email, password_hash, first_name, last_name, role) VALUES
('alice@company.com', '$2b$12$...', 'Alice', 'Smith', 'employee'),
('bob@company.com', '$2b$12$...', 'Bob', 'Johnson', 'employee'),
('charlie@company.com', '$2b$12$...', 'Charlie', 'Brown', 'employee');

-- Seed teams
INSERT INTO teams (name, description, manager_id) VALUES
('Engineering', 'Software development team', 1),
('Marketing', 'Marketing and sales team', 2);

-- Seed team members
INSERT INTO team_members (user_id, team_id) VALUES
(3, 1), (4, 1), (5, 2);

-- Seed sample clocks (last 30 days)
INSERT INTO clocks (user_id, time, status) VALUES
(3, '2025-10-06 08:30:00+00', 'arrival'),
(3, '2025-10-06 17:00:00+00', 'departure'),
(4, '2025-10-06 09:00:00+00', 'arrival'),
(4, '2025-10-06 18:00:00+00', 'departure');
```

---

## Performance Considerations

### Query Optimization Techniques

**1. Index Coverage:**
```sql
-- Bad: Requires table lookup
SELECT * FROM clocks WHERE user_id = 123;

-- Good: Covered by index
SELECT id, time, status FROM clocks WHERE user_id = 123;
```

**2. Avoid N+1 Queries:**
```sql
-- Bad: N+1 query (1 team query + N user queries)
teams = SELECT * FROM teams;
for each team:
  manager = SELECT * FROM users WHERE id = team.manager_id;

-- Good: Single JOIN query
SELECT t.*, u.first_name, u.last_name
FROM teams t
LEFT JOIN users u ON t.manager_id = u.id;
```

**3. Pagination:**
```sql
-- Paginate large result sets
SELECT * FROM clocks
WHERE user_id = 123
ORDER BY time DESC
LIMIT 20 OFFSET 0;  -- Page 1

LIMIT 20 OFFSET 20; -- Page 2
```

**4. Materialized Views for Heavy Queries:**
```sql
-- Pre-calculate expensive aggregations
CREATE MATERIALIZED VIEW monthly_working_hours AS
SELECT
  user_id,
  DATE_TRUNC('month', time) as month,
  SUM(/* hours calculation */) as total_hours
FROM clocks
GROUP BY user_id, DATE_TRUNC('month', time);

-- Refresh daily
REFRESH MATERIALIZED VIEW monthly_working_hours;
```

### Database Tuning

**Connection Pooling:**
```
- Pool size: 10-20 connections per application instance
- Max connections: 100 (database server limit)
- Connection timeout: 30 seconds
- Idle timeout: 10 minutes
```

**Cache Configuration:**
```
- Query cache: Enabled for SELECT queries
- Buffer pool: 50-70% of available RAM
- Log buffer: 16-32 MB
```

---

## Backup & Recovery

### Backup Strategy

**Backup Types:**

```
1. Daily Full Backup
   - Schedule: 2:00 AM daily (low usage time)
   - Retention: 30 days
   - Storage: Off-site/cloud storage
   - Size: ~500MB initially, growing

2. Incremental Backup (Optional)
   - Schedule: Every 6 hours
   - Retention: 7 days
   - Purpose: Reduce data loss window

3. Transaction Log Backup
   - Schedule: Every 15 minutes
   - Retention: 7 days
   - Purpose: Point-in-time recovery
```

**Backup Commands (Example):**

```bash
# PostgreSQL backup
pg_dump -U postgres timemanager > backup_$(date +%Y%m%d).sql

# MySQL backup
mysqldump -u root -p timemanager > backup_$(date +%Y%m%d).sql

# Automated backup script
#!/bin/bash
BACKUP_DIR=/backups
DATE=$(date +%Y%m%d_%H%M%S)
pg_dump timemanager | gzip > $BACKUP_DIR/backup_$DATE.sql.gz

# Keep only last 30 days
find $BACKUP_DIR -name "backup_*.sql.gz" -mtime +30 -delete
```

### Recovery Procedures

**Full Database Restore:**

```bash
# PostgreSQL restore
psql -U postgres timemanager < backup_20251006.sql

# MySQL restore
mysql -u root -p timemanager < backup_20251006.sql
```

**Point-in-Time Recovery:**

```bash
# Restore to specific timestamp
# 1. Restore last full backup
# 2. Apply transaction logs up to target time
# 3. Verify data integrity
```

**Disaster Recovery Plan:**

```
RTO (Recovery Time Objective): < 4 hours
RPO (Recovery Point Objective): < 24 hours

Recovery Steps:
  1. Identify failure (5 minutes)
  2. Provision new database server (30 minutes)
  3. Restore from latest backup (1 hour)
  4. Apply transaction logs (30 minutes)
  5. Verify data integrity (1 hour)
  6. Update application configuration (30 minutes)
  7. Resume service (15 minutes)

Total: ~3.5 hours (within RTO)
```

---

## Database Selection Criteria Summary

### Decision Matrix

| Requirement | PostgreSQL | MariaDB/MySQL | MongoDB |
|-------------|------------|---------------|---------|
| **Relational Model** | ✅ Excellent | ✅ Excellent | ⚠️ Limited |
| **Time Functions** | ✅ Best (INTERVAL, AGE) | ✅ Good | ⚠️ Manual |
| **ACID Transactions** | ✅ Full | ✅ Full | ⚠️ Partial |
| **Complex Queries** | ✅ Window functions | ✅ Basic | ⚠️ Aggregation pipeline |
| **Materialized Views** | ✅ Native | ❌ Manual | ❌ Manual |
| **JSON Support** | ✅ Native (JSONB) | ✅ JSON type | ✅ Native |
| **Soft Delete Support** | ✅ Partial indexes | ✅ WHERE clause | ✅ Query filters |
| **Time-Series** | ✅ TimescaleDB ext | ⚠️ Manual | ✅ Good |

**Recommendation**: PostgreSQL or MariaDB/MySQL (both excellent for this use case)

---

**Document Status**: Living document - Update when database technology is selected
**Review Frequency**: Before implementation, after major schema changes
**Owner**: Backend Team
