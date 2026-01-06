# Time Manager

SaaS Workforce Management Platform - A complete time tracking and workforce management solution built with modern technologies.

## 📋 Project Overview

Time Manager is a comprehensive SaaS application designed for efficient workforce time tracking, management, and analytics. Built with a focus on performance, security, and scalability.

## 🚀 Tech Stack

### Backend
- **Rust** + **Axum** 0.7 - High-performance async web framework
- **Diesel** 2.1 - Type-safe ORM with PostgreSQL
- **PostgreSQL** 16 - Robust relational database
- **JWT** - Secure authentication

### Frontend
- **React** 18 + **TypeScript** 5 - Modern type-safe UI
- **Vite** 5 - Lightning-fast build tool
- **Tailwind CSS** - Utility-first styling
- **Shadcn/UI** - Accessible component library

### Infrastructure
- **Docker Compose** - Service orchestration
- **Traefik** v2.11 - Reverse proxy with automatic HTTPS
- **Prometheus** + **Loki** + **Grafana** - Observability stack
- **Mailpit** - Email testing

### CI/CD
- **GitHub Actions** - Automated testing and quality gates
- **Cargo** (clippy, fmt) - Rust linting and formatting
- **ESLint** + **Prettier** - JavaScript/TypeScript quality

## ✨ Features

- ✅ Multi-tenant organization support
- ✅ Role-based access control (Admin, Manager, Employee)
- ✅ RESTful API with comprehensive error handling
- ✅ Real-time metrics and monitoring
- ✅ Security headers and rate limiting
- ✅ CORS configuration
- ✅ Database migrations
- ✅ Automated CI/CD pipeline

## 🛠️ Prerequisites

- **Docker** 20.10+ and **Docker Compose** v2.0+
- **Rust** 1.92+ (for local development)
- **Node.js** 20+ (for frontend development)
- **PostgreSQL** 16+ (provided via Docker)

## 🚀 Quick Start

### 1. Clone the repository
```bash
git clone <repository-url>
cd time-manager
```

### 2. Start all services
```bash
docker compose up -d
```

### 3. Verify services
```bash
docker compose ps
```

### 4. Access the application
- **Frontend**: http://localhost:8000
- **Backend API**: http://localhost:8000/api/health
- **Grafana**: http://localhost:3001 (admin/admin)
- **Prometheus**: http://localhost:9090
- **Mailpit**: http://localhost:8025
- **Traefik Dashboard**: http://localhost:8081

## 📁 Project Structure

```
.
├── backend/                 # Rust/Axum API
│   ├── src/
│   │   ├── main.rs         # Application entry point
│   │   ├── api/            # API routes and handlers
│   │   ├── config/         # Configuration management
│   │   ├── domain/         # Domain models and enums
│   │   ├── error/          # Error handling
│   │   └── models/         # Database models
│   ├── migrations/         # Diesel migrations
│   └── Dockerfile          # Multi-stage Rust build
│
├── frontend/               # React/TypeScript app
│   ├── src/
│   │   ├── components/     # React components
│   │   ├── lib/            # Utilities
│   │   └── main.tsx        # Application entry
│   ├── nginx.conf          # Nginx configuration
│   └── Dockerfile          # Multi-stage Node build
│
├── infrastructure/         # Configuration files
│   ├── traefik/           # Reverse proxy config
│   ├── prometheus/        # Metrics collection
│   ├── loki/              # Log aggregation
│   └── grafana/           # Dashboards and datasources
│
├── .github/workflows/     # CI/CD pipelines
├── docker-compose.yml     # Service orchestration
└── docs/                  # Additional documentation
```

## 🔐 Environment Variables

### Backend (.env)
```bash
APP_HOST=0.0.0.0
APP_PORT=8080
DATABASE_URL=postgres://timemanager:timemanager_dev_password@postgres:5432/timemanager
RUST_LOG=debug
JWT_SECRET=dev-secret-key-not-for-production
CORS_ALLOWED_ORIGINS=http://localhost:3000,http://localhost:8000
```

### Frontend (.env)
```bash
VITE_API_URL=http://localhost:8000/api/v1
```

## 🗄️ Database

### Run migrations
```bash
cd backend
diesel migration run
```

### Rollback migration
```bash
diesel migration revert
```

### Load seed data
```bash
docker compose exec postgres psql -U timemanager -d timemanager -f /path/to/seed.sql
```

**Test Credentials**:
- Admin: admin@acme.com / Password123!
- Manager: manager@acme.com / Password123!
- Employee: employee@acme.com / Password123!

## 💻 Development

### Backend Development
```bash
cd backend

# Run with hot reload
cargo watch -x run

# Format code
cargo fmt

# Lint
cargo clippy

# Build release
cargo build --release
```

### Frontend Development
```bash
cd frontend

# Install dependencies
npm install

# Development server
npm run dev

# Lint
npm run lint

# Type check
npm run type-check

# Build
npm run build
```

## 🧪 Testing

### Run all tests
```bash
# Backend
cd backend && cargo test

# Frontend
cd frontend && npm test
```

## 🔒 Security

- ✅ Rate limiting (100 req/min global, 5 req/min auth endpoints)
- ✅ Security headers (CSP, X-Frame-Options, HSTS)
- ✅ CORS configuration
- ✅ JWT-based authentication
- ✅ Input validation
- ✅ SQL injection protection (Diesel ORM)

## 📊 Monitoring

Access Grafana at http://localhost:3001:
- **Username**: admin
- **Password**: admin

**Available Metrics**:
- HTTP request rate and latency
- Error rates (4xx, 5xx)
- Database connection pool status
- Application logs (via Loki)

## 🔄 CI/CD

GitHub Actions workflows run on every push:
- **Backend**: Format check, linting (clippy), build
- **Frontend**: ESLint, TypeScript check, build
- **Security**: Dependency audits (cargo-audit, npm-audit)

## 🐛 Troubleshooting

### Port conflicts
If port 8000 is already in use:
```bash
# Edit docker-compose.yml
# Change "8000:80" to "8001:80" for traefik service
```

### Database connection issues
```bash
# Check PostgreSQL is healthy
docker compose ps postgres

# View logs
docker compose logs postgres
```

### Frontend not loading
```bash
# Rebuild frontend
docker compose build frontend
docker compose up -d frontend
```

## 📚 Additional Documentation

- [Setup Guide](docs/SETUP.md) - Detailed installation instructions
- [Development Guide](docs/DEVELOPMENT.md) - Development workflows and patterns

## 📄 License

This project is part of an academic assignment at EPITECH.

## 🤝 Contributing

1. Create a feature branch
2. Make your changes
3. Run tests and linting
4. Submit a pull request

---

**Built with ❤️ for EPITECH MSC1**
