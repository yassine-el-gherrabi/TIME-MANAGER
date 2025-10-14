# Time Manager - Quick Start Guide

## 📋 Project Overview

Trinity Time Manager is a comprehensive time tracking application built with a modern microservices architecture.

**Technology Stack:**
- **Backend**: Go (Gin framework) with Clean Architecture
- **Frontend**: React 18 + TypeScript + Vite
- **API Gateway**: KrakenD v2.5
- **Database**: PostgreSQL 16
- **DevOps**: Docker + Taskfile + GitHub Actions

## 🚀 Quick Start (Development)

### Prerequisites

- Docker & Docker Compose
- Go 1.21+
- Node.js 20+
- Task CLI (`brew install go-task/tap/go-task`)

### 1. Clone and Setup

```bash
# Clone repository
git clone git@github.com:EpitechMscProPromo2027/T-DEV-700-project-MAR_13.git
cd T-DEV-700-project-MAR_13

# Copy environment files
cp .env.example .env
cp backend/.env.example backend/.env
cp frontend/.env.example frontend/.env
```

### 2. Start All Services

```bash
# Start everything (database, backend, frontend, gateway)
task dev

# Or with Docker Compose directly
docker-compose up
```

**Services will be available at:**
- Frontend: http://localhost:3000
- API Gateway: http://localhost:8080
- Backend API: http://localhost:4000 (internal)
- Database: localhost:5432

### 3. Initialize Database

```bash
# Run migrations
task db:migrate

# Seed initial data (optional)
task db:seed
```

### 4. Verify Installation

```bash
# Run all tests
task test

# Or individually
task test:backend
task test:frontend
```

## 📂 Project Structure

```
time-manager/
├── backend/          # Go API service
├── frontend/         # React application
├── gateway/          # KrakenD configuration
├── scripts/          # Utility scripts
├── docs/            # Documentation
├── .github/         # CI/CD workflows
├── compose.yml      # Docker Compose (dev)
├── compose.prod.yml # Docker Compose (prod)
└── Taskfile.yml     # Task automation
```

## 🛠️ Common Development Tasks

### Backend Development

```bash
# Navigate to backend
cd backend

# Start with hot reload
task dev

# Run tests with coverage
task test

# Run linter
task lint

# Build binary
task build
```

### Frontend Development

```bash
# Navigate to frontend
cd frontend

# Start dev server
task dev

# Run tests
task test

# Build for production
task build

# Type checking
task type-check
```

### Database Operations

```bash
# Create new migration
task db:create-migration name=add_new_table

# Run migrations
task db:migrate

# Rollback last migration
task db:rollback

# Reset database (DANGER)
task db:reset
```

### Gateway Configuration

```bash
# Validate KrakenD config
task gateway:validate

# Reload gateway (no downtime)
task gateway:reload

# View gateway logs
task gateway:logs
```

## 🏗️ Architecture Overview

### Request Flow

```
Client (Browser)
    ↓
KrakenD Gateway (Port 8080)
    ↓ [Rate Limiting, JWT Validation, Caching]
Backend API (Port 4000)
    ↓ [Business Logic, Authorization]
PostgreSQL Database (Port 5432)
```

### Key Components

**Backend (Go):**
- Clean Architecture with 4 layers (Domain, Application, Infrastructure, API)
- JWT authentication with refresh tokens
- RBAC (Employee/Manager roles)
- Comprehensive test coverage

**Frontend (React):**
- Feature-based architecture
- Zustand for state management
- TanStack Query for server state
- shadcn/ui components

**Gateway (KrakenD):**
- Rate limiting (5 req/min for login, 100 req/min for API)
- JWT validation
- Response caching
- Request aggregation

## 📚 Documentation Links

- [Complete Architecture](./ARCHITECTURE.md) - System design and technology decisions
- [Database Schema](./DATABASE.md) - ERD, tables, indexes, queries
- [API Specification](./API.md) - All endpoints with examples
- [Frontend Tickets](./FRONTEND_TICKETS.md) - Development tasks and sprints
- [Project Structure](./PROJECT_STRUCTURE.md) - Complete directory tree

## 🔑 Default Credentials (Development)

```
Admin User:
  Email: admin@timemanager.com
  Password: Admin123!

Manager User:
  Email: manager@timemanager.com
  Password: Manager123!

Employee User:
  Email: employee@timemanager.com
  Password: Employee123!
```

## 🧪 Testing

### Backend Tests

```bash
# All tests with coverage
cd backend && task test

# Specific package
go test ./internal/domain/services/... -v

# Integration tests only
go test ./tests/integration/... -v

# Generate coverage report
task test:coverage
```

### Frontend Tests

```bash
# All tests
cd frontend && task test

# Watch mode
npm test -- --watch

# Coverage report
npm test -- --coverage

# E2E tests (requires running app)
npm run test:e2e
```

## 🚢 Production Deployment

### Build Production Images

```bash
# Build all services
task build:prod

# Or individually
docker build -f backend/Dockerfile -t time-manager-backend .
docker build -f frontend/Dockerfile -t time-manager-frontend .
```

### Deploy with Docker Compose

```bash
# Start production stack
docker-compose -f compose.prod.yml up -d

# View logs
docker-compose -f compose.prod.yml logs -f

# Stop services
docker-compose -f compose.prod.yml down
```

### Health Checks

```bash
# Backend health
curl http://localhost:8080/health

# Gateway health
curl http://localhost:8080/__health

# Database connection
task db:ping
```

## 🔒 Environment Variables

### Backend (.env)

```env
# Server
PORT=4000
ENVIRONMENT=development

# Database
DATABASE_URL=postgres://timemanager:password@db:5432/timemanager?sslmode=disable

# JWT
JWT_SECRET=your-super-secret-key-change-in-production
JWT_EXPIRATION=15m
REFRESH_TOKEN_EXPIRATION=7d

# CORS
CORS_ALLOWED_ORIGINS=http://localhost:3000

# Rate Limiting
RATE_LIMIT_REQUESTS=100
RATE_LIMIT_WINDOW=1m
```

### Frontend (.env)

```env
# API Gateway
VITE_API_BASE_URL=http://localhost:8080

# Environment
VITE_ENVIRONMENT=development

# Feature Flags
VITE_ENABLE_ANALYTICS=false
VITE_ENABLE_DEBUG=true
```

## 📊 Monitoring & Logs

### View Logs

```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f backend
docker-compose logs -f frontend
docker-compose logs -f gateway

# Backend logs (if running locally)
cd backend && tail -f logs/app.log
```

### Metrics

```bash
# Backend metrics (Prometheus format)
curl http://localhost:4000/metrics

# Gateway metrics
curl http://localhost:8080/__stats
```

## 🐛 Troubleshooting

### Database Connection Issues

```bash
# Check database status
docker-compose ps db

# Connect to database
docker-compose exec db psql -U timemanager

# Reset database
task db:reset
```

### Gateway Issues

```bash
# Validate configuration
task gateway:validate

# Check gateway logs
docker-compose logs gateway

# Restart gateway
docker-compose restart gateway
```

### Frontend Build Issues

```bash
# Clear node_modules and reinstall
cd frontend
rm -rf node_modules package-lock.json
npm install

# Clear Vite cache
rm -rf .vite
```

### Backend Build Issues

```bash
# Clear Go build cache
cd backend
go clean -cache -modcache -testcache

# Re-download dependencies
go mod download
go mod tidy
```

## 📝 Git Workflow

### Feature Development

```bash
# Create feature branch
git checkout -b feature/FE-001-project-setup

# Commit changes
git add .
git commit -m "feat(frontend): FE-001 setup project structure"

# Push and create PR
git push -u origin feature/FE-001-project-setup
```

### Commit Message Format

```
<type>(<scope>): <subject>

Types: feat, fix, docs, style, refactor, test, chore
Scopes: frontend, backend, gateway, database, ci, docs

Examples:
feat(backend): add user authentication endpoint
fix(frontend): resolve clock-in button state issue
docs(api): update authentication flow documentation
```

## 🎯 Development Workflow

### Daily Workflow

1. Pull latest changes: `git pull origin main`
2. Start services: `task dev`
3. Create feature branch: `git checkout -b feature/your-ticket`
4. Make changes
5. Run tests: `task test`
6. Commit and push
7. Create pull request

### Before Committing

```bash
# Run full test suite
task test

# Run linters
task lint

# Check types (frontend)
cd frontend && task type-check

# Format code
task format
```

## 🔧 IDE Setup

### VS Code Recommended Extensions

```json
{
  "recommendations": [
    "golang.go",
    "bradlc.vscode-tailwindcss",
    "dbaeumer.vscode-eslint",
    "esbenp.prettier-vscode",
    "ms-azuretools.vscode-docker",
    "task.vscode-task"
  ]
}
```

### VS Code Settings

```json
{
  "go.testFlags": ["-v", "-race"],
  "go.lintTool": "golangci-lint",
  "editor.formatOnSave": true,
  "editor.defaultFormatter": "esbenp.prettier-vscode",
  "[go]": {
    "editor.defaultFormatter": "golang.go"
  }
}
```

## 📖 Learning Resources

### Go Resources
- [Effective Go](https://golang.org/doc/effective_go)
- [Go by Example](https://gobyexample.com/)
- [Clean Architecture in Go](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)

### React Resources
- [React Documentation](https://react.dev/)
- [TanStack Query](https://tanstack.com/query/latest)
- [Zustand](https://github.com/pmndrs/zustand)

### KrakenD Resources
- [KrakenD Documentation](https://www.krakend.io/docs/)
- [KrakenD Playground](https://designer.krakend.io/)

## 🤝 Contributing

1. Review [ARCHITECTURE.md](./ARCHITECTURE.md) for design decisions
2. Check [FRONTEND_TICKETS.md](./FRONTEND_TICKETS.md) for available tasks
3. Follow [API.md](./API.md) for API contracts
4. Maintain test coverage >80%
5. Update documentation for new features

## 📞 Support

- **Technical Issues**: Create GitHub issue
- **Documentation**: See `docs/` directory
- **API Questions**: Check `docs/API.md`
- **Architecture Questions**: Check `docs/ARCHITECTURE.md`

---

**Quick Links:**
- [GitHub Repository](https://github.com/EpitechMscProPromo2027/T-DEV-700-project-MAR_13)
- [Project PDF](/Users/yassinelechef/WorkSpace/EPITECH/MSC1/Time Manager/project.pdf)
- [CI/CD Pipeline](./.github/workflows/)

**Last Updated**: 2025-10-06
