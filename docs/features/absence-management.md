# Absence Management

> Gestion des congés et absences avec workflow d'approbation

---

## Vue d'ensemble

```mermaid
graph TB
    subgraph Employee["👤 Employee"]
        Request["📝 Demande"]
        Balance["💰 Solde"]
        History["📜 Historique"]
    end

    subgraph Workflow["🔄 Workflow"]
        Pending["⏳ En attente"]
        Approve["✅ Approuver"]
        Reject["❌ Rejeter"]
    end

    subgraph Admin["🏢 Admin"]
        Types["📋 Types d'absence"]
        Balances["💰 Soldes"]
        Config["⚙️ Configuration"]
    end

    Request --> Pending
    Pending --> Approve
    Pending --> Reject
    Balance --> Request
    Types --> Request
    Balances --> Balance
```

---

## Types d'absence

### Types prédéfinis

| Type | Décompte solde | Approbation requise | Exemple |
|------|----------------|---------------------|---------|
| Congés payés | ✅ | ✅ | Vacances annuelles |
| RTT | ✅ | ✅ | Réduction temps travail |
| Maladie | ❌ | ✅ | Arrêt maladie |
| Maternité/Paternité | ❌ | ✅ | Congé parental |
| Sans solde | ❌ | ✅ | Congé non rémunéré |
| Formation | ❌ | ✅ | Formation professionnelle |
| Événement familial | ❌ | ✅ | Mariage, décès |

### Configuration d'un type

```json
{
  "id": "uuid",
  "organization_id": "uuid",
  "name": "Congés payés",
  "code": "CP",
  "color": "#4CAF50",
  "deducts_balance": true,
  "requires_approval": true,
  "requires_justification": false,
  "max_consecutive_days": 30,
  "min_notice_days": 14,
  "active": true
}
```

---

## Workflow des demandes

### Cycle de vie

```mermaid
stateDiagram-v2
    [*] --> Draft: Créer
    Draft --> Pending: Soumettre
    Pending --> Approved: Manager approuve
    Pending --> Rejected: Manager refuse

    Draft --> Cancelled: Annuler
    Pending --> Cancelled: Annuler

    Approved --> [*]: Terminé
    Rejected --> Draft: Modifier
    Cancelled --> [*]: Terminé
```

### États

| État | Description | Actions possibles |
|------|-------------|-------------------|
| `draft` | Brouillon | Modifier, Soumettre, Annuler |
| `pending` | En attente approbation | Annuler (employee), Approuver/Rejeter (manager) |
| `approved` | Approuvé | - |
| `rejected` | Rejeté | Modifier et re-soumettre |
| `cancelled` | Annulé | - |

---

## Création d'une demande

### Flux

```mermaid
sequenceDiagram
    participant E as Employee
    participant B as Backend
    participant DB as Database

    E->>B: POST /absences
    Note over E,B: {type_id, start_date, end_date, reason?}

    B->>DB: Get absence type
    B->>B: Validate dates

    alt Dates invalides
        B-->>E: 400 Invalid dates
    else Dates OK
        B->>DB: Check balance (if deducts)

        alt Solde insuffisant
            B-->>E: 400 Insufficient balance
        else Solde OK
            B->>DB: Check conflicts

            alt Conflit existant
                B-->>E: 409 Conflict with existing absence
            else Pas de conflit
                B->>DB: Create absence
                B->>B: Notify manager
                B-->>E: 201 Absence created
            end
        end
    end
```

### Validations

| Règle | Description |
|-------|-------------|
| Dates | `start_date` ≤ `end_date` |
| Futur | `start_date` ≥ aujourd'hui |
| Préavis | Respect du `min_notice_days` |
| Durée max | Respect du `max_consecutive_days` |
| Solde | Si `deducts_balance`, solde suffisant |
| Conflits | Pas de chevauchement avec autre absence |

---

## Approbation

### Vue Manager

```mermaid
graph LR
    subgraph Queue["📋 Demandes en attente"]
        A1["Alice<br/>15-20 Jan<br/>CP"]
        A2["Bob<br/>22 Jan<br/>RTT"]
        A3["Carol<br/>1-5 Feb<br/>Formation"]
    end

    subgraph Actions["Actions"]
        Approve["✅ Approuver"]
        Reject["❌ Rejeter"]
        Details["📄 Détails"]
    end

    A1 --> Actions
    A2 --> Actions
    A3 --> Actions
```

### Approbation

```mermaid
sequenceDiagram
    participant M as Manager
    participant B as Backend
    participant DB as Database
    participant N as Notification

    M->>B: POST /absences/:id/approve
    Note over M,B: {notes?: string}

    B->>DB: Get absence
    B->>B: Verify manager scope

    alt Not pending
        B-->>M: 400 Not pending
    else Is pending
        B->>DB: Update status = approved
        B->>DB: Deduct balance (if applicable)
        B->>N: Notify employee
        B-->>M: 200 Approved
    end
```

### Rejet

```mermaid
sequenceDiagram
    participant M as Manager
    participant B as Backend
    participant DB as Database
    participant N as Notification

    M->>B: POST /absences/:id/reject
    Note over M,B: {reason: string}

    B->>DB: Get absence
    B->>DB: Update status = rejected
    B->>DB: Store rejection reason
    B->>N: Notify employee with reason
    B-->>M: 200 Rejected
```

---

## Gestion des soldes

### Structure

```json
{
  "id": "uuid",
  "user_id": "uuid",
  "absence_type_id": "uuid",
  "year": 2024,
  "initial_balance": 25.0,
  "used": 10.0,
  "pending": 5.0,
  "remaining": 10.0,
  "carry_over": 3.0
}
```

### Calculs

```
remaining = initial_balance + carry_over - used - pending
```

### Ajustements

```mermaid
sequenceDiagram
    participant A as Admin
    participant B as Backend
    participant DB as Database

    A->>B: PUT /balances/:id/adjust
    Note over A,B: {adjustment: +5, reason: "Correction"}

    B->>DB: Update balance
    B->>DB: Log adjustment in audit
    B-->>A: 200 Balance adjusted
```

---

## Calendrier des absences

### Vue équipe

```mermaid
gantt
    title Absences Janvier 2024
    dateFormat  YYYY-MM-DD
    section Alice
    CP           :2024-01-15, 5d
    section Bob
    RTT          :2024-01-22, 1d
    section Carol
    Formation    :2024-01-08, 3d
```

### Conflits détectés

```json
{
  "error": "conflict",
  "message": "Absence conflicts with existing approved absence",
  "conflict": {
    "id": "uuid",
    "type": "Congés payés",
    "dates": "2024-01-15 to 2024-01-20"
  }
}
```

---

## Endpoints

### Employee

| Endpoint | Méthode | Description |
|----------|---------|-------------|
| `/absences` | GET | Lister mes absences |
| `/absences` | POST | Créer une demande |
| `/absences/:id` | GET | Détails d'une absence |
| `/absences/:id/cancel` | POST | Annuler ma demande |
| `/balances/me` | GET | Voir mes soldes |

### Manager

| Endpoint | Méthode | Description |
|----------|---------|-------------|
| `/absences/pending` | GET | Demandes en attente (équipe) |
| `/absences/:id/approve` | POST | Approuver |
| `/absences/:id/reject` | POST | Rejeter |

### Admin

| Endpoint | Méthode | Description |
|----------|---------|-------------|
| `/absence-types` | GET/POST | Gérer les types |
| `/absence-types/:id` | GET/PUT/DELETE | CRUD type |
| `/balances` | GET | Tous les soldes |
| `/balances/:id/adjust` | PUT | Ajuster un solde |

---

## Notifications

### Événements

| Événement | Destinataire | Message |
|-----------|--------------|---------|
| Demande créée | Manager | "Nouvelle demande de {user}" |
| Demande approuvée | Employee | "Votre demande a été approuvée" |
| Demande rejetée | Employee | "Votre demande a été rejetée: {reason}" |
| Demande annulée | Manager | "{user} a annulé sa demande" |
| Solde bas | Employee | "Votre solde de CP est bas: {remaining} jours" |

---

## Rapports

### Métriques

| Métrique | Description |
|----------|-------------|
| Taux d'absence | % jours absence / jours travaillés |
| Jours par type | Répartition par type d'absence |
| Soldes moyens | Moyenne des soldes restants |
| Absences approuvées | Nombre et durée totale |

### Export

```
GET /reports/export?type=absences&from=2024-01-01&to=2024-12-31&format=csv
```

---

## Configuration avancée

### Politiques d'entreprise

```yaml
absence_policies:
  # Report des congés
  carry_over:
    enabled: true
    max_days: 5
    expiry_date: "2024-03-31"

  # Blackout periods
  blackout:
    - start: "2024-12-20"
      end: "2024-12-31"
      reason: "Fermeture annuelle"

  # Approbation automatique
  auto_approve:
    max_days: 2
    types: ["RTT"]
```

---

## Liens connexes

- [RBAC](./rbac.md)
- [Clock Management](./clock-management.md)
- [KPIs](./kpis.md)
