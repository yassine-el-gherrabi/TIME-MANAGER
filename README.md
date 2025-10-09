# ⏰ Time Manager

> A modern time tracking and workforce management application built with Go, React, and KrakenD.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Go](https://img.shields.io/badge/Go-1.21+-00ADD8?logo=go)](https://golang.org/)
[![React](https://img.shields.io/badge/React-19.2+-61DAFB?logo=react)](https://reactjs.org/)

## 📋 Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Architecture](#architecture)
- [Tech Stack](#tech-stack)
- [Getting Started](#getting-started)
- [Project Structure](#project-structure)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [License](#license)

## 🎯 Overview

Time Manager is a comprehensive workforce management solution designed for companies to track employee working hours, manage teams, and generate insightful reports. The application provides role-based access for employees and managers with an intuitive web interface.

## ✨ Features

### For Employees
- ⏱️ Clock in/out functionality
- 📊 View personal working hours and history
- 📅 Monthly calendar view of attendance
- 📈 Personal statistics and reports

### For Managers
- 👥 User and team management (CRUD operations)
- 📊 Dashboard with Key Performance Indicators (KPIs)
- 📈 Team performance reports and analytics
- 👀 Monitor employee working hours
- 📉 Lateness and overtime tracking

## 🏗️ Architecture

```
┌─────────────┐
│   Client    │
│  (Browser)  │
└──────┬──────┘
       │
       │ HTTP
       ▼
┌─────────────┐
│   KrakenD   │ ◄─── Reverse Proxy & API Gateway
│ (Port 8000) │
└──────┬──────┘
       │
       ├─── /api/*  ──────────┐
       │                      │
       └─── /*  ──┐           │
                  │           │
         ┌────────▼──────┐   │
         │   Frontend    │   │
         │    (React)    │   │
         │  (Port 3000)  │   │
         └───────────────┘   │
                             │
                  ┌──────────▼────────┐
                  │     Backend       │
                  │       (Go)        │
                  │   (Port 8080)     │
                  └──────────┬────────┘
                             │
                  ┌──────────▼────────┐
                  │    Database       │
                  │  (PostgreSQL)     │
                  │   (Port 5432)     │
                  └───────────────────┘
```

## 🛠️ Tech Stack

### Backend
- **Language**: Go 1.21+
- **Framework**: Net/HTTP (standard library) with custom routing
- **Database**: PostgreSQL (or MongoDB - configurable)
- **Authentication**: JWT (JSON Web Tokens)
- **Testing**: Go testing package

**Why Go?**
- High performance and concurrency
- Static typing and compilation
- Simple and maintainable code
- Built-in testing and tooling

### Frontend
- **Framework**: React 19.2+
- **UI Library**: Magic UI + Tailwind CSS
- **Routing**: React Router v6
- **State Management**: React Context API
- **HTTP Client**: Axios
- **Form Management**: React Hook Form + Yup
- **Testing**: Jest + React Testing Library

**Why React + Magic UI?**
- Component-based architecture
- Large ecosystem and community
- Magic UI provides modern, accessible components
- Excellent performance with hooks

### Infrastructure
- **Reverse Proxy**: KrakenD (API Gateway)
- **Containerization**: Docker + Docker Compose
- **CI/CD**: GitHub Actions
- **Version Control**: Git + GitHub

**Why KrakenD?**
- Lightweight and fast API Gateway
- Built-in rate limiting and CORS
- Easy configuration
- Perfect for microservices

## 🚀 Getting Started

### Prerequisites

- **Docker** 24.0+
- **Docker Compose** 2.0+
- **Node.js** 18+ (for local frontend development)
- **Go** 1.21+ (for local backend development)
- **Git**

### Quick Start with Docker

1. **Clone the repository**
   ```bash
   git clone https://github.com/EPITECHMSC/time-manager.git
   cd time-manager
   ```

2. **Configure environment variables**
   ```bash
   # Copy example files
   cp .env.example .env
   cp back/.env.example back/.env
   cp front/.env.example front/.env.development

   # Edit the files with your values
   nano .env
   nano back/.env
   nano front/.env.development
   ```

3. **Launch the application**
   ```bash
   docker-compose up --build
   ```

4. **Access the application**
   - Frontend: http://localhost:8000
   - Backend API: http://localhost:8000/api
   - Direct Backend (dev): http://localhost:8080

### Local Development (without Docker)

#### Backend
```bash
cd back
cp .env.example .env
# Edit .env with your database credentials
go mod download
go run main.go
```

#### Frontend
```bash
cd front
cp .env.example .env.development
npm install
npm start
```

## 📁 Project Structure

```
time-manager/
├── back/                   # Backend (Go)
│   ├── cmd/               # Application entrypoints
│   ├── internal/          # Private application code
│   │   ├── api/          # HTTP handlers
│   │   ├── auth/         # Authentication logic
│   │   ├── models/       # Data models
│   │   ├── repository/   # Database layer
│   │   └── service/      # Business logic
│   ├── pkg/              # Public libraries
│   ├── migrations/       # Database migrations
│   └── tests/            # Backend tests
│
├── front/                 # Frontend (React)
│   ├── public/           # Static assets
│   └── src/
│       ├── api/          # API services
│       ├── components/   # React components
│       ├── contexts/     # React contexts
│       ├── hooks/        # Custom hooks
│       ├── pages/        # Page components
│       ├── routes/       # Routing configuration
│       └── utils/        # Utility functions
│
├── docs/                  # Documentation
│   ├── architecture.md   # Architecture details
│   ├── api.md           # API documentation
│   └── kpis.md          # KPI definitions
│
├── .github/              # GitHub configuration
│   └── workflows/       # CI/CD pipelines
│
├── docker-compose.yml    # Docker services (development)
├── docker-compose.prod.yml # Production configuration
├── krakend.json         # KrakenD configuration
├── COMMIT_CONVENTION.md # Commit message guidelines
├── CONTRIBUTING.md      # Contribution guide
└── README.md            # This file
```

## 📚 Documentation

- [Architecture Details](docs/architecture.md) - System design and data flow
- [API Documentation](docs/api.md) - REST API endpoints and examples
- [KPI Definitions](docs/kpis.md) - Key Performance Indicators explained
- [Commit Conventions](COMMIT_CONVENTION.md) - Git commit message format
- [Contributing Guide](CONTRIBUTING.md) - How to contribute to this project

## 🧪 Testing

### Backend Tests
```bash
cd back
go test ./... -v
go test ./... -cover
```

### Frontend Tests
```bash
cd front
npm test
npm test -- --coverage
```

### CI/CD
All tests are automatically run on every pull request via GitHub Actions.

## 🤝 Contributing

We welcome contributions! Please read our [Contributing Guide](CONTRIBUTING.md) for details on:

- Git workflow and branch strategy
- Commit message conventions
- Pull request process
- Code review guidelines

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👥 Team

- **Yassine EL GHERRABI** - Project Lead

## 🙏 Acknowledgments

- EPITECH for the project subject
- The Go and React communities
- All contributors to this project

---

**Note**: This project is part of the EPITECH MSC curriculum (Master of Science in Computer Science).

For support or questions, please open an issue on GitHub.
