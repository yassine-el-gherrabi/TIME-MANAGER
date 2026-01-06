# TM-E07 : Pointage (Clock In/Out)

## Informations Epic

| Champ | Valeur |
|-------|--------|
| **ID** | TM-E07 |
| **Titre** | Pointage (Clock In/Out) |
| **Priorité** | P1 - Critique |
| **Estimation globale** | 26 SP |
| **Sprint cible** | Sprint 5 |
| **Dépendances** | TM-E06 (Horaires) |

---

## Description

### Contexte

Le pointage est la fonctionnalité centrale de Time Manager. Les employés pointent leur arrivée et leur départ chaque jour. Le système calcule automatiquement le temps de travail et permet de demander des corrections en cas d'oubli, qui sont validées par le manager.

### Objectif Business

Permettre un suivi précis et fiable du temps de travail, avec un processus de correction transparent qui responsabilise les employés tout en donnant aux managers le contrôle nécessaire.

### Valeur Apportée

- **Pour les employés** : Pointage simple en un clic, historique accessible, corrections possibles
- **Pour les managers** : Vue d'ensemble de l'équipe, validation des corrections, alertes anomalies
- **Pour les RH** : Données fiables pour la paie, traçabilité des modifications

---

## Scope

### Inclus

- Pointage arrivée (clock in) et départ (clock out)
- Calcul automatique de la durée travaillée
- Historique des pointages avec filtres
- Demande de correction avec workflow de validation
- Vue temps réel de la présence de l'équipe
- Widget de pointage pour le dashboard
- Empêchement du double pointage

### Exclus

- Pointage géolocalisé
- Pointage par badge/NFC
- Gestion des pauses (hors pause midi définie dans horaires)
- Heures supplémentaires avec validation spécifique

---

## Critères de Succès de l'Epic

- [ ] Un employé peut pointer son arrivée en un clic
- [ ] Un employé peut pointer son départ
- [ ] Le système empêche un double pointage le même jour
- [ ] Un employé peut voir son historique de pointages
- [ ] Un employé peut demander une correction (oubli, erreur)
- [ ] Un manager reçoit les demandes de correction de son équipe
- [ ] Un manager peut approuver ou rejeter une correction
- [ ] La vue "présence équipe" affiche le statut en temps réel

---

## User Stories

---

### TM-51 : Pointer son arrivée (Clock In)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** employé,
**Je veux** pointer mon arrivée au travail,
**Afin d'** enregistrer mon heure de début de journée.

#### Contexte Détaillé

Le clock in crée une nouvelle entrée de pointage avec l'heure actuelle. L'entrée reste "ouverte" (clock_out = null) jusqu'au pointage de départ.

#### Critères d'Acceptation

- [ ] Endpoint `POST /api/v1/clock/in` créé
- [ ] Crée une entrée avec clock_in = timestamp actuel
- [ ] clock_out = null (session ouverte)
- [ ] date = date du jour (timezone de l'organisation)
- [ ] status = "approved" (pas besoin de validation pour un pointage normal)
- [ ] is_manual = false (pointage automatique)
- [ ] Empêche le double clock in :
  - Si une session est déjà ouverte (clock_out = null) → erreur 409
- [ ] Empêche si déjà une session complète aujourd'hui → erreur 409
- [ ] Accessible à tous les rôles
- [ ] Retour : entrée de pointage créée

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-51.1 | Créer migration table clock_entries | 1h |
| TM-51.2 | Créer enum ClockStatus (pending, approved, rejected) | 0.5h |
| TM-51.3 | Créer modèle ClockEntry et repository | 1h |
| TM-51.4 | Implémenter ClockService.clock_in() | 1.5h |
| TM-51.5 | Créer endpoint POST /clock/in | 0.5h |
| TM-51.6 | Tests d'intégration (succès + erreurs) | 1.5h |

---

### TM-52 : Pointer son départ (Clock Out)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** employé,
**Je veux** pointer mon départ du travail,
**Afin d'** enregistrer mon heure de fin de journée.

#### Contexte Détaillé

Le clock out ferme la session ouverte et calcule automatiquement la durée travaillée.

#### Critères d'Acceptation

- [ ] Endpoint `POST /api/v1/clock/out` créé
- [ ] Trouve la session ouverte du jour (clock_out = null)
- [ ] Met à jour clock_out = timestamp actuel
- [ ] Calcule duration_minutes = clock_out - clock_in (en minutes)
- [ ] Erreur 400 si pas de session ouverte
- [ ] Accessible à tous les rôles
- [ ] Retour : entrée de pointage mise à jour

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-52.1 | Implémenter ClockRepository.find_open_session() | 0.5h |
| TM-52.2 | Implémenter calcul duration | 0.5h |
| TM-52.3 | Implémenter ClockService.clock_out() | 1h |
| TM-52.4 | Créer endpoint POST /clock/out | 0.5h |
| TM-52.5 | Tests d'intégration | 1h |

---

### TM-53 : Voir son pointage actuel

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 1 SP |
| **Assigné** | - |

#### Description

**En tant qu'** employé,
**Je veux** voir si j'ai une session de pointage en cours,
**Afin de** savoir si je dois pointer entrée ou sortie.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/clock/current` créé
- [ ] Retourne la session ouverte du jour si existe :
  - clock_in, elapsed_minutes (temps depuis clock_in)
- [ ] Retourne null/204 si pas de session ouverte
- [ ] Accessible à tous les rôles

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-53.1 | Implémenter ClockService.get_current() | 0.5h |
| TM-53.2 | Créer endpoint GET /clock/current | 0.5h |
| TM-53.3 | Tests d'intégration | 0.5h |

---

### TM-54 : Historique des pointages

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** employé,
**Je veux** voir mon historique de pointages,
**Afin de** vérifier mes heures passées.

#### Contexte Détaillé

L'historique permet de consulter tous les pointages passés avec des filtres. Les managers peuvent voir l'historique de leur équipe, les admins voient tout.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/clock` créé
- [ ] Filtres query params :
  - `user_id` : filtrer par utilisateur (manager/admin)
  - `team_id` : filtrer par équipe (manager/admin)
  - `start_date`, `end_date` : période
  - `status` : filtrer par statut
- [ ] Pagination (page, per_page)
- [ ] Tri par date décroissant par défaut
- [ ] Employee : voit uniquement ses pointages
- [ ] Manager : voit son équipe
- [ ] Admin : voit toute l'organisation
- [ ] Retourne : liste avec clock_in, clock_out, duration_minutes, status, date

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-54.1 | Implémenter ClockRepository.find_all() avec filtres | 1.5h |
| TM-54.2 | Implémenter logique permissions | 1h |
| TM-54.3 | Créer DTO ClockListQuery | 0.5h |
| TM-54.4 | Créer endpoint GET /clock | 0.5h |
| TM-54.5 | Tests d'intégration | 1h |

---

### TM-55 : Détail d'un pointage

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 1 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur,
**Je veux** voir le détail d'un pointage,
**Afin de** consulter toutes ses informations.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/clock/:id` créé
- [ ] Retourne toutes les infos + user info + notes
- [ ] Mêmes règles d'accès que l'historique
- [ ] Retour 404 si pointage d'une autre org ou non autorisé

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-55.1 | Créer ClockRepository.find_by_id() | 0.5h |
| TM-55.2 | Implémenter vérification permissions | 0.5h |
| TM-55.3 | Créer endpoint GET /clock/:id | 0.5h |
| TM-55.4 | Tests d'intégration | 0.5h |

---

### TM-56 : Demande de correction

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** employé,
**Je veux** demander une correction sur un pointage,
**Afin de** rectifier un oubli ou une erreur.

#### Contexte Détaillé

Si un employé oublie de pointer ou fait une erreur, il peut demander une correction. La demande passe en statut "pending" et attend la validation du manager.

Cas d'usage :
- Oubli de clock out → demander ajout de l'heure de départ
- Erreur de pointage → demander modification
- Oubli de clock in → demander création manuelle

#### Critères d'Acceptation

- [ ] Endpoint `POST /api/v1/clock/:id/request-correction` créé
- [ ] Body :
  ```json
  {
    "requested_clock_in": "09:00:00",
    "requested_clock_out": "18:00:00",
    "reason": "Oubli de pointer à l'arrivée"
  }
  ```
- [ ] Passe le status à "pending"
- [ ] Stocke les valeurs demandées dans des champs dédiés
- [ ] Seul le propriétaire du pointage peut demander
- [ ] Notification envoyée au manager
- [ ] Retour : pointage avec status pending

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-56.1 | Ajouter champs requested_* à clock_entries | 0.5h |
| TM-56.2 | Créer DTO CorrectionRequest | 0.5h |
| TM-56.3 | Implémenter ClockService.request_correction() | 1h |
| TM-56.4 | Créer endpoint POST /clock/:id/request-correction | 0.5h |
| TM-56.5 | Intégrer notification (préparer pour E10) | 0.5h |
| TM-56.6 | Tests d'intégration | 1h |

---

### TM-57 : Création manuelle de pointage

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** employé,
**Je veux** créer un pointage pour un jour où j'ai oublié de pointer,
**Afin de** régulariser ma situation.

#### Contexte Détaillé

Si l'employé a complètement oublié de pointer un jour, il peut créer une entrée manuelle qui sera soumise à validation.

#### Critères d'Acceptation

- [ ] Endpoint `POST /api/v1/clock/manual` créé
- [ ] Body :
  ```json
  {
    "date": "2024-01-15",
    "clock_in": "09:00:00",
    "clock_out": "18:00:00",
    "reason": "Oubli de pointage"
  }
  ```
- [ ] is_manual = true
- [ ] status = "pending" (requiert validation)
- [ ] Vérifie qu'il n'y a pas déjà un pointage pour ce jour
- [ ] Date ne peut pas être dans le futur
- [ ] Date ne peut pas être trop ancienne (> 30 jours configurable)
- [ ] Notification au manager
- [ ] Accessible à tous les rôles

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-57.1 | Créer DTO ManualClockRequest | 0.5h |
| TM-57.2 | Implémenter validations (date, duplicata) | 1h |
| TM-57.3 | Implémenter ClockService.create_manual() | 1h |
| TM-57.4 | Créer endpoint POST /clock/manual | 0.5h |
| TM-57.5 | Tests d'intégration | 1h |

---

### TM-58 : Approbation de correction

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** manager,
**Je veux** approuver une demande de correction,
**Afin de** valider les heures de mon équipe.

#### Critères d'Acceptation

- [ ] Endpoint `POST /api/v1/clock/:id/approve` créé
- [ ] Applique les valeurs requested_* → clock_in/clock_out
- [ ] Recalcule duration_minutes
- [ ] Met status = "approved"
- [ ] Met approved_by = current_user.id
- [ ] Met approved_at = now()
- [ ] Manager : uniquement pour son équipe
- [ ] Admin : pour toute l'organisation
- [ ] Notification au demandeur
- [ ] Retour : pointage mis à jour

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-58.1 | Ajouter champs approved_by, approved_at | 0.5h |
| TM-58.2 | Implémenter ClockService.approve() | 1h |
| TM-58.3 | Créer endpoint POST /clock/:id/approve | 0.5h |
| TM-58.4 | Tests d'intégration | 1h |

---

### TM-59 : Rejet de correction

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 1 SP |
| **Assigné** | - |

#### Description

**En tant que** manager,
**Je veux** rejeter une demande de correction,
**Afin de** refuser une demande non justifiée.

#### Critères d'Acceptation

- [ ] Endpoint `POST /api/v1/clock/:id/reject` créé
- [ ] Body : `{ "rejection_reason": "..." }`
- [ ] Met status = "rejected"
- [ ] Stocke rejection_reason
- [ ] Ne modifie pas les valeurs clock_in/clock_out
- [ ] Mêmes permissions que approve
- [ ] Notification au demandeur avec raison
- [ ] Retour : pointage mis à jour

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-59.1 | Ajouter champ rejection_reason | 0.5h |
| TM-59.2 | Implémenter ClockService.reject() | 0.5h |
| TM-59.3 | Créer endpoint POST /clock/:id/reject | 0.5h |
| TM-59.4 | Tests d'intégration | 0.5h |

---

### TM-60 : Liste des corrections en attente

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** manager,
**Je veux** voir toutes les demandes de correction en attente,
**Afin de** les traiter rapidement.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/clock/pending-corrections` créé
- [ ] Retourne tous les pointages avec status = "pending"
- [ ] Manager : uniquement son équipe
- [ ] Admin : toute l'organisation
- [ ] Tri par date de demande (plus ancien en premier)
- [ ] Inclut infos user + requested_* + reason

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-60.1 | Implémenter ClockRepository.find_pending() | 1h |
| TM-60.2 | Créer endpoint GET /clock/pending-corrections | 0.5h |
| TM-60.3 | Tests d'intégration | 1h |

---

### TM-61 : Widget de pointage (Frontend)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** employé,
**Je veux** un widget simple pour pointer,
**Afin de** pointer en un clic depuis mon dashboard.

#### Critères d'Acceptation

- [ ] Composant ClockWidget créé
- [ ] Affiche l'état actuel :
  - Pas pointé aujourd'hui → bouton "Pointer l'arrivée"
  - Session en cours → bouton "Pointer le départ" + chrono temps écoulé
  - Session terminée → résumé de la journée
- [ ] Animation au clic (feedback visuel)
- [ ] Mise à jour temps réel du chrono (toutes les secondes)
- [ ] Gestion des erreurs (double pointage, etc.)
- [ ] Affichage heure de pointage après action

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-61.1 | Créer hook useClock (current, clockIn, clockOut) | 1h |
| TM-61.2 | Créer composant ClockWidget | 2h |
| TM-61.3 | Implémenter chrono temps réel | 1h |
| TM-61.4 | Ajouter animations et feedback | 1h |
| TM-61.5 | Tests composant | 1h |

---

### TM-62 : Page historique pointages (Frontend)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** employé,
**Je veux** une page pour voir mon historique de pointages,
**Afin de** vérifier mes heures passées.

#### Critères d'Acceptation

- [ ] Page `/clock/history` créée
- [ ] Tableau avec colonnes : Date, Arrivée, Départ, Durée, Statut, Actions
- [ ] Filtres :
  - Période (date picker range)
  - Statut (tous, approuvé, en attente, rejeté)
- [ ] Vue par jour/semaine/mois (toggle)
- [ ] Total heures sur la période affichée
- [ ] Bouton "Demander correction" sur chaque ligne
- [ ] Bouton "Ajouter pointage manquant"
- [ ] Indicateurs visuels :
  - Badge couleur pour status
  - Icône si correction demandée
- [ ] Responsive (cards sur mobile)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-62.1 | Créer hook useClockHistory avec filtres | 1h |
| TM-62.2 | Créer composant ClockHistoryTable | 2h |
| TM-62.3 | Créer composants filtres (DateRangePicker, StatusFilter) | 1h |
| TM-62.4 | Créer modal RequestCorrectionModal | 1h |
| TM-62.5 | Créer modal ManualClockModal | 1h |
| TM-62.6 | Créer page ClockHistoryPage | 1h |
| TM-62.7 | Tests composants | 1h |

---

### TM-63 : Page validations manager (Frontend)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant que** manager,
**Je veux** une page pour valider les demandes de correction,
**Afin de** traiter rapidement les demandes de mon équipe.

#### Critères d'Acceptation

- [ ] Page `/clock/validations` créée
- [ ] Liste des demandes en attente avec :
  - Employé (nom, photo placeholder)
  - Date du pointage
  - Valeurs actuelles vs demandées (mise en évidence des différences)
  - Raison de la demande
  - Date de la demande
- [ ] Actions par ligne :
  - Bouton Approuver (vert)
  - Bouton Rejeter (rouge) → modal avec raison obligatoire
- [ ] Badge compteur dans la sidebar (nombre en attente)
- [ ] Message si aucune demande en attente
- [ ] Notification toast après action

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-63.1 | Créer hook usePendingCorrections | 1h |
| TM-63.2 | Créer composant CorrectionCard | 1.5h |
| TM-63.3 | Créer modal RejectCorrectionModal | 1h |
| TM-63.4 | Implémenter mutations approve/reject | 1h |
| TM-63.5 | Créer page ClockValidationsPage | 1h |
| TM-63.6 | Ajouter badge sidebar | 0.5h |
| TM-63.7 | Tests composants | 1h |

---

### TM-64 : Vue présence équipe (Frontend)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant que** manager,
**Je veux** voir qui est présent dans mon équipe en temps réel,
**Afin de** suivre la présence au bureau.

#### Critères d'Acceptation

- [ ] Page `/team/presence` ou section dans dashboard manager
- [ ] Liste des membres de l'équipe avec :
  - Nom
  - Statut (présent/absent/pas encore pointé)
  - Heure d'arrivée si présent
  - Temps de travail aujourd'hui
- [ ] Indicateurs visuels (pastille verte/rouge/grise)
- [ ] Filtre par équipe (si manager multi-équipes)
- [ ] Rafraîchissement automatique (polling ou websocket)
- [ ] Compteurs : X présents / Y total

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-64.1 | Créer endpoint GET /clock/team-presence | 1h |
| TM-64.2 | Créer hook useTeamPresence avec polling | 1h |
| TM-64.3 | Créer composant PresenceCard | 1h |
| TM-64.4 | Créer composant TeamPresenceList | 1h |
| TM-64.5 | Créer page/section TeamPresencePage | 1h |
| TM-64.6 | Tests composants | 1h |

---

## Récapitulatif des Estimations

| Story | Titre | SP |
|-------|-------|:--:|
| TM-51 | Pointer son arrivée (Clock In) | 3 |
| TM-52 | Pointer son départ (Clock Out) | 2 |
| TM-53 | Voir son pointage actuel | 1 |
| TM-54 | Historique des pointages | 3 |
| TM-55 | Détail d'un pointage | 1 |
| TM-56 | Demande de correction | 3 |
| TM-57 | Création manuelle de pointage | 2 |
| TM-58 | Approbation de correction | 2 |
| TM-59 | Rejet de correction | 1 |
| TM-60 | Liste des corrections en attente | 2 |
| TM-61 | Widget de pointage (Frontend) | 3 |
| TM-62 | Page historique pointages (Frontend) | 3 |
| TM-63 | Page validations manager (Frontend) | 3 |
| TM-64 | Vue présence équipe (Frontend) | 3 |
| **Total** | | **32 SP** |

---

## Notes Techniques

### Modèle ClockEntry

```rust
pub struct ClockEntry {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub clock_in: Option<NaiveTime>,
    pub clock_out: Option<NaiveTime>,
    pub duration_minutes: Option<i32>,
    pub is_manual: bool,
    pub status: ClockStatus,
    // Champs correction
    pub requested_clock_in: Option<NaiveTime>,
    pub requested_clock_out: Option<NaiveTime>,
    pub correction_reason: Option<String>,
    pub rejection_reason: Option<String>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub enum ClockStatus {
    Approved,
    Pending,
    Rejected,
}
```

### Endpoints Récapitulatif

| Méthode | Endpoint | Permission |
|---------|----------|------------|
| POST | /api/v1/clock/in | All |
| POST | /api/v1/clock/out | All |
| GET | /api/v1/clock/current | All |
| GET | /api/v1/clock | All* |
| GET | /api/v1/clock/:id | All* |
| POST | /api/v1/clock/:id/request-correction | Owner |
| POST | /api/v1/clock/manual | All |
| POST | /api/v1/clock/:id/approve | Manager*, Admin |
| POST | /api/v1/clock/:id/reject | Manager*, Admin |
| GET | /api/v1/clock/pending-corrections | Manager*, Admin |
| GET | /api/v1/clock/team-presence | Manager*, Admin |

*Manager = uniquement son équipe, All = self ou selon permissions

### Workflow de Correction

```
Pointage normal
      │
      ▼
   APPROVED ◄────────────────┐
      │                      │
      │ Demande correction   │ Approve
      ▼                      │
   PENDING ──────────────────┤
      │                      │
      │ Reject               │
      ▼                      │
   REJECTED                  │
```

### Calcul de Durée

```rust
fn calculate_duration(clock_in: NaiveTime, clock_out: NaiveTime) -> i32 {
    (clock_out - clock_in).num_minutes() as i32
}
```
