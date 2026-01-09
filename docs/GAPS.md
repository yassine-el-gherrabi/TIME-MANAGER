# Time Manager - Fonctionnalités Manquantes

> Comparaison avec `docs/TIME MANAGER REFERENCE.md`
>
> Date: 2026-01-09 | Complétude actuelle: ~75%

---

## Ordre de Priorité (Validé)

### Sprint 1: Quick Wins - Dashboard & Profile (~3.5h)
> Objectif: Compléter l'UX de base et le dashboard Manager

| # | Tâche | Effort | Status |
|---|-------|--------|--------|
| 1.1 | Charts Recharts (Dashboard) | 2h | [x] |
| 1.2 | Phone field (migration) | 0.5h | [x] |
| 1.3 | Profile Page (frontend) | 1h | [x] |

**Personas débloqués**: Manager 90% → 100%

---

### Sprint 2: Notifications System (~5h)
> Objectif: Système de notifications complet (full stack)

| # | Tâche | Effort | Status |
|---|-------|--------|--------|
| 2.1 | Migration table `notifications` | 0.5h | [ ] |
| 2.2 | Backend: Model + Repository | 1h | [ ] |
| 2.3 | Backend: Service + Handlers | 1.5h | [ ] |
| 2.4 | Frontend: NotificationBell + Page | 2h | [ ] |

**Endpoints à créer**:
- `GET /v1/notifications`
- `PUT /v1/notifications/:id/read`
- `PUT /v1/notifications/read-all`

---

### Sprint 3: Audit Logs (~5h)
> Objectif: Traçabilité complète pour Super Admin

| # | Tâche | Effort | Status |
|---|-------|--------|--------|
| 3.1 | Migration table `audit_logs` | 0.5h | [ ] |
| 3.2 | Backend: Model + Repository | 1h | [ ] |
| 3.3 | Backend: Service + Middleware | 1.5h | [ ] |
| 3.4 | Frontend: AuditLogsPage | 2h | [ ] |

**Personas débloqués**: Super Admin 50% → 80%

---

### Sprint 4: Admin Features (~6h)
> Objectif: Compléter Admin et Super Admin

| # | Tâche | Effort | Status |
|---|-------|--------|--------|
| 4.1 | Export CSV (backend + UI) | 2h | [ ] |
| 4.2 | Organizations CRUD (backend) | 2h | [ ] |
| 4.3 | OrganizationSettingsPage | 1h | [ ] |
| 4.4 | Soft Delete Users | 1h | [ ] |

**Personas débloqués**: Admin 80% → 100%, Super Admin 80% → 100%

---

## Résumé

| Sprint | Durée | Personas Impactés | Status |
|--------|-------|-------------------|--------|
| Sprint 1 | 3.5h | Manager ✅ | [x] |
| Sprint 2 | 5h | Tous (notifications) | [ ] |
| Sprint 3 | 5h | Super Admin | [ ] |
| Sprint 4 | 6h | Admin + Super Admin ✅ | [ ] |
| **Total** | **19.5h** | **100% tous personas** | |

---

## Détails Techniques

### Sprint 1.1 - Charts Recharts

**Installation**:
```bash
cd frontend && npm install recharts
```

**Composants à créer**:
- `HoursBarChart` - Heures travaillées par jour (semaine)
- `TrendLineChart` - Évolution mensuelle des heures

**Fichiers impactés**:
- `frontend/src/pages/DashboardPage.tsx`
- `frontend/src/components/kpi/` (nouveaux composants)

---

### Sprint 1.2 - Phone Field

**Migration**:
```sql
ALTER TABLE users ADD COLUMN phone VARCHAR(20);
```

**Fichiers impactés**:
- `backend/migrations/` (nouvelle migration)
- `backend/src/schema.rs` (regenerate)
- `backend/src/models/user.rs`

---

### Sprint 1.3 - Profile Page

**Route**: `/profile`

**Fonctionnalités**:
- Afficher infos utilisateur
- Éditer: first_name, last_name, phone
- Pas d'édition email/role (admin only)

**Fichiers à créer**:
- `frontend/src/pages/ProfilePage.tsx`
- `frontend/src/components/profile/ProfileForm.tsx`

---

### Sprint 2 - Notifications

**Table**:
```sql
CREATE TABLE notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id),
    user_id UUID NOT NULL REFERENCES users(id),
    type VARCHAR(50) NOT NULL,
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    data JSONB,
    read_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

**Types de notifications**:
```typescript
type NotificationType =
  | 'absence_approved'
  | 'absence_rejected'
  | 'absence_pending'      // Manager
  | 'clock_correction'     // Manager
  | 'clock_approved'
  | 'clock_rejected';
```

---

### Sprint 3 - Audit Logs

**Table**:
```sql
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID REFERENCES organizations(id),
    user_id UUID REFERENCES users(id),
    action VARCHAR(50) NOT NULL,
    entity_type VARCHAR(100) NOT NULL,
    entity_id UUID NOT NULL,
    old_values JSONB,
    new_values JSONB,
    ip_address VARCHAR(45),
    user_agent VARCHAR(512),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

**Actions à logger**:
```rust
enum AuditAction {
    Create,
    Update,
    Delete,
    Login,
    Logout,
    PasswordChange,
}
```

---

### Sprint 4.1 - Export CSV

**Endpoint**: `GET /v1/reports/export`

**Query params**:
- `type`: `clocks` | `absences` | `users`
- `start_date`, `end_date`
- `user_id` (optional)

**Headers**: `Content-Type: text/csv`

---

### Sprint 4.2 - Organizations CRUD

**Endpoints** (Super Admin only):
- `GET /v1/organizations`
- `GET /v1/organizations/:id`
- `POST /v1/organizations`
- `PUT /v1/organizations/:id`
- `DELETE /v1/organizations/:id`

---

### Sprint 4.4 - Soft Delete

**Migration**:
```sql
ALTER TABLE users ADD COLUMN deleted_at TIMESTAMPTZ;
CREATE INDEX idx_users_active ON users(organization_id) WHERE deleted_at IS NULL;
```

**Endpoints**:
- `DELETE /v1/users/:id` → set deleted_at
- `PUT /v1/users/:id/restore` → unset deleted_at

---

## Personas - Projection Finale

| Persona | Avant | Après Sprint 1 | Après Sprint 2 | Après Sprint 3 | Après Sprint 4 |
|---------|-------|----------------|----------------|----------------|----------------|
| Employé | 100% | 100% | 100% | 100% | 100% |
| Manager | 90% | **100%** | 100% | 100% | 100% |
| Admin | 80% | 80% | 85% | 85% | **100%** |
| Super Admin | 50% | 50% | 55% | **80%** | **100%** |
