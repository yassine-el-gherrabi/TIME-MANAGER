# Time Manager - Architecture Documentation

## 📐 System Architecture Overview

Time Manager follows a **microservices-inspired architecture** with clear separation of concerns between frontend, backend, API gateway, and database layers.

```
┌─────────────────────────────────────────────────────────────┐
│                         CLIENT                              │
│                    (Web Browser)                            │
└────────────────────────┬────────────────────────────────────┘
                         │ HTTP/HTTPS
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                    KRAKEND (Port 8000)                      │
│              API Gateway & Reverse Proxy                    │
│  • Rate Limiting  • CORS  • Request Routing                 │
└──────────────┬─────────────────────────┬────────────────────┘
               │                         │
      /api/*   │                         │  /*
               ▼                         ▼
┌──────────────────────────┐   ┌────────────────────────────┐
│   BACKEND (Port 8080)    │   │   FRONTEND (Port 3000)     │
│         Go API           │   │      React SPA             │
│  • REST Endpoints        │   │  • React Router v6         │
│  • JWT Authentication    │   │  • Context API             │
│  • Business Logic        │   │  • Magic UI Components     │
│  • Layered Architecture  │   │  • Axios HTTP Client       │
└───────────┬──────────────┘   └────────────────────────────┘
            │
            │ SQL Queries
            ▼
┌─────────────────────────────────────────────────────────────┐
│              DATABASE (Port 5432)                           │
│                   PostgreSQL                                │
│  • Relational Schema  • Migrations  • Indexes               │
└─────────────────────────────────────────────────────────────┘
```

### Architecture Layers

1. **Client Layer**: Modern web browsers (Chrome, Firefox, Safari, Edge)
2. **Gateway Layer**: KrakenD for routing, security, and traffic management
3. **Frontend Layer**: React single-page application for user interface
4. **Backend Layer**: Go REST API for business logic and data processing
5. **Database Layer**: PostgreSQL for persistent data storage

---

## 🔄 Data Flow

### 1. User Authentication Flow

```
User Login Request
    ↓
Frontend → POST /api/auth/login (email, password)
    ↓
KrakenD → Routes to Backend
    ↓
Backend → Handler Layer
    ↓
Backend → Service Layer (validate credentials, bcrypt comparison)
    ↓
Backend → Repository Layer (query user from database)
    ↓
Database → Returns user data
    ↓
Backend → Service Layer (generate JWT access + refresh tokens)
    ↓
Backend → Response {accessToken, refreshToken, user}
    ↓
KrakenD → Forward response
    ↓
Frontend → Store tokens, redirect to dashboard
```

### 2. Protected Resource Access Flow

```
User Action (e.g., Clock In)
    ↓
Frontend → GET /api/clocks (Authorization: Bearer <token>)
    ↓
KrakenD → Routes to Backend
    ↓
Backend → Auth Middleware (verify JWT signature, extract user ID)
    ↓
Backend → Handler Layer
    ↓
Backend → Service Layer (business logic)
    ↓
Backend → Repository Layer (query/insert database)
    ↓
Database → Return data
    ↓
Backend → Response with data
    ↓
KrakenD → Forward response
    ↓
Frontend → Update UI state
```

### 3. Data Update Flow (Clock In/Out)

```
User Clicks "Clock In" Button
    ↓
Frontend → POST /api/clocks {time: timestamp, status: true}
    ↓
KrakenD → Routes to Backend
    ↓
Backend → Auth Middleware (verify user)
    ↓
Backend → Handler Layer (parse request)
    ↓
Backend → Service Layer
    ├─ Validate: No active clock-in exists
    ├─ Business logic: Create new clock entry
    └─ Call repository
    ↓
Backend → Repository Layer (INSERT INTO clocks)
    ↓
Database → Persist data, return new record
    ↓
Backend → Response {success, clockData}
    ↓
Frontend → Update UI, show success notification
```

---

## 🏗️ Backend Architecture (Go)

### Layered Architecture Pattern

```
┌─────────────────────────────────────────────────┐
│                  HTTP Handler                    │
│  • Request parsing                              │
│  • Response formatting                          │
│  • Input validation                             │
│  • Error handling                               │
└────────────────────┬────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────┐
│                Service Layer                     │
│  • Business logic                               │
│  • Data transformation                          │
│  • Authorization rules                          │
│  • Transaction management                       │
└────────────────────┬────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────┐
│              Repository Layer                    │
│  • Database queries                             │
│  • ORM operations                               │
│  • Data persistence                             │
│  • Query optimization                           │
└────────────────────┬────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────┐
│                 Model Layer                      │
│  • Data structures                              │
│  • Validation rules                             │
│  • Database mappings                            │
└─────────────────────────────────────────────────┘
```

### Backend Directory Structure

```
back/
├── cmd/
│   └── api/
│       └── main.go              # Application entrypoint
├── internal/
│   ├── api/
│   │   ├── handlers/            # HTTP request handlers
│   │   │   ├── auth.go          # Authentication endpoints
│   │   │   ├── users.go         # User CRUD endpoints
│   │   │   ├── clocks.go        # Clock in/out endpoints
│   │   │   ├── teams.go         # Team management endpoints
│   │   │   └── reports.go       # Reports and KPIs endpoints
│   │   ├── middleware/          # HTTP middleware
│   │   │   ├── auth.go          # JWT authentication
│   │   │   ├── cors.go          # CORS configuration
│   │   │   ├── logger.go        # Request logging
│   │   │   └── ratelimit.go     # Rate limiting
│   │   └── router.go            # Route definitions
│   ├── auth/
│   │   ├── jwt.go               # JWT token generation/validation
│   │   ├── password.go          # Password hashing (bcrypt)
│   │   └── permissions.go       # Role-based access control
│   ├── service/
│   │   ├── user_service.go      # User business logic
│   │   ├── clock_service.go     # Clock business logic
│   │   ├── team_service.go      # Team business logic
│   │   └── report_service.go    # Report generation logic
│   ├── repository/
│   │   ├── user_repo.go         # User database operations
│   │   ├── clock_repo.go        # Clock database operations
│   │   ├── team_repo.go         # Team database operations
│   │   └── database.go          # Database connection setup
│   └── models/
│       ├── user.go              # User model
│       ├── clock.go             # Clock model
│       ├── team.go              # Team model
│       └── common.go            # Shared models (pagination, etc.)
├── pkg/
│   ├── config/                  # Configuration management
│   ├── logger/                  # Logging utilities
│   └── validator/               # Input validation
├── migrations/                  # Database migrations
│   ├── 001_create_users.sql
│   ├── 002_create_clocks.sql
│   └── 003_create_teams.sql
├── tests/                       # Backend tests
│   ├── integration/
│   └── unit/
├── Dockerfile                   # Backend container
├── go.mod                       # Go dependencies
└── go.sum                       # Dependency checksums
```

### Key Backend Patterns

**1. Dependency Injection**: Services receive repository dependencies via constructors
**2. Interface Abstraction**: Repositories implement interfaces for testability
**3. Error Handling**: Custom error types with HTTP status code mapping
**4. Middleware Chain**: Authentication → Logging → Rate Limiting → Handler
**5. Transaction Management**: Service layer manages database transactions

---

## 🎨 Frontend Architecture (React)

### Component Architecture

```
┌─────────────────────────────────────────────────┐
│                   App.jsx                        │
│            (Root Component)                      │
│  • React Router v6 setup                        │
│  • Context Providers                            │
│  • Global error boundary                        │
└────────────────────┬────────────────────────────┘
                     │
        ┌────────────┴────────────┐
        ▼                         ▼
┌─────────────────┐    ┌─────────────────────┐
│     Pages       │    │     Contexts         │
│  • Dashboard    │    │  • AuthContext       │
│  • Login        │    │  • ThemeContext      │
│  • Teams        │    │  • NotificationCtx   │
│  • Profile      │    └─────────────────────┘
│  • Reports      │
└────────┬────────┘
         │
    ┌────┴──────────────┐
    ▼                   ▼
┌──────────┐    ┌─────────────────┐
│Components│    │      Hooks       │
│ • Common │    │  • useAuth       │
│ • Layout │    │  • useClocks     │
│ • Forms  │    │  • useTeams      │
│ • Charts │    │  • useDebounce   │
└──────────┘    └─────────────────┘
```

### Frontend Directory Structure

```
front/
├── public/
│   ├── index.html
│   ├── favicon.ico
│   └── manifest.json
├── src/
│   ├── api/                     # API service layer
│   │   ├── authApi.js           # Authentication API calls
│   │   ├── userApi.js           # User API calls
│   │   ├── clockApi.js          # Clock API calls
│   │   ├── teamApi.js           # Team API calls
│   │   ├── reportApi.js         # Report API calls
│   │   └── axiosConfig.js       # Axios instance configuration
│   ├── components/
│   │   ├── common/              # Reusable components
│   │   │   ├── Button.jsx
│   │   │   ├── Input.jsx
│   │   │   ├── Card.jsx
│   │   │   ├── Modal.jsx
│   │   │   └── Spinner.jsx
│   │   ├── layout/              # Layout components
│   │   │   ├── Header.jsx
│   │   │   ├── Sidebar.jsx
│   │   │   ├── Footer.jsx
│   │   │   └── Layout.jsx
│   │   ├── auth/                # Authentication components
│   │   │   ├── LoginForm.jsx
│   │   │   ├── RegisterForm.jsx
│   │   │   └── ProtectedRoute.jsx
│   │   ├── clock/               # Clock components
│   │   │   ├── ClockInOutButton.jsx
│   │   │   ├── ClockHistory.jsx
│   │   │   └── ClockChart.jsx
│   │   └── team/                # Team components
│   │       ├── TeamList.jsx
│   │       ├── TeamMemberCard.jsx
│   │       └── TeamForm.jsx
│   ├── contexts/                # React Context API
│   │   ├── AuthContext.jsx      # Authentication state
│   │   ├── ThemeContext.jsx     # Theme management
│   │   └── NotificationContext.jsx
│   ├── hooks/                   # Custom React hooks
│   │   ├── useAuth.js           # Authentication hook
│   │   ├── useClocks.js         # Clock data hook
│   │   ├── useTeams.js          # Team data hook
│   │   ├── useDebounce.js       # Debounce utility
│   │   └── useForm.js           # Form handling hook
│   ├── pages/                   # Page components
│   │   ├── Dashboard.jsx        # Main dashboard
│   │   ├── Login.jsx            # Login page
│   │   ├── Register.jsx         # Registration page
│   │   ├── Profile.jsx          # User profile
│   │   ├── Teams.jsx            # Team management
│   │   ├── Reports.jsx          # Reports and KPIs
│   │   └── NotFound.jsx         # 404 page
│   ├── routes/                  # Routing configuration
│   │   └── AppRoutes.jsx        # Route definitions
│   ├── utils/                   # Utility functions
│   │   ├── dateUtils.js         # Date formatting
│   │   ├── validators.js        # Form validation
│   │   └── constants.js         # App constants
│   ├── styles/                  # Global styles
│   │   └── globals.css          # Tailwind + custom CSS
│   ├── App.jsx                  # Root component
│   └── index.js                 # Application entrypoint
├── Dockerfile                   # Frontend container
├── package.json                 # NPM dependencies
└── tailwind.config.js           # Tailwind configuration
```

### Key Frontend Patterns

**1. Container/Presentational Pattern**: Separate data logic from presentation
**2. Custom Hooks**: Reusable logic extraction (useAuth, useClocks, useTeams)
**3. Context API**: Global state management for auth, theme, notifications
**4. Protected Routes**: HOC for authentication-required pages
**5. API Service Layer**: Centralized API calls with Axios interceptors

---

## 🗄️ Database Schema

### Entity Relationship Diagram

```
┌─────────────────────────┐
│         Users           │
│─────────────────────────│
│ id (PK)                 │
│ email                   │
│ password_hash           │
│ first_name              │
│ last_name               │
│ role (employee/manager) │
│ team_id (FK)            │
│ created_at              │
│ updated_at              │
└───────────┬─────────────┘
            │
            │ 1:N
            ▼
┌─────────────────────────┐
│        Clocks           │
│─────────────────────────│
│ id (PK)                 │
│ user_id (FK)            │
│ time (timestamp)        │
│ status (in/out)         │
│ created_at              │
└─────────────────────────┘

┌─────────────────────────┐
│         Teams           │
│─────────────────────────│
│ id (PK)                 │
│ name                    │
│ manager_id (FK)         │
│ created_at              │
│ updated_at              │
└─────────────────────────┘
```

### Tables

**users**
- Primary Key: `id` (serial)
- Unique: `email`
- Indexed: `email`, `team_id`
- Relationships: N:1 with teams, 1:N with clocks

**clocks**
- Primary Key: `id` (serial)
- Foreign Key: `user_id` → users(id)
- Indexed: `user_id`, `time`
- Composite Index: (user_id, time) for efficient querying

**teams**
- Primary Key: `id` (serial)
- Foreign Key: `manager_id` → users(id)
- Unique: `name`
- Relationships: 1:N with users

---

## 🔐 Security Architecture

### Authentication & Authorization

**JWT Token System**:
```
Access Token:
  • Lifetime: 24 hours
  • Payload: {user_id, email, role}
  • Storage: Frontend memory (not localStorage)
  • Transmitted: Authorization header (Bearer token)

Refresh Token:
  • Lifetime: 7 days
  • Payload: {user_id, token_version}
  • Storage: HttpOnly cookie (secure)
  • Purpose: Obtain new access token
```

**Password Security**:
- Algorithm: bcrypt (cost factor: 12)
- Salt: Auto-generated per password
- Never stored or transmitted in plaintext

**Role-Based Access Control (RBAC)**:
```
Employee Role:
  • Read own clocks
  • Create/update own clocks
  • Read own profile
  • Update own profile

Manager Role (extends Employee):
  • Read all users in team
  • Read all clocks in team
  • Create/update team members
  • Generate team reports
```

### Security Best Practices

1. **CORS**: Strict origin validation via KrakenD
2. **Rate Limiting**: Prevent brute force attacks (100 req/min per IP)
3. **Input Validation**: Server-side validation for all endpoints
4. **SQL Injection Prevention**: Parameterized queries only
5. **XSS Prevention**: Content-Security-Policy headers
6. **HTTPS Only**: TLS 1.3 in production
7. **Environment Variables**: Secrets never hardcoded

---

## 🔌 API Design

### REST Principles

**Base URL**: `http://localhost:8000/api` (via KrakenD)

**HTTP Methods**:
- GET: Retrieve resources
- POST: Create resources
- PUT: Update resources (full)
- PATCH: Partial updates
- DELETE: Remove resources

### Endpoint Structure

```
Authentication:
POST   /api/auth/register        # User registration
POST   /api/auth/login           # User login
POST   /api/auth/refresh         # Refresh access token
POST   /api/auth/logout          # User logout

Users:
GET    /api/users                # List users (manager only)
GET    /api/users/:id            # Get user details
PUT    /api/users/:id            # Update user
DELETE /api/users/:id            # Delete user (manager only)

Clocks:
GET    /api/clocks               # List clocks (filtered by role)
GET    /api/clocks/:id           # Get clock details
POST   /api/clocks               # Clock in/out
PUT    /api/clocks/:id           # Update clock entry
DELETE /api/clocks/:id           # Delete clock entry

Teams:
GET    /api/teams                # List teams
GET    /api/teams/:id            # Get team details
POST   /api/teams                # Create team (manager only)
PUT    /api/teams/:id            # Update team (manager only)
DELETE /api/teams/:id            # Delete team (manager only)

Reports:
GET    /api/reports/kpis         # Get KPI dashboard (manager only)
GET    /api/reports/user/:id     # User working time report
GET    /api/reports/team/:id     # Team performance report
```

### API Response Format

**Success Response**:
```json
{
  "success": true,
  "data": { /* resource data */ },
  "message": "Operation successful"
}
```

**Error Response**:
```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid email format",
    "details": [/* validation errors */]
  }
}
```

**Pagination**:
```json
{
  "success": true,
  "data": [/* items */],
  "pagination": {
    "page": 1,
    "pageSize": 10,
    "total": 150,
    "totalPages": 15
  }
}
```

---

## 🚀 Deployment Architecture

### Docker Compose Configuration

```
┌────────────────────────────────────────────────┐
│            Docker Compose Network              │
│                                                │
│  ┌──────────────┐      ┌──────────────┐       │
│  │   KrakenD    │◄─────┤  Frontend    │       │
│  │  (Gateway)   │      │   (React)    │       │
│  │  Port: 8000  │      │  Port: 3000  │       │
│  └──────┬───────┘      └──────────────┘       │
│         │                                      │
│         │ Proxy                                │
│         │                                      │
│  ┌──────▼───────┐      ┌──────────────┐       │
│  │   Backend    │─────►│  Database    │       │
│  │     (Go)     │      │ (PostgreSQL) │       │
│  │  Port: 8080  │      │  Port: 5432  │       │
│  └──────────────┘      └──────────────┘       │
│                                                │
│         Volume Mounts:                         │
│         • db-data → /var/lib/postgresql/data   │
│         • ./back → /app (dev mode)             │
│         • ./front → /app (dev mode)            │
└────────────────────────────────────────────────┘
```

### Container Details

**KrakenD Container**:
- Image: `devopsfaith/krakend:latest`
- Configuration: `krakend.json`
- Purpose: API Gateway, reverse proxy, rate limiting

**Backend Container**:
- Image: `golang:1.21-alpine` (multi-stage build)
- Production: Compiled binary (~15MB)
- Hot Reload: Air for development

**Frontend Container**:
- Image: `node:18-alpine` (multi-stage build)
- Production: Nginx serving static files (~20MB)
- Hot Reload: Vite dev server for development

**Database Container**:
- Image: `postgres:15-alpine`
- Persistent Volume: `db-data`
- Init Scripts: Database migrations

---

## 🛠️ Technology Justifications

### Backend: Go

**Why Go?**
- **Performance**: Compiled language, efficient concurrency with goroutines
- **Simplicity**: Clean syntax, minimal boilerplate, easy maintenance
- **Standard Library**: Robust net/http, database/sql, no heavy dependencies
- **Type Safety**: Static typing reduces runtime errors
- **Deployment**: Single binary, cross-platform compilation
- **Ecosystem**: Excellent tooling (gofmt, golangci-lint, go test)

### Frontend: React 19.2+

**Why React?**
- **Component Model**: Reusable, testable UI components
- **Ecosystem**: Vast library ecosystem (React Router, React Hook Form)
- **Performance**: Virtual DOM, efficient updates, React Server Components
- **Developer Experience**: Hot reload, excellent DevTools
- **Community**: Large community, extensive documentation
- **Magic UI Integration**: Modern, accessible component library

### Gateway: KrakenD

**Why KrakenD?**
- **Performance**: High-throughput, low-latency API gateway
- **Configuration**: Declarative JSON configuration
- **Features**: Built-in rate limiting, CORS, aggregation
- **Lightweight**: No dependencies, stateless design
- **Security**: Request/response manipulation, authentication plugins

### Database: PostgreSQL

**Why PostgreSQL?**
- **Reliability**: ACID compliance, data integrity guarantees
- **Features**: Advanced queries, JSON support, full-text search
- **Performance**: Excellent indexing, query optimization
- **Extensions**: PostGIS, pgcrypto, pg_stat_statements
- **Community**: Mature ecosystem, extensive documentation

---

## 📊 Scalability Considerations

### Horizontal Scaling

**Frontend**: Stateless React app, easy to replicate behind load balancer

**Backend**: Stateless Go API, scale horizontally with Docker replicas
```bash
docker-compose up --scale backend=3
```

**Database**: PostgreSQL read replicas for reporting queries

### Performance Optimizations

**Backend**:
- Connection pooling (pgx/pgxpool)
- Database indexing (user_id, time columns)
- Caching (Redis for session data - future)

**Frontend**:
- Code splitting (React.lazy)
- Asset compression (gzip, brotli)
- CDN for static assets (production)

**API Gateway**:
- Response caching (KrakenD)
- Request batching
- Rate limiting per endpoint

---

## 🧪 Testing Strategy

### Backend Testing

**Unit Tests**: Test business logic in service layer
```bash
go test ./internal/service/... -v
```

**Integration Tests**: Test database operations
```bash
go test ./internal/repository/... -v -tags=integration
```

**Coverage Target**: >70%

### Frontend Testing

**Component Tests**: React Testing Library
```bash
npm test
```

**E2E Tests**: Playwright (future)
```bash
npm run test:e2e
```

**Coverage Target**: >70%

---

## 📚 Related Documentation

- [API Documentation](./api.md) - Complete API endpoint reference
- [KPI Definitions](./kpis.md) - Key Performance Indicator specifications
- [CONTRIBUTING.md](../CONTRIBUTING.md) - Development workflow and standards
- [README.md](../README.md) - Quick start and project overview
