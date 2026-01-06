# Setup Guide

Complete setup instructions for the Time Manager project.

## Prerequisites Installation

### Docker & Docker Compose

**macOS**:
```bash
# Install Docker Desktop
brew install --cask docker
```

**Linux (Ubuntu/Debian)**:
```bash
# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Install Docker Compose
sudo apt-get update
sudo apt-get install docker-compose-plugin
```

**Windows**:
Download and install Docker Desktop from https://www.docker.com/products/docker-desktop

### Rust (Optional - for local development)

```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

### Node.js (Optional - for local development)

```bash
# macOS
brew install node@20

# Linux (Ubuntu/Debian)
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs

# Verify installation
node --version
npm --version
```

## Project Setup

### 1. Clone Repository
```bash
git clone <repository-url>
cd time-manager
```

### 2. Environment Configuration

#### Backend Environment
```bash
cd backend
cp .env.example .env
# Edit .env with your configuration
```

**Backend .env variables**:
- `APP_HOST`: Server bind address (default: 0.0.0.0)
- `APP_PORT`: Server port (default: 8080)
- `DATABASE_URL`: PostgreSQL connection string
- `RUST_LOG`: Log level (debug, info, warn, error)
- `JWT_SECRET`: Secret for JWT token signing (CHANGE IN PRODUCTION!)
- `CORS_ALLOWED_ORIGINS`: Allowed CORS origins

#### Frontend Environment
```bash
cd ../frontend
cp .env.example .env
# Edit .env with your configuration
```

**Frontend .env variables**:
- `VITE_API_URL`: Backend API URL (default: http://localhost:8000/api/v1)

### 3. Start Services

#### Using Docker Compose (Recommended)
```bash
# Build and start all services
docker compose up -d

# View logs
docker compose logs -f

# Stop services
docker compose down

# Remove volumes (WARNING: deletes data)
docker compose down -v
```

#### Local Development

**Backend**:
```bash
cd backend

# Install Diesel CLI
cargo install diesel_cli --no-default-features --features postgres

# Run migrations
diesel migration run

# Start server
cargo run

# Or with hot reload
cargo watch -x run
```

**Frontend**:
```bash
cd frontend

# Install dependencies
npm install

# Start development server
npm run dev
```

## Database Setup

### 1. Access PostgreSQL
```bash
# Via Docker
docker compose exec postgres psql -U timemanager -d timemanager

# Locally (if PostgreSQL is running)
psql -h localhost -U timemanager -d timemanager
```

### 2. Run Migrations
```bash
cd backend

# Run all pending migrations
diesel migration run

# Rollback last migration
diesel migration revert

# Redo last migration
diesel migration redo
```

### 3. Load Seed Data
```bash
# Execute seed script
docker compose exec postgres psql -U timemanager -d timemanager < backend/scripts/seed.sql

# Or manually
docker compose exec -i postgres psql -U timemanager -d timemanager <<EOF
INSERT INTO organizations (id, name, slug, timezone)
VALUES ('00000000-0000-0000-0000-000000000001', 'ACME Corp', 'acme', 'Europe/Paris');
EOF
```

## Verification

### 1. Check Service Health
```bash
# All services status
docker compose ps

# Expected output: All services "Up" or "Up (healthy)"
```

### 2. Test Endpoints
```bash
# Backend health
curl http://localhost:8000/api/health
# Expected: {"status":"ok","version":"0.1.0","timestamp":...}

# Frontend
curl -I http://localhost:8000
# Expected: HTTP/1.1 200 OK

# Grafana
curl -I http://localhost:3001
# Expected: HTTP/1.1 200 OK

# Prometheus targets
curl http://localhost:9090/api/v1/targets
# Expected: JSON with target status
```

### 3. Access Web Interfaces

| Service | URL | Credentials |
|---------|-----|-------------|
| Frontend | http://localhost:8000 | - |
| Grafana | http://localhost:3001 | admin/admin |
| Prometheus | http://localhost:9090 | - |
| Traefik Dashboard | http://localhost:8081 | - |
| Mailpit | http://localhost:8025 | - |

## Troubleshooting

### Port Already in Use
```bash
# Find process using port
lsof -i :8000

# Change port in docker-compose.yml
# For traefik service, change "8000:80" to "8001:80"
```

### Permission Denied Errors
```bash
# Linux: Add user to docker group
sudo usermod -aG docker $USER
# Logout and login again
```

### Database Connection Failed
```bash
# Check PostgreSQL logs
docker compose logs postgres

# Restart PostgreSQL
docker compose restart postgres

# Verify connection
docker compose exec postgres pg_isready -U timemanager
```

### Frontend Build Errors
```bash
# Clear node_modules
cd frontend
rm -rf node_modules package-lock.json
npm install

# Rebuild container
docker compose build frontend
docker compose up -d frontend
```

### Rust Compilation Errors
```bash
# Update Rust toolchain
rustup update stable

# Clean build cache
cd backend
cargo clean
cargo build

# Check Diesel CLI
diesel --version
```

## Next Steps

After successful setup:
1. Read the [Development Guide](DEVELOPMENT.md)
2. Explore the API documentation
3. Review the test suite
4. Check CI/CD workflows

## Production Deployment

**⚠️ Before deploying to production**:
1. Change all default passwords and secrets
2. Use HTTPS (configure Traefik Let's Encrypt)
3. Set `RUST_LOG=info` or `warn`
4. Enable database backups
5. Configure proper CORS origins
6. Review security headers
7. Set up monitoring alerts
8. Use environment-specific .env files
