# Fonctionnalités

> Documentation des flux fonctionnels de Time Manager

---

## Vue d'ensemble

```mermaid
graph TB
    subgraph Core["🎯 Core Features"]
        Auth["🔐 Authentification"]
        Clock["⏱️ Pointage"]
        Absences["🏖️ Absences"]
    end

    subgraph Management["👥 Management"]
        RBAC["👤 RBAC"]
        Teams["👥 Équipes"]
        Schedules["📅 Plannings"]
    end

    subgraph Analytics["📊 Analytics"]
        KPIs["📈 KPIs"]
        Reports["📋 Rapports"]
    end

    Auth --> Clock
    Auth --> Absences
    RBAC --> Teams
    Teams --> Clock
    Teams --> Absences
    Schedules --> Clock
    Clock --> KPIs
    Absences --> KPIs
    KPIs --> Reports
```

---

## Fonctionnalités par rôle

| Fonctionnalité | Employee | Manager | Admin | Super Admin |
|----------------|----------|---------|-------|-------------|
| Clock in/out | ✅ | ✅ | ✅ | ✅ |
| Voir mes KPIs | ✅ | ✅ | ✅ | ✅ |
| Demander absence | ✅ | ✅ | ✅ | ✅ |
| Approuver pointages | ❌ | ✅ | ✅ | ✅ |
| Approuver absences | ❌ | ✅ | ✅ | ✅ |
| Gérer équipes | ❌ | ❌ | ✅ | ✅ |
| Gérer utilisateurs | ❌ | ❌ | ✅ | ✅ |
| Gérer organisations | ❌ | ❌ | ❌ | ✅ |
| Voir audit logs | ❌ | ❌ | ❌ | ✅ |

---

## Documentation détaillée

| Document | Description |
|----------|-------------|
| [Auth Flow](./auth-flow.md) | Authentification, sessions, invitations |
| [RBAC](./rbac.md) | Rôles, permissions, hiérarchie |
| [Clock Management](./clock-management.md) | Pointage, approbations, restrictions |
| [Absence Management](./absence-management.md) | Congés, workflow, soldes |
| [Schedules](./schedules.md) | Plannings de travail |
| [KPIs](./kpis.md) | Indicateurs et analytics |

---

## Flux principaux

### Journée type d'un employé

```mermaid
sequenceDiagram
    participant E as Employé
    participant S as Système

    E->>S: Login
    S-->>E: Dashboard

    Note over E,S: Début de journée
    E->>S: Clock In
    S-->>E: Confirmation

    Note over E,S: Pause déjeuner
    E->>S: Clock Out
    E->>S: Clock In

    Note over E,S: Fin de journée
    E->>S: Clock Out
    S-->>E: Résumé journée

    E->>S: Logout
```

### Workflow d'absence

```mermaid
stateDiagram-v2
    [*] --> Draft: Créer demande
    Draft --> Pending: Soumettre
    Pending --> Approved: Manager approuve
    Pending --> Rejected: Manager refuse
    Approved --> [*]
    Rejected --> Draft: Modifier
    Draft --> Cancelled: Annuler
    Cancelled --> [*]
```

---

## Liens connexes

- [API Reference](../api/)
- [Architecture](../architecture/)
