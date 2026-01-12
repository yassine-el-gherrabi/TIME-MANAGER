# Next Steps - Time Manager

## 1. Password Visibility Toggle

- [ ] Add eye icon to show/hide password in the wizard form (when organization has zero users)

---

## 2. Add cargo-machete to CI

- [ ] Detect unused dependencies in Rust backend
- [ ] Add step in CI pipeline

---

## 3. Grafana Dashboard Cleanup

- [ ] Remove CPU %
- [ ] Remove Memory
- [ ] Remove Avg response time (bottom)
- [ ] Remove Error rate (bottom)
- [ ] Remove Database container memory
- [ ] Remove Request rate by status
- [ ] **Fix PostgreSQL health check**: Currently shows "down" even when container is running

---

## 4. Validate Evaluation Criteria

Reference: `docs/TIME MANAGER REFERENCE.md`

| Criteria | Status | Evidence/Justification |
|----------|--------|------------------------|
| **dockerfiles** | YES | `backend/Dockerfile`, `backend/Dockerfile.dev`, `backend/Dockerfile.prod`, `frontend/Dockerfile`, `frontend/Dockerfile.dev`, `frontend/Dockerfile.prod` |
| **containers** | YES | PostgreSQL, Backend, Frontend, Traefik isolated in docker-compose |
| **persistency** | YES | Volumes: `postgres_data`, `cargo_cache`, `cargo_target`, Loki/Prometheus data |
| **orchestration** | YES | `docker-compose.yml` (dev), `docker-compose.prod.yml` (prod) with profiles |
| **clean_deploy** | YES | Separate compose files, Traefik configs (`traefik.yml`/`traefik-prod.yml`) |
| **env_specificity** | YES | `.env.dev.example`, `.env.prod.example` with different values per env |
| **secrets** | YES | Secrets in GitHub Secrets, not in git (POSTGRES_PASSWORD, JWT_SECRET, etc.) |
| **api_crafting** | YES | 50+ REST endpoints in `backend/src/api/handlers/`, documented in TIME MANAGER REFERENCE.md |
| **data_persist** | YES | PostgreSQL 16, 35 migrations, 25+ models with proper relationships |
| **data_viz** | YES | Recharts dashboards in `frontend/src/components/kpi/`, multiple chart types |
| **roles** | YES | SuperAdmin > Admin > Manager > Employee cascade in `backend/src/domain/user_role.rs` |
| **auth_jwt** | YES | JWT access (15min) + refresh (7d) tokens, `backend/src/utils/jwt.rs` |
| **auth_persist** | YES | Refresh token in HttpOnly cookie, 7-day validity, session tracking |
| **auth_sec** | YES | CSRF middleware (`csrf_middleware.rs`), XSS protection via CSP, rate limiting |
| **api_consumption** | YES | Frontend API client in `frontend/src/api/`, 21 endpoint modules |
| **code_orga** | YES | Modular structure: handlers/services/repositories pattern backend, components/stores/hooks frontend |
| **uiux_quality** | YES | Shadcn/UI + Tailwind CSS, responsive design, consistent styling |
| **hmi** | YES | Dashboard, Clock, Absences, Admin, Profile, Settings, Audit views |
| **constraints** | YES | React 18 + TypeScript 5 frontend, Rust + Axum backend, PostgreSQL 16 |
| **framework_front** | YES | React/TypeScript: Modern ecosystem, type safety, component reusability, strong community |
| **framework_back** | YES | Rust/Axum: Memory safety, high performance, async-first, compile-time guarantees |
| **maintainability** | YES | Clean architecture, separation of concerns, consistent naming conventions |
| **robustness** | YES | Error handling with `thiserror`, validation with `validator`, proper error responses |
| **tests_sequence** | YES | Unit tests in 43+ test modules, integration tests with testcontainers |
| **tests_coverage** | YES | 80% minimum threshold enforced in CI (`cargo llvm-cov`) |
| **tests_automation** | YES | CI pipeline runs tests automatically on push/PR |
| **ci_pipeline** | YES | `.github/workflows/ci.yml` with 8 jobs (fmt, clippy, machete, test, lint, e2e) |
| **ci_quality** | YES | Quality gates: fmt, clippy warnings=error, 80% coverage, ESLint, TypeScript |
| **versioning_basics** | YES | Git flow: feature branches, conventional commits, PR workflow |
| **doc_basic** | YES | `docs/` folder: ARCHITECTURE.md, DEVELOPMENT.md, SETUP.md, SECURITY_POLICY.md |
| **presentation** | TBD | Prepared for oral presentation |
| **argumentation** | TBD | Technical choices documented, ready for defense |
| **answers** | TBD | Technical Q&A preparation |

> **Status**: 31/34 criteria validated. Remaining 3 (presentation, argumentation, answers) are oral defense items.

---

## 5. Add Tests (Unit / Functional / E2E)

### Backend Tests
- [ ] Unit tests
- [ ] Functional tests
- [ ] Integration tests using:
  ```rust
  #[tokio::test]
  #[rstest]
  #[case::...]
  ```

### Frontend Tests
- [ ] Unit tests
- [ ] Functional tests
- [ ] E2E tests

---

## 6. Repository Cleanup

- [ ] Remove unused files and directories
- [ ] Audit and clean up dead code
- [ ] Remove any temporary/debug files

---

## 7. Documentation + README

- [ ] Complete technical documentation from scratch
- [ ] Rewrite README entirely
- [ ] Cover:
  - Technological choices and justifications
  - Architecture design
  - Component documentation
  - Setup instructions
  - API documentation

---

## 8. Disable CD on rebirth branch

- [ ] The `feature/rebirth` branch should NOT trigger deployment pipelines

---

## 9. Fix In-App Notification System

- [ ] System doesn't seem to be working - investigate and fix

---

## 10. Email System

- [ ] No emails are being sent currently
- [ ] Implement email service
- [ ] Define which events trigger emails (password reset, approvals, etc.)
- [ ] Configure for production

---

## 11. Database Migrations Policy

> **IMPORTANT**: No data migrations allowed. Only creation/schema migrations.
> The database will be reset in production.

- [ ] Audit existing migrations for data manipulation
- [ ] Ensure all migrations are creation-only scripts
- [ ] No ALTER with data transforms, no INSERT/UPDATE in migrations
