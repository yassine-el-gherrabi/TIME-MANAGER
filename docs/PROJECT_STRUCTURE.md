# Time Manager - Complete Project Structure

> **Stack**: Go (Backend) + React (Frontend) + KrakenD (API Gateway)
> **DevOps**: Docker + Docker Compose + Taskfile
> **Last Updated**: 2025-10-06

---

## Project Tree

```
Time-Manager/
│
├── .github/                           # GitHub configuration
│   └── workflows/                     # CI/CD pipelines
│       ├── backend-ci.yml            # Go backend CI
│       ├── frontend-ci.yml           # React frontend CI
│       └── integration-ci.yml        # Full integration tests
│
├── backend/                           # Go API Backend
│   ├── cmd/                          # Application entry points
│   │   └── api/
│   │       └── main.go               # Main application
│   │
│   ├── internal/                     # Private application code
│   │   ├── config/                   # Configuration management
│   │   │   ├── config.go
│   │   │   └── database.go
│   │   │
│   │   ├── domain/                   # Business logic layer
│   │   │   ├── entities/             # Domain entities
│   │   │   │   ├── user.go
│   │   │   │   ├── team.go
│   │   │   │   ├── clock.go
│   │   │   │   └── refresh_token.go
│   │   │   │
│   │   │   ├── repositories/         # Repository interfaces
│   │   │   │   ├── user_repository.go
│   │   │   │   ├── team_repository.go
│   │   │   │   └── clock_repository.go
│   │   │   │
│   │   │   └── services/             # Business logic services
│   │   │       ├── auth_service.go
│   │   │       ├── user_service.go
│   │   │       ├── team_service.go
│   │   │       ├── clock_service.go
│   │   │       └── report_service.go
│   │   │
│   │   ├── infrastructure/           # External dependencies
│   │   │   ├── database/             # Database implementation
│   │   │   │   ├── postgres.go       # PostgreSQL connection
│   │   │   │   ├── migrations/       # SQL migrations
│   │   │   │   │   ├── 000001_create_users_table.up.sql
│   │   │   │   │   ├── 000001_create_users_table.down.sql
│   │   │   │   │   ├── 000002_create_teams_table.up.sql
│   │   │   │   │   ├── 000002_create_teams_table.down.sql
│   │   │   │   │   ├── 000003_create_clocks_table.up.sql
│   │   │   │   │   ├── 000003_create_clocks_table.down.sql
│   │   │   │   │   ├── 000004_create_team_members_table.up.sql
│   │   │   │   │   ├── 000004_create_team_members_table.down.sql
│   │   │   │   │   ├── 000005_create_refresh_tokens_table.up.sql
│   │   │   │   │   └── 000005_create_refresh_tokens_table.down.sql
│   │   │   │   │
│   │   │   │   └── repositories/     # Repository implementations
│   │   │   │       ├── postgres_user_repository.go
│   │   │   │       ├── postgres_team_repository.go
│   │   │   │       └── postgres_clock_repository.go
│   │   │   │
│   │   │   └── auth/                 # Authentication infrastructure
│   │   │       ├── jwt.go            # JWT token handling
│   │   │       └── password.go       # Password hashing (bcrypt)
│   │   │
│   │   ├── api/                      # HTTP API layer
│   │   │   ├── routes/               # Route definitions
│   │   │   │   └── routes.go
│   │   │   │
│   │   │   ├── handlers/             # HTTP handlers
│   │   │   │   ├── auth_handler.go
│   │   │   │   ├── user_handler.go
│   │   │   │   ├── team_handler.go
│   │   │   │   ├── clock_handler.go
│   │   │   │   └── report_handler.go
│   │   │   │
│   │   │   ├── middleware/           # HTTP middleware
│   │   │   │   ├── auth.go           # JWT validation
│   │   │   │   ├── cors.go           # CORS configuration
│   │   │   │   ├── logger.go         # Request logging
│   │   │   │   └── rate_limiter.go   # Rate limiting
│   │   │   │
│   │   │   ├── requests/             # Request DTOs
│   │   │   │   ├── auth_request.go
│   │   │   │   ├── user_request.go
│   │   │   │   ├── team_request.go
│   │   │   │   └── clock_request.go
│   │   │   │
│   │   │   └── responses/            # Response DTOs
│   │   │       ├── user_response.go
│   │   │       ├── team_response.go
│   │   │       ├── clock_response.go
│   │   │       └── report_response.go
│   │   │
│   │   └── utils/                    # Utility functions
│   │       ├── validator.go          # Input validation
│   │       ├── errors.go             # Error handling
│   │       └── pagination.go         # Pagination helpers
│   │
│   ├── pkg/                          # Public libraries (can be imported)
│   │   └── logger/                   # Custom logger package
│   │       └── logger.go
│   │
│   ├── tests/                        # Test files
│   │   ├── integration/              # Integration tests
│   │   │   ├── auth_test.go
│   │   │   ├── user_test.go
│   │   │   └── clock_test.go
│   │   │
│   │   ├── unit/                     # Unit tests
│   │   │   ├── services/
│   │   │   │   ├── auth_service_test.go
│   │   │   │   └── clock_service_test.go
│   │   │   │
│   │   │   └── handlers/
│   │   │       └── auth_handler_test.go
│   │   │
│   │   ├── fixtures/                 # Test data
│   │   │   └── users.go
│   │   │
│   │   └── testutils/                # Test utilities
│   │       ├── database.go           # Test database helpers
│   │       └── auth.go               # Test auth helpers
│   │
│   ├── scripts/                      # Utility scripts
│   │   ├── seed.go                   # Database seeding
│   │   └── migrate.sh                # Migration helper
│   │
│   ├── .env.example                  # Environment variables template
│   ├── .gitignore                    # Git ignore rules
│   ├── Dockerfile                    # Development Dockerfile
│   ├── Dockerfile.prod               # Production multi-stage Dockerfile
│   ├── Taskfile.yml                  # Backend-specific tasks
│   ├── go.mod                        # Go module dependencies
│   ├── go.sum                        # Go checksum file
│   └── README.md                     # Backend documentation
│
├── frontend/                         # React Frontend
│   ├── public/                       # Static assets
│   │   ├── favicon.ico
│   │   └── robots.txt
│   │
│   ├── src/
│   │   ├── api/                      # API client layer
│   │   │   ├── client.ts             # Axios client (KrakenD)
│   │   │   └── endpoints/            # API endpoint functions
│   │   │       ├── auth.ts
│   │   │       ├── users.ts
│   │   │       ├── teams.ts
│   │   │       ├── clocks.ts
│   │   │       └── reports.ts
│   │   │
│   │   ├── assets/                   # Static assets (images, fonts)
│   │   │   ├── images/
│   │   │   └── fonts/
│   │   │
│   │   ├── components/               # Reusable components
│   │   │   ├── ui/                   # shadcn/ui components
│   │   │   │   ├── button.tsx
│   │   │   │   ├── input.tsx
│   │   │   │   ├── card.tsx
│   │   │   │   ├── dialog.tsx
│   │   │   │   ├── table.tsx
│   │   │   │   └── toast.tsx
│   │   │   │
│   │   │   ├── layout/               # Layout components
│   │   │   │   ├── Header.tsx
│   │   │   │   ├── Sidebar.tsx
│   │   │   │   └── Footer.tsx
│   │   │   │
│   │   │   ├── charts/               # Chart components (Recharts)
│   │   │   │   ├── WorkingHoursChart.tsx
│   │   │   │   └── TeamComparisonChart.tsx
│   │   │   │
│   │   │   ├── tables/               # Table components
│   │   │   │   ├── RecentClocksTable.tsx
│   │   │   │   └── UsersTable.tsx
│   │   │   │
│   │   │   ├── forms/                # Form components
│   │   │   │   ├── LoginForm.tsx
│   │   │   │   └── UserForm.tsx
│   │   │   │
│   │   │   ├── ProtectedRoute.tsx    # Auth route guard
│   │   │   ├── RoleGuard.tsx         # Role-based guard
│   │   │   └── ErrorBoundary.tsx     # Error handling
│   │   │
│   │   ├── features/                 # Feature modules
│   │   │   ├── auth/
│   │   │   │   ├── components/
│   │   │   │   │   └── LoginForm.tsx
│   │   │   │   ├── hooks/
│   │   │   │   │   └── useAuth.ts
│   │   │   │   └── pages/
│   │   │   │       └── LoginPage.tsx
│   │   │   │
│   │   │   ├── clocks/
│   │   │   │   ├── components/
│   │   │   │   │   ├── ClockButton.tsx
│   │   │   │   │   └── ClockHistory.tsx
│   │   │   │   ├── hooks/
│   │   │   │   │   └── useClock.ts
│   │   │   │   └── pages/
│   │   │   │       ├── ClockPage.tsx
│   │   │   │       └── HistoryPage.tsx
│   │   │   │
│   │   │   ├── dashboard/
│   │   │   │   ├── components/
│   │   │   │   │   ├── KPICard.tsx
│   │   │   │   │   └── QuickActions.tsx
│   │   │   │   └── pages/
│   │   │   │       ├── EmployeeDashboard.tsx
│   │   │   │       └── ManagerDashboard.tsx
│   │   │   │
│   │   │   ├── teams/
│   │   │   │   ├── components/
│   │   │   │   │   ├── TeamCard.tsx
│   │   │   │   │   ├── CreateTeamDialog.tsx
│   │   │   │   │   └── TeamMembersDialog.tsx
│   │   │   │   ├── hooks/
│   │   │   │   │   └── useTeams.ts
│   │   │   │   └── pages/
│   │   │   │       └── TeamsPage.tsx
│   │   │   │
│   │   │   ├── users/
│   │   │   │   ├── components/
│   │   │   │   │   └── UserCard.tsx
│   │   │   │   ├── hooks/
│   │   │   │   │   └── useUsers.ts
│   │   │   │   └── pages/
│   │   │   │       ├── UsersPage.tsx
│   │   │   │       └── ProfilePage.tsx
│   │   │   │
│   │   │   └── reports/
│   │   │       ├── components/
│   │   │       │   ├── KPICards.tsx
│   │   │       │   └── ReportFilters.tsx
│   │   │       └── pages/
│   │   │           └── ReportsPage.tsx
│   │   │
│   │   ├── hooks/                    # Custom hooks
│   │   │   ├── useAuth.ts
│   │   │   ├── useRole.ts
│   │   │   └── useDebounce.ts
│   │   │
│   │   ├── layouts/                  # Page layouts
│   │   │   ├── RootLayout.tsx
│   │   │   ├── AuthLayout.tsx
│   │   │   └── DashboardLayout.tsx
│   │   │
│   │   ├── lib/                      # Library configurations
│   │   │   ├── queryClient.ts        # React Query config
│   │   │   └── queryKeys.ts          # Query key factory
│   │   │
│   │   ├── store/                    # State management (Zustand)
│   │   │   └── authStore.ts
│   │   │
│   │   ├── types/                    # TypeScript types
│   │   │   ├── user.ts
│   │   │   ├── team.ts
│   │   │   ├── clock.ts
│   │   │   └── api.ts
│   │   │
│   │   ├── utils/                    # Utility functions
│   │   │   ├── date.ts               # Date formatting (date-fns)
│   │   │   ├── formatters.ts         # Data formatters
│   │   │   └── validators.ts         # Validation helpers
│   │   │
│   │   ├── App.tsx                   # Root component
│   │   ├── main.tsx                  # Application entry
│   │   └── router.tsx                # React Router config
│   │
│   ├── tests/                        # Test files
│   │   ├── unit/                     # Unit tests
│   │   │   └── components/
│   │   │       └── LoginForm.test.tsx
│   │   │
│   │   ├── integration/              # Integration tests
│   │   │   └── auth.test.tsx
│   │   │
│   │   └── e2e/                      # E2E tests (Playwright)
│   │       ├── login.spec.ts
│   │       └── clock.spec.ts
│   │
│   ├── .env.development              # Dev environment variables
│   ├── .env.production               # Prod environment variables
│   ├── .env.example                  # Environment template
│   ├── .eslintrc.json                # ESLint configuration
│   ├── .gitignore                    # Git ignore rules
│   ├── .prettierrc                   # Prettier configuration
│   ├── Dockerfile                    # Development Dockerfile
│   ├── Dockerfile.prod               # Production multi-stage Dockerfile
│   ├── index.html                    # HTML entry point
│   ├── nginx.conf                    # NGINX config for production
│   ├── package.json                  # NPM dependencies
│   ├── package-lock.json             # NPM lock file
│   ├── postcss.config.js             # PostCSS config
│   ├── tailwind.config.js            # TailwindCSS config
│   ├── tsconfig.json                 # TypeScript config
│   ├── tsconfig.node.json            # TypeScript Node config
│   ├── vite.config.ts                # Vite configuration
│   ├── vitest.config.ts              # Vitest test config
│   ├── Taskfile.yml                  # Frontend-specific tasks
│   └── README.md                     # Frontend documentation
│
├── gateway/                          # KrakenD API Gateway
│   ├── config/                       # KrakenD configuration
│   │   ├── krakend.json              # Main KrakenD config
│   │   ├── settings/                 # Modular settings
│   │   │   ├── endpoints.json        # Endpoint definitions
│   │   │   ├── backends.json         # Backend services
│   │   │   └── middleware.json       # Middleware config
│   │   │
│   │   └── templates/                # Config templates
│   │       └── krakend.tmpl
│   │
│   ├── plugins/                      # Custom KrakenD plugins (optional)
│   │   └── custom_auth.go
│   │
│   ├── Dockerfile                    # KrakenD Dockerfile
│   ├── Taskfile.yml                  # Gateway-specific tasks
│   └── README.md                     # Gateway documentation
│
├── scripts/                          # Project-wide scripts
│   ├── init-db.sh                    # Initialize database
│   ├── seed-data.sh                  # Seed test data
│   ├── backup-db.sh                  # Database backup
│   └── deploy.sh                     # Deployment script
│
├── docs/                             # Documentation
│   ├── ARCHITECTURE.md               # System architecture
│   ├── API.md                        # API documentation
│   ├── DATABASE.md                   # Database schema
│   ├── FRONTEND_TICKETS.md           # Frontend tickets
│   ├── PROJECT_STRUCTURE.md          # This file
│   ├── DEPLOYMENT.md                 # Deployment guide
│   └── CONTRIBUTING.md               # Contribution guidelines
│
├── .github/                          # GitHub configuration
│   ├── workflows/                    # CI/CD pipelines
│   │   ├── backend-ci.yml
│   │   ├── frontend-ci.yml
│   │   └── integration-ci.yml
│   │
│   └── PULL_REQUEST_TEMPLATE.md      # PR template
│
├── .gitignore                        # Root gitignore
├── .env.example                      # Global environment template
├── compose.yml                       # Development Docker Compose
├── compose.prod.yml                  # Production Docker Compose
├── Taskfile.yml                      # Root Taskfile (orchestrates all)
└── README.md                         # Project documentation

```

---

## Detailed Component Breakdown

### 1. Backend (Go) Structure

**Architecture Pattern**: Clean Architecture / Hexagonal

```
Layers:
1. Domain Layer (internal/domain/)
   - Pure business logic
   - No external dependencies
   - Entities and interfaces

2. Application Layer (internal/domain/services/)
   - Use cases
   - Orchestrates domain logic
   - Transaction boundaries

3. Infrastructure Layer (internal/infrastructure/)
   - External dependencies (DB, auth)
   - Repository implementations
   - Framework-specific code

4. API Layer (internal/api/)
   - HTTP handlers
   - Request/Response DTOs
   - Middleware
```

**Key Go Files:**

**cmd/api/main.go**:
```go
package main

import (
    "log"
    "github.com/yourusername/timemanager/internal/config"
    "github.com/yourusername/timemanager/internal/infrastructure/database"
    "github.com/yourusername/timemanager/internal/api/routes"
)

func main() {
    cfg := config.Load()
    db := database.NewPostgresDB(cfg.DatabaseURL)
    defer db.Close()

    router := routes.SetupRouter(db)
    log.Fatal(router.Run(cfg.ServerPort))
}
```

**Backend Dependencies (go.mod)**:
```
module github.com/yourusername/timemanager

go 1.21

require (
    github.com/gin-gonic/gin v1.9.1           // Web framework
    github.com/lib/pq v1.10.9                 // PostgreSQL driver
    github.com/golang-jwt/jwt/v5 v5.0.0       // JWT
    golang.org/x/crypto v0.14.0               // Password hashing
    github.com/go-playground/validator/v10    // Validation
    github.com/joho/godotenv v1.5.1           // Environment variables
    github.com/golang-migrate/migrate/v4      // Database migrations
    github.com/stretchr/testify v1.8.4        // Testing
)
```

---

### 2. Frontend (React) Structure

**Architecture Pattern**: Feature-based with Domain-Driven Design

```
Features Organization:
- Each feature is self-contained
- Components, hooks, pages co-located
- Shared components in /components
- State management per feature or global
```

**Key Frontend Files:**

**vite.config.ts**:
```typescript
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  server: {
    port: 5173,
    proxy: {
      '/api': {
        target: 'http://localhost:8080', // KrakenD Gateway
        changeOrigin: true,
      },
    },
  },
});
```

**Frontend Dependencies (package.json)**:
```json
{
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "react-router-dom": "^6.20.0",
    "@tanstack/react-query": "^5.15.0",
    "zustand": "^4.4.7",
    "axios": "^1.6.2",
    "react-hook-form": "^7.49.0",
    "@hookform/resolvers": "^3.3.2",
    "zod": "^3.22.4",
    "date-fns": "^3.0.0",
    "recharts": "^2.10.0",
    "lucide-react": "^0.292.0",
    "clsx": "^2.0.0",
    "tailwindcss": "^3.4.0"
  },
  "devDependencies": {
    "@types/react": "^18.2.43",
    "@vitejs/plugin-react": "^4.2.1",
    "vite": "^5.0.8",
    "typescript": "^5.3.3",
    "vitest": "^1.0.4",
    "@testing-library/react": "^14.1.2",
    "eslint": "^8.55.0",
    "prettier": "^3.1.1"
  }
}
```

---

### 3. KrakenD Gateway Structure

**krakend.json** (Main Config):
```json
{
  "$schema": "https://www.krakend.io/schema/v2.5/krakend.json",
  "version": 3,
  "name": "Time Manager API Gateway",
  "port": 8080,
  "timeout": "10s",
  "cache_ttl": "5m",
  "endpoints": [
    {
      "endpoint": "/auth/login",
      "method": "POST",
      "backend": [
        {
          "url_pattern": "/api/auth/login",
          "host": ["http://backend:4000"],
          "method": "POST"
        }
      ],
      "extra_config": {
        "qos/ratelimit/router": {
          "max_rate": 5,
          "client_max_rate": 5,
          "every": "1m"
        }
      }
    },
    {
      "endpoint": "/users",
      "method": "GET",
      "backend": [
        {
          "url_pattern": "/api/users",
          "host": ["http://backend:4000"]
        }
      ],
      "extra_config": {
        "auth/validator": {
          "alg": "HS256",
          "jwk_url": "http://backend:4000/.well-known/jwks.json",
          "disable_jwk_security": true
        }
      }
    }
  ]
}
```

---

### 4. Docker Compose Setup

**compose.yml** (Development):
```yaml
version: '3.8'

services:
  database:
    image: postgres:16-alpine
    container_name: timemanager_db
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
      POSTGRES_DB: timemanager_dev
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 10s
      timeout: 5s
      retries: 5
    networks:
      - timemanager

  backend:
    build:
      context: ./backend
      dockerfile: Dockerfile
    container_name: timemanager_backend
    environment:
      DATABASE_URL: postgresql://postgres:postgres@database:5432/timemanager_dev?sslmode=disable
      JWT_SECRET: dev_secret_key_change_in_production
      PORT: 4000
    ports:
      - "4000:4000"
    volumes:
      - ./backend:/app
    depends_on:
      database:
        condition: service_healthy
    networks:
      - timemanager
    command: task dev

  frontend:
    build:
      context: ./frontend
      dockerfile: Dockerfile
    container_name: timemanager_frontend
    environment:
      VITE_KRAKEND_URL: http://localhost:8080
    ports:
      - "5173:5173"
    volumes:
      - ./frontend:/app
      - /app/node_modules
    networks:
      - timemanager
    command: npm run dev

  gateway:
    image: devopsfaith/krakend:2.5
    container_name: timemanager_gateway
    ports:
      - "8080:8080"
    volumes:
      - ./gateway/config/krakend.json:/etc/krakend/krakend.json
    depends_on:
      - backend
    networks:
      - timemanager
    command: run -c /etc/krakend/krakend.json

volumes:
  postgres_data:

networks:
  timemanager:
    driver: bridge
```

**compose.prod.yml** (Production):
```yaml
version: '3.8'

services:
  database:
    image: postgres:16-alpine
    restart: always
    environment:
      POSTGRES_USER: ${DB_USER}
      POSTGRES_PASSWORD: ${DB_PASSWORD}
      POSTGRES_DB: ${DB_NAME}
    volumes:
      - postgres_data:/var/lib/postgresql/data
    networks:
      - timemanager

  backend:
    build:
      context: ./backend
      dockerfile: Dockerfile.prod
    restart: always
    environment:
      DATABASE_URL: ${DATABASE_URL}
      JWT_SECRET: ${JWT_SECRET}
      PORT: 4000
    depends_on:
      - database
    networks:
      - timemanager

  frontend:
    build:
      context: ./frontend
      dockerfile: Dockerfile.prod
      args:
        VITE_KRAKEND_URL: ${KRAKEND_URL}
    restart: always
    networks:
      - timemanager

  gateway:
    image: devopsfaith/krakend:2.5
    restart: always
    ports:
      - "80:8080"
      - "443:8443"
    volumes:
      - ./gateway/config/krakend.json:/etc/krakend/krakend.json
      - ./gateway/ssl:/etc/krakend/ssl
    depends_on:
      - backend
    networks:
      - timemanager

  reverse-proxy:
    image: nginx:alpine
    restart: always
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx/nginx.conf:/etc/nginx/nginx.conf
      - ./nginx/ssl:/etc/nginx/ssl
      - frontend_static:/usr/share/nginx/html
    depends_on:
      - gateway
    networks:
      - timemanager

volumes:
  postgres_data:
  frontend_static:

networks:
  timemanager:
    driver: bridge
```

---

### 5. Taskfile Configuration

**Root Taskfile.yml** (Orchestrates all):
```yaml
version: '3'

tasks:
  # Development
  dev:
    desc: Start all services in development mode
    cmds:
      - docker-compose up

  dev:backend:
    desc: Start backend only
    dir: backend
    cmds:
      - task dev

  dev:frontend:
    desc: Start frontend only
    dir: frontend
    cmds:
      - task dev

  dev:gateway:
    desc: Start gateway only
    cmds:
      - docker-compose up gateway

  # Building
  build:
    desc: Build all services
    cmds:
      - task: build:backend
      - task: build:frontend

  build:backend:
    desc: Build backend
    dir: backend
    cmds:
      - task build

  build:frontend:
    desc: Build frontend
    dir: frontend
    cmds:
      - task build

  # Testing
  test:
    desc: Run all tests
    cmds:
      - task: test:backend
      - task: test:frontend

  test:backend:
    desc: Run backend tests
    dir: backend
    cmds:
      - task test

  test:frontend:
    desc: Run frontend tests
    dir: frontend
    cmds:
      - task test

  # Database
  db:migrate:
    desc: Run database migrations
    dir: backend
    cmds:
      - task migrate

  db:seed:
    desc: Seed database with test data
    cmds:
      - ./scripts/seed-data.sh

  db:reset:
    desc: Reset database (drop + migrate + seed)
    cmds:
      - task: db:drop
      - task: db:migrate
      - task: db:seed

  # Cleanup
  clean:
    desc: Clean all build artifacts
    cmds:
      - docker-compose down -v
      - rm -rf backend/bin
      - rm -rf frontend/dist
      - rm -rf frontend/node_modules

  # Production
  prod:up:
    desc: Start production environment
    cmds:
      - docker-compose -f compose.prod.yml up -d

  prod:down:
    desc: Stop production environment
    cmds:
      - docker-compose -f compose.prod.yml down

  # Utilities
  logs:
    desc: View logs from all services
    cmds:
      - docker-compose logs -f

  logs:backend:
    desc: View backend logs
    cmds:
      - docker-compose logs -f backend

  logs:frontend:
    desc: View frontend logs
    cmds:
      - docker-compose logs -f frontend

  logs:gateway:
    desc: View gateway logs
    cmds:
      - docker-compose logs -f gateway
```

**Backend Taskfile.yml**:
```yaml
version: '3'

tasks:
  dev:
    desc: Run backend in development mode
    cmds:
      - air # Live reload tool for Go

  build:
    desc: Build backend binary
    cmds:
      - go build -o bin/api cmd/api/main.go

  test:
    desc: Run all tests
    cmds:
      - go test ./... -v -cover

  test:integration:
    desc: Run integration tests
    cmds:
      - go test ./tests/integration/... -v

  test:coverage:
    desc: Generate test coverage report
    cmds:
      - go test ./... -coverprofile=coverage.out
      - go tool cover -html=coverage.out -o coverage.html

  migrate:
    desc: Run database migrations
    cmds:
      - migrate -path internal/infrastructure/database/migrations -database "${DATABASE_URL}" up

  migrate:down:
    desc: Rollback last migration
    cmds:
      - migrate -path internal/infrastructure/database/migrations -database "${DATABASE_URL}" down 1

  lint:
    desc: Run linter
    cmds:
      - golangci-lint run

  fmt:
    desc: Format code
    cmds:
      - go fmt ./...
      - goimports -w .

  deps:
    desc: Download dependencies
    cmds:
      - go mod download
      - go mod tidy
```

**Frontend Taskfile.yml**:
```yaml
version: '3'

tasks:
  dev:
    desc: Run frontend in development mode
    cmds:
      - npm run dev

  build:
    desc: Build frontend for production
    cmds:
      - npm run build

  preview:
    desc: Preview production build locally
    cmds:
      - npm run preview

  test:
    desc: Run all tests
    cmds:
      - npm test

  test:watch:
    desc: Run tests in watch mode
    cmds:
      - npm test -- --watch

  test:coverage:
    desc: Generate test coverage report
    cmds:
      - npm test -- --coverage

  test:e2e:
    desc: Run E2E tests
    cmds:
      - npm run test:e2e

  lint:
    desc: Run linter
    cmds:
      - npm run lint

  lint:fix:
    desc: Fix linting issues
    cmds:
      - npm run lint -- --fix

  format:
    desc: Format code with Prettier
    cmds:
      - npm run format

  type-check:
    desc: Run TypeScript type checking
    cmds:
      - npm run type-check

  deps:
    desc: Install dependencies
    cmds:
      - npm ci
```

---

## Environment Variables

**.env.example** (Root):
```bash
# Database
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/timemanager_dev?sslmode=disable
DB_USER=postgres
DB_PASSWORD=postgres
DB_NAME=timemanager_dev

# Backend
BACKEND_PORT=4000
JWT_SECRET=your-secret-key-change-in-production
JWT_EXPIRATION=900  # 15 minutes in seconds

# Frontend
VITE_KRAKEND_URL=http://localhost:8080

# KrakenD Gateway
KRAKEND_PORT=8080

# Environment
NODE_ENV=development
GO_ENV=development
```

---

## Data Flow Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        CLIENT (Browser)                      │
│                   React SPA (Port 5173)                      │
└────────────────────┬────────────────────────────────────────┘
                     │ HTTP Requests
                     │ (Authorization: Bearer {JWT})
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                   KrakenD API Gateway                        │
│                      (Port 8080)                             │
│                                                              │
│  Features:                                                   │
│  - Rate limiting (5 req/min for /auth/*)                    │
│  - JWT validation                                            │
│  - Request aggregation                                       │
│  - Response caching (5 min TTL)                             │
│  - CORS handling                                             │
└────────────────────┬────────────────────────────────────────┘
                     │ Forwarded Requests
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                   Go Backend API                             │
│                   (Port 4000)                                │
│                                                              │
│  Layers:                                                     │
│  1. HTTP Handlers (Gin)                                     │
│  2. Middleware (Auth, CORS, Logging)                        │
│  3. Services (Business Logic)                               │
│  4. Repositories (Data Access)                              │
└────────────────────┬────────────────────────────────────────┘
                     │ SQL Queries
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                   PostgreSQL Database                        │
│                      (Port 5432)                             │
│                                                              │
│  Tables:                                                     │
│  - users, teams, team_members                               │
│  - clocks, refresh_tokens                                   │
└─────────────────────────────────────────────────────────────┘
```

---

## Request Flow Example: Clock In

```
1. User clicks "Clock In" button
   ↓
2. React Frontend sends POST /clocks
   Headers: { Authorization: "Bearer {jwt}" }
   ↓
3. KrakenD Gateway receives request
   - Validates JWT signature
   - Checks rate limit
   - Forwards to backend
   ↓
4. Go Backend receives POST /api/clocks
   - Auth middleware extracts user from JWT
   - ClockHandler calls ClockService
   - ClockService calls ClockRepository
   ↓
5. PostgreSQL inserts clock entry
   INSERT INTO clocks (user_id, time, status) VALUES (...)
   ↓
6. Response flows back:
   Backend → Gateway → Frontend
   ↓
7. React updates UI (React Query invalidates cache)
```

---

## Development Workflow

### 1. Initial Setup
```bash
# Clone repository
git clone <repo-url>
cd Time-Manager

# Copy environment variables
cp .env.example .env

# Start all services
task dev
```

### 2. Backend Development
```bash
# Terminal 1 - Backend only
task dev:backend

# Terminal 2 - Watch tests
cd backend
task test:watch

# Run migrations
task db:migrate

# Seed database
task db:seed
```

### 3. Frontend Development
```bash
# Terminal 1 - Frontend only
task dev:frontend

# Terminal 2 - Watch tests
cd frontend
task test:watch

# Type checking
task type-check
```

### 4. Full Stack Development
```bash
# Start everything
task dev

# View logs
task logs

# Run all tests
task test
```

---

## Production Deployment

```bash
# Build production images
docker-compose -f compose.prod.yml build

# Start production
task prod:up

# View logs
docker-compose -f compose.prod.yml logs -f

# Stop production
task prod:down
```

---

## File Naming Conventions

**Backend (Go)**:
- Files: `snake_case.go` (e.g., `user_repository.go`)
- Packages: lowercase, single word (e.g., `package handlers`)
- Structs: PascalCase (e.g., `type UserService struct`)
- Interfaces: PascalCase with -er suffix (e.g., `type UserRepository interface`)

**Frontend (React)**:
- Components: PascalCase (e.g., `LoginForm.tsx`)
- Hooks: camelCase with use prefix (e.g., `useAuth.ts`)
- Utilities: camelCase (e.g., `formatDate.ts`)
- Types: PascalCase (e.g., `User.ts`)

---

**Document Status**: Complete project structure - Ready for implementation
**Last Updated**: 2025-10-06
**Owner**: Development Team
