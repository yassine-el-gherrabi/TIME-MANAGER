# TM-E08 : Absences & Congés

## Informations Epic

| Champ | Valeur |
|-------|--------|
| **ID** | TM-E08 |
| **Titre** | Absences & Congés |
| **Priorité** | P1 - Haute |
| **Estimation globale** | 34 SP |
| **Sprint cible** | Sprint 4-5 |
| **Dépendances** | TM-E05 (Équipes), TM-E04 (Utilisateurs) |

---

## Description

### Contexte

La gestion des absences est un pilier de toute application de gestion du temps. Les employés doivent pouvoir demander des congés (payés, maladie, sans solde, etc.), et les managers doivent pouvoir valider ou refuser ces demandes. Le système doit également gérer les soldes de congés et les jours fériés de l'organisation.

### Objectif Business

Digitaliser et automatiser le workflow complet de demande et validation des absences, en assurant une traçabilité totale et une gestion précise des soldes de congés.

### Valeur Apportée

- **Pour les employés** : Demande d'absence simple avec visibilité sur soldes et statut
- **Pour les managers** : Validation rapide avec vue calendrier des absences équipe
- **Pour les admins** : Configuration des types d'absences et jours fériés
- **Pour l'organisation** : Réduction des erreurs de calcul et traçabilité complète

---

## Scope

### Inclus

- Types d'absences configurables par organisation
- Workflow de demande/validation/refus
- Gestion des soldes de congés par type
- Configuration des jours fériés par organisation
- Calendrier des absences équipe
- Annulation de demande (avant date de début)

### Exclus

- Import automatique des jours fériés nationaux
- Calcul automatique des droits acquis (prorata)
- Reports de congés d'une année sur l'autre
- Absences récurrentes (tous les lundis)
- Intégration paie externe

---

## Critères de Succès de l'Epic

- [ ] Un employé peut soumettre une demande d'absence
- [ ] Un manager peut approuver/refuser les demandes de son équipe
- [ ] Les soldes sont mis à jour automatiquement après approbation
- [ ] Un admin peut configurer les types d'absences
- [ ] Un admin peut définir les jours fériés de l'organisation
- [ ] Le calendrier affiche les absences de l'équipe
- [ ] Une demande ne peut chevaucher une période déjà approuvée

---

## User Stories

---

### TM-41 : Types d'absences

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** configurer les types d'absences disponibles,
**Afin de** définir les catégories d'absences pour mon organisation.

#### Contexte Détaillé

Chaque organisation peut avoir ses propres types d'absences : congés payés, RTT, maladie, sans solde, congé parental, etc. Certains types impactent le solde, d'autres non (ex : maladie avec justificatif).

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/absence-types` créé
- [ ] Endpoint `POST /api/v1/absence-types` créé (admin)
- [ ] Champs : id, name, code, color, affects_balance, requires_justification
- [ ] Validation code unique dans l'organisation
- [ ] Types par défaut créés à l'inscription organisation
- [ ] Soft delete pour préserver l'historique

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-41.1 | Créer migration table absence_types | 0.5h |
| TM-41.2 | Créer modèle AbsenceType et repository | 1h |
| TM-41.3 | Seed types par défaut (CP, RTT, Maladie, Sans solde) | 0.5h |
| TM-41.4 | Créer endpoints CRUD | 1h |
| TM-41.5 | Tests d'intégration | 1h |

---

### TM-42 : Soldes de congés

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** gérer les soldes de congés des utilisateurs,
**Afin de** suivre les droits et consommations de chacun.

#### Contexte Détaillé

Chaque utilisateur a un solde par type d'absence qui l'impacte. Le solde comprend :
- `entitled` : droits acquis (ex : 25 jours CP/an)
- `taken` : jours pris
- `pending` : jours en attente de validation
- `remaining` : entitled - taken - pending

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/users/:id/leave-balances` créé
- [ ] Endpoint `PUT /api/v1/users/:id/leave-balances/:type_id` créé (admin)
- [ ] Calcul automatique du remaining
- [ ] Mise à jour automatique lors validation/annulation absence
- [ ] Historique des modifications de solde

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-42.1 | Créer migration table leave_balances | 0.5h |
| TM-42.2 | Créer modèle LeaveBalance et repository | 1h |
| TM-42.3 | Implémenter calcul automatique remaining | 1h |
| TM-42.4 | Créer endpoints consultation et modification | 1h |
| TM-42.5 | Implémenter mise à jour auto sur absence | 1h |
| TM-42.6 | Tests d'intégration | 1h |

---

### TM-43 : Jours fériés

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** configurer les jours fériés de mon organisation,
**Afin qu'** ils soient pris en compte dans les calculs de temps.

#### Contexte Détaillé

Les jours fériés sont propres à chaque organisation (différents selon les pays). Ils impactent :
- Le calcul des jours ouvrés dans une demande d'absence
- Le calcul des heures travaillées théoriques
- L'affichage dans le calendrier

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/holidays` créé
- [ ] Endpoint `POST /api/v1/holidays` créé (admin)
- [ ] Endpoint `DELETE /api/v1/holidays/:id` créé (admin)
- [ ] Champs : id, date, name, recurring (annuel ou ponctuel)
- [ ] Filtrage par année
- [ ] Utilisé dans le calcul des jours ouvrés

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-43.1 | Créer migration table holidays | 0.5h |
| TM-43.2 | Créer modèle Holiday et repository | 1h |
| TM-43.3 | Implémenter fonction is_holiday(date) | 0.5h |
| TM-43.4 | Créer endpoints CRUD | 1h |
| TM-43.5 | Tests d'intégration | 1h |

---

### TM-44 : Demande d'absence

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** employé,
**Je veux** soumettre une demande d'absence,
**Afin de** faire valider mes congés par mon manager.

#### Contexte Détaillé

Une demande d'absence contient :
- Type d'absence
- Date de début et date de fin
- Commentaire optionnel
- Justificatif optionnel (URL fichier ou upload)

Le nombre de jours ouvrés est calculé automatiquement (excluant weekends et jours fériés).

#### Critères d'Acceptation

- [ ] Endpoint `POST /api/v1/absences` créé
- [ ] Champs requis : absence_type_id, start_date, end_date
- [ ] Champs optionnels : comment, justification_url
- [ ] Validation : end_date >= start_date
- [ ] Validation : pas de chevauchement avec absence existante (PENDING ou APPROVED)
- [ ] Validation : solde suffisant si type affects_balance
- [ ] Calcul automatique du nombre de jours ouvrés
- [ ] Statut initial : PENDING
- [ ] Notification au manager

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-44.1 | Créer migration table absences | 0.5h |
| TM-44.2 | Créer modèle Absence et repository | 1h |
| TM-44.3 | Implémenter calcul jours ouvrés | 1h |
| TM-44.4 | Implémenter validations métier | 1h |
| TM-44.5 | Créer AbsenceService.create() | 1h |
| TM-44.6 | Créer endpoint POST /absences | 0.5h |
| TM-44.7 | Tests d'intégration | 1h |

---

### TM-45 : Liste des absences

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur connecté,
**Je veux** voir la liste des absences,
**Afin de** consulter mes demandes ou celles de mon équipe.

#### Contexte Détaillé

Les filtres et résultats dépendent du rôle :
- Employee : voit uniquement ses propres absences
- Manager : voit les absences de son équipe
- Admin : voit toutes les absences de l'organisation

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/absences` créé
- [ ] Filtres : user_id, status, type_id, date_range
- [ ] Retourne : id, user, type, dates, status, working_days, comment
- [ ] Scope automatique selon rôle
- [ ] Tri par date de début (desc)
- [ ] Pagination

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-45.1 | Implémenter AbsenceRepository.find_all() avec filtres | 1h |
| TM-45.2 | Implémenter scope par rôle | 1h |
| TM-45.3 | Créer endpoint GET /absences | 0.5h |
| TM-45.4 | Tests d'intégration par rôle | 1h |

---

### TM-46 : Détail d'une absence

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 1 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur concerné,
**Je veux** voir le détail d'une absence,
**Afin de** consulter toutes les informations associées.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/absences/:id` créé
- [ ] Retourne tous les champs + historique des changements de statut
- [ ] Accessible au propriétaire, son manager, ou admin
- [ ] Retour 404 si absence d'une autre organisation

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-46.1 | Créer endpoint GET /absences/:id | 0.5h |
| TM-46.2 | Implémenter vérification d'accès | 0.5h |
| TM-46.3 | Tests d'intégration | 0.5h |

---

### TM-47 : Approbation d'une absence

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant que** manager,
**Je veux** approuver une demande d'absence,
**Afin de** valider le congé d'un membre de mon équipe.

#### Contexte Détaillé

L'approbation :
- Change le statut de PENDING à APPROVED
- Met à jour le solde (pending → taken)
- Envoie une notification à l'employé
- Enregistre la date et l'auteur de l'approbation

#### Critères d'Acceptation

- [ ] Endpoint `POST /api/v1/absences/:id/approve` créé
- [ ] Body optionnel : { "comment": "..." }
- [ ] Validation : absence en statut PENDING uniquement
- [ ] Validation : manager de l'équipe ou admin
- [ ] Mise à jour automatique du solde
- [ ] Notification à l'employé
- [ ] Audit log

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-47.1 | Implémenter AbsenceService.approve() | 1.5h |
| TM-47.2 | Implémenter mise à jour solde | 1h |
| TM-47.3 | Créer endpoint POST /absences/:id/approve | 0.5h |
| TM-47.4 | Tests d'intégration | 1h |

---

### TM-48 : Refus d'une absence

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** manager,
**Je veux** refuser une demande d'absence,
**Afin de** ne pas valider un congé non acceptable.

#### Contexte Détaillé

Le refus :
- Change le statut de PENDING à REJECTED
- Libère les jours "pending" du solde
- Envoie une notification à l'employé avec le motif
- Le motif de refus est obligatoire

#### Critères d'Acceptation

- [ ] Endpoint `POST /api/v1/absences/:id/reject` créé
- [ ] Body requis : { "reason": "..." }
- [ ] Validation : absence en statut PENDING uniquement
- [ ] Validation : manager de l'équipe ou admin
- [ ] Libération du solde pending
- [ ] Notification à l'employé avec motif
- [ ] Audit log

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-48.1 | Implémenter AbsenceService.reject() | 1h |
| TM-48.2 | Implémenter libération solde pending | 0.5h |
| TM-48.3 | Créer endpoint POST /absences/:id/reject | 0.5h |
| TM-48.4 | Tests d'intégration | 1h |

---

### TM-49 : Annulation d'une absence

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** employé,
**Je veux** annuler ma demande d'absence,
**Afin de** revenir sur ma demande si mes plans changent.

#### Contexte Détaillé

L'annulation est possible :
- Si PENDING : annulation directe
- Si APPROVED : annulation possible uniquement si start_date est dans le futur

L'annulation libère le solde (pending ou taken).

#### Critères d'Acceptation

- [ ] Endpoint `POST /api/v1/absences/:id/cancel` créé
- [ ] Validation : propriétaire de la demande uniquement
- [ ] Si PENDING : annulation immédiate
- [ ] Si APPROVED : uniquement si start_date > today
- [ ] Libération du solde
- [ ] Notification au manager
- [ ] Statut final : CANCELLED

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-49.1 | Implémenter AbsenceService.cancel() | 1h |
| TM-49.2 | Implémenter règles d'annulation | 0.5h |
| TM-49.3 | Créer endpoint POST /absences/:id/cancel | 0.5h |
| TM-49.4 | Tests d'intégration | 1h |

---

### TM-50 : Absences en attente (manager)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 1 SP |
| **Assigné** | - |

#### Description

**En tant que** manager,
**Je veux** voir les demandes d'absence en attente de mon équipe,
**Afin de** traiter rapidement les validations.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/absences/pending` créé
- [ ] Retourne uniquement les absences PENDING de l'équipe du manager
- [ ] Tri par date de création (FIFO)
- [ ] Badge count pour le menu

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-50.1 | Créer endpoint GET /absences/pending | 0.5h |
| TM-50.2 | Implémenter filtre équipe manager | 0.5h |
| TM-50.3 | Tests d'intégration | 0.5h |

---

### TM-51 : Formulaire demande d'absence (Frontend)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** employé,
**Je veux** un formulaire pour demander une absence,
**Afin de** soumettre ma demande facilement.

#### Critères d'Acceptation

- [ ] Modal ou page `/absences/new` créée
- [ ] Sélecteur type d'absence avec couleurs
- [ ] Date picker pour début et fin
- [ ] Affichage automatique du nombre de jours ouvrés
- [ ] Affichage du solde disponible pour le type sélectionné
- [ ] Champ commentaire optionnel
- [ ] Validation temps réel (chevauchement, solde)
- [ ] Bouton soumettre avec loading state
- [ ] Message de succès et redirection

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-51.1 | Créer hook useAbsences (queries + mutations) | 1h |
| TM-51.2 | Créer composant AbsenceTypeSelector | 1h |
| TM-51.3 | Créer composant DateRangePicker | 1h |
| TM-51.4 | Créer composant WorkingDaysPreview | 0.5h |
| TM-51.5 | Créer formulaire AbsenceRequestForm | 2h |
| TM-51.6 | Implémenter validations temps réel | 1h |
| TM-51.7 | Tests composants | 1h |

---

### TM-52 : Liste mes absences (Frontend)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** employé,
**Je veux** voir la liste de mes demandes d'absence,
**Afin de** suivre leur statut et historique.

#### Critères d'Acceptation

- [ ] Page `/absences` créée
- [ ] Tableau avec colonnes : type, dates, jours, statut, actions
- [ ] Filtres par statut et type
- [ ] Badge couleur par statut (pending=jaune, approved=vert, rejected=rouge)
- [ ] Action "Annuler" sur PENDING et APPROVED futurs
- [ ] Bouton "Nouvelle demande"
- [ ] Vue mobile en cards

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-52.1 | Créer composant AbsenceStatusBadge | 0.5h |
| TM-52.2 | Créer composant AbsenceCard | 1h |
| TM-52.3 | Créer page AbsencesListPage | 2h |
| TM-52.4 | Implémenter filtres et tri | 1h |
| TM-52.5 | Tests composants | 1h |

---

### TM-53 : Calendrier absences équipe (Frontend)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 5 SP |
| **Assigné** | - |

#### Description

**En tant que** manager,
**Je veux** voir un calendrier des absences de mon équipe,
**Afin de** visualiser les disponibilités et planifier.

#### Critères d'Acceptation

- [ ] Composant calendrier mensuel créé
- [ ] Affichage des absences approuvées par membre
- [ ] Code couleur par type d'absence
- [ ] Jours fériés marqués
- [ ] Navigation mois précédent/suivant
- [ ] Clic sur absence → détail
- [ ] Export optionnel (CSV/PDF)
- [ ] Vue semaine disponible

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-53.1 | Créer composant CalendarGrid | 2h |
| TM-53.2 | Créer composant AbsenceBar (barre horizontale) | 1h |
| TM-53.3 | Implémenter navigation temporelle | 1h |
| TM-53.4 | Intégrer jours fériés | 0.5h |
| TM-53.5 | Créer page TeamCalendarPage | 2h |
| TM-53.6 | Implémenter export CSV | 1h |
| TM-53.7 | Tests composants | 1h |

---

### TM-54 : Page validation absences (Frontend)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant que** manager,
**Je veux** une page pour valider les demandes d'absence,
**Afin de** traiter efficacement les demandes de mon équipe.

#### Critères d'Acceptation

- [ ] Page `/absences/pending` créée
- [ ] Liste des demandes en attente de mon équipe
- [ ] Pour chaque demande : employé, type, dates, jours, commentaire
- [ ] Boutons "Approuver" et "Refuser"
- [ ] Modal motif obligatoire pour refus
- [ ] Mise à jour temps réel de la liste
- [ ] Badge dans le menu latéral avec count

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-54.1 | Créer composant PendingAbsenceCard | 1h |
| TM-54.2 | Créer modal RejectReasonModal | 1h |
| TM-54.3 | Créer page PendingAbsencesPage | 2h |
| TM-54.4 | Implémenter badge count menu | 0.5h |
| TM-54.5 | Tests composants | 1h |

---

### TM-55 : Gestion soldes utilisateur (Frontend)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** gérer les soldes de congés des utilisateurs,
**Afin de** initialiser ou ajuster leurs droits.

#### Critères d'Acceptation

- [ ] Section dans le détail utilisateur
- [ ] Tableau des soldes par type : acquis, pris, en attente, restant
- [ ] Bouton "Modifier" → modal édition
- [ ] Historique des modifications visible
- [ ] Validation : acquis >= 0

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-55.1 | Créer composant LeaveBalancesTable | 1h |
| TM-55.2 | Créer modal EditLeaveBalanceModal | 1h |
| TM-55.3 | Intégrer dans UserDetailPage | 0.5h |
| TM-55.4 | Tests composants | 0.5h |

---

## Récapitulatif des Estimations

| Story | Titre | SP |
|-------|-------|:--:|
| TM-41 | Types d'absences | 2 |
| TM-42 | Soldes de congés | 3 |
| TM-43 | Jours fériés | 2 |
| TM-44 | Demande d'absence | 3 |
| TM-45 | Liste des absences | 2 |
| TM-46 | Détail d'une absence | 1 |
| TM-47 | Approbation d'une absence | 3 |
| TM-48 | Refus d'une absence | 2 |
| TM-49 | Annulation d'une absence | 2 |
| TM-50 | Absences en attente (manager) | 1 |
| TM-51 | Formulaire demande d'absence (Frontend) | 3 |
| TM-52 | Liste mes absences (Frontend) | 3 |
| TM-53 | Calendrier absences équipe (Frontend) | 5 |
| TM-54 | Page validation absences (Frontend) | 3 |
| TM-55 | Gestion soldes utilisateur (Frontend) | 2 |
| **Total** | | **37 SP** |

---

## Notes Techniques

### Modèle Absence

```rust
pub struct Absence {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub absence_type_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub working_days: i32,
    pub status: AbsenceStatus,
    pub comment: Option<String>,
    pub justification_url: Option<String>,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub review_comment: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub enum AbsenceStatus {
    Pending,
    Approved,
    Rejected,
    Cancelled,
}
```

### Machine à États Absence

```
PENDING → APPROVED (par manager)
PENDING → REJECTED (par manager)
PENDING → CANCELLED (par employé)
APPROVED → CANCELLED (par employé, si futur)
```

### Calcul Jours Ouvrés

```rust
fn calculate_working_days(start: NaiveDate, end: NaiveDate, holidays: &[Holiday]) -> i32 {
    let mut count = 0;
    let mut current = start;
    while current <= end {
        if !is_weekend(current) && !is_holiday(current, holidays) {
            count += 1;
        }
        current = current.succ();
    }
    count
}
```

### Endpoints Récapitulatif

| Méthode | Endpoint | Permission |
|---------|----------|------------|
| GET | /api/v1/absence-types | All |
| POST | /api/v1/absence-types | Admin |
| GET | /api/v1/holidays | All |
| POST | /api/v1/holidays | Admin |
| DELETE | /api/v1/holidays/:id | Admin |
| GET | /api/v1/absences | Scoped |
| POST | /api/v1/absences | All |
| GET | /api/v1/absences/:id | Owner/Manager/Admin |
| POST | /api/v1/absences/:id/approve | Manager/Admin |
| POST | /api/v1/absences/:id/reject | Manager/Admin |
| POST | /api/v1/absences/:id/cancel | Owner |
| GET | /api/v1/absences/pending | Manager/Admin |
| GET | /api/v1/users/:id/leave-balances | Owner/Admin |
| PUT | /api/v1/users/:id/leave-balances/:type_id | Admin |

### Règles Métier

| Règle | Description |
|-------|-------------|
| Pas de chevauchement | Une absence ne peut chevaucher une existante (PENDING/APPROVED) |
| Solde suffisant | Si type affects_balance, solde remaining >= working_days |
| Annulation future | APPROVED ne peut être annulé que si start_date > today |
| Motif refus obligatoire | Le refus nécessite un motif |
| Jours ouvrés auto | Le calcul exclut weekends et jours fériés |
