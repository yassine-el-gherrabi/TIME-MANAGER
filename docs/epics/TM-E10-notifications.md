# TM-E10 : Notifications

## Informations Epic

| Champ | Valeur |
|-------|--------|
| **ID** | TM-E10 |
| **Titre** | Notifications |
| **Priorité** | P2 - Moyenne |
| **Estimation globale** | 13 SP |
| **Sprint cible** | Sprint 5-6 |
| **Dépendances** | TM-E07 (Pointage), TM-E08 (Absences) |

---

## Description

### Contexte

Les notifications permettent de tenir les utilisateurs informés des événements importants : demandes à valider, absences approuvées/refusées, corrections de pointage, etc. Elles améliorent la réactivité et la fluidité des workflows de validation.

### Objectif Business

Assurer que les actions nécessitant une réponse (validations, corrections) sont traitées rapidement grâce à des notifications en temps réel ou quasi temps réel, réduisant ainsi les délais de traitement.

### Valeur Apportée

- **Pour les employés** : Information immédiate sur le statut de leurs demandes
- **Pour les managers** : Alertes sur les demandes en attente de leur équipe
- **Pour tous** : Réduction des délais de traitement et meilleure communication

---

## Scope

### Inclus

- Notifications in-app (centre de notifications)
- Marquage lu/non-lu
- Types de notifications (absence, pointage, système)
- Badge de notifications non lues
- Préférences de notifications utilisateur

### Exclus

- Notifications push navigateur (Web Push API)
- Notifications email
- Notifications SMS
- Notifications temps réel (WebSockets) - polling simple
- Notifications groupées/digests

---

## Critères de Succès de l'Epic

- [ ] Les notifications sont créées automatiquement lors des événements clés
- [ ] Un utilisateur voit ses notifications non lues
- [ ] Le badge affiche le nombre de notifications non lues
- [ ] Un utilisateur peut marquer une notification comme lue
- [ ] Les préférences permettent de désactiver certains types

---

## User Stories

---

### TM-65 : Modèle de notification

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur backend,
**Je veux** un modèle de notifications,
**Afin de** stocker et gérer les notifications utilisateur.

#### Contexte Détaillé

Une notification contient :
- Le destinataire (user_id)
- Le type (absence_approved, correction_requested, etc.)
- Le titre et le message
- Un lien optionnel vers la ressource concernée
- Le statut lu/non-lu
- La date de création

#### Critères d'Acceptation

- [ ] Migration table notifications créée
- [ ] Modèle Notification avec tous les champs
- [ ] Enum NotificationType avec les types définis
- [ ] Index sur user_id et read_at pour performance
- [ ] Repository avec méthodes de base

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-65.1 | Créer migration table notifications | 0.5h |
| TM-65.2 | Créer enum NotificationType | 0.5h |
| TM-65.3 | Créer modèle Notification | 1h |
| TM-65.4 | Créer NotificationRepository | 1h |
| TM-65.5 | Tests unitaires | 1h |

---

### TM-66 : Service de notification

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur backend,
**Je veux** un service centralisé de notifications,
**Afin de** créer des notifications depuis n'importe quel module.

#### Contexte Détaillé

Le NotificationService expose des méthodes comme :
- `notify_absence_approved(absence, reviewer)`
- `notify_absence_rejected(absence, reviewer, reason)`
- `notify_correction_requested(clock_entry, requester)`
- etc.

Il est injecté dans les autres services qui l'appellent aux moments appropriés.

#### Critères d'Acceptation

- [ ] NotificationService créé
- [ ] Méthodes pour chaque type d'événement
- [ ] Templates de messages par type
- [ ] Intégration dans AbsenceService
- [ ] Intégration dans ClockService
- [ ] Tests unitaires

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-66.1 | Créer NotificationService | 1h |
| TM-66.2 | Définir templates de messages | 1h |
| TM-66.3 | Intégrer dans AbsenceService | 0.5h |
| TM-66.4 | Intégrer dans ClockService | 0.5h |
| TM-66.5 | Tests unitaires | 1h |

---

### TM-67 : Liste des notifications

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 1 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur connecté,
**Je veux** voir la liste de mes notifications,
**Afin de** consulter les événements me concernant.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/notifications` créé
- [ ] Retourne les notifications de l'utilisateur courant
- [ ] Tri par date (plus récentes en premier)
- [ ] Filtres optionnels : read (true/false), type
- [ ] Pagination (limit/offset)
- [ ] Inclut le count total non lues

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-67.1 | Implémenter NotificationRepository.find_by_user() | 0.5h |
| TM-67.2 | Créer endpoint GET /notifications | 0.5h |
| TM-67.3 | Tests d'intégration | 0.5h |

---

### TM-68 : Marquer comme lu

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 1 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur connecté,
**Je veux** marquer une notification comme lue,
**Afin de** gérer mes notifications.

#### Critères d'Acceptation

- [ ] Endpoint `PUT /api/v1/notifications/:id/read` créé
- [ ] Endpoint `PUT /api/v1/notifications/read-all` créé
- [ ] Validation : notification appartient à l'utilisateur
- [ ] Mise à jour du champ read_at
- [ ] Idempotent (relire une notification déjà lue = OK)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-68.1 | Implémenter mark_as_read() | 0.5h |
| TM-68.2 | Implémenter mark_all_as_read() | 0.5h |
| TM-68.3 | Créer endpoints | 0.5h |
| TM-68.4 | Tests d'intégration | 0.5h |

---

### TM-69 : Compteur notifications non lues

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 1 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur connecté,
**Je veux** connaître le nombre de notifications non lues,
**Afin d'** être alerté des nouveaux événements.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/notifications/unread-count` créé
- [ ] Retourne : { count: number }
- [ ] Performance optimisée (COUNT SQL, pas de fetch)
- [ ] Utilisé pour le badge dans le header

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-69.1 | Implémenter count_unread() | 0.5h |
| TM-69.2 | Créer endpoint GET /notifications/unread-count | 0.5h |
| TM-69.3 | Tests d'intégration | 0.5h |

---

### TM-70 : Préférences de notifications

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P2 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur connecté,
**Je veux** configurer mes préférences de notifications,
**Afin de** choisir quels types de notifications recevoir.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/users/me/notification-preferences` créé
- [ ] Endpoint `PUT /api/v1/users/me/notification-preferences` créé
- [ ] Préférences par type : absence_status, correction_status, team_requests
- [ ] Valeurs : enabled (true/false)
- [ ] Par défaut : tout activé
- [ ] Le service vérifie les préférences avant de créer

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-70.1 | Ajouter colonne notification_preferences (JSONB) | 0.5h |
| TM-70.2 | Créer endpoints get/update preferences | 1h |
| TM-70.3 | Modifier NotificationService pour vérifier | 1h |
| TM-70.4 | Tests d'intégration | 0.5h |

---

### TM-71 : Centre de notifications (Frontend)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur connecté,
**Je veux** un centre de notifications dans l'interface,
**Afin de** consulter et gérer mes notifications.

#### Critères d'Acceptation

- [ ] Icône cloche dans le header avec badge count
- [ ] Dropdown au clic avec liste des notifications récentes
- [ ] Distinction visuelle lu/non-lu
- [ ] Clic sur notification → marque lu + navigation vers ressource
- [ ] Bouton "Tout marquer comme lu"
- [ ] Lien vers page complète si > 5 notifications
- [ ] Polling toutes les 30 secondes pour le count

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-71.1 | Créer hook useNotifications | 1h |
| TM-71.2 | Créer composant NotificationBell | 1h |
| TM-71.3 | Créer composant NotificationDropdown | 1.5h |
| TM-71.4 | Créer composant NotificationItem | 1h |
| TM-71.5 | Implémenter polling | 0.5h |
| TM-71.6 | Intégrer dans Header | 0.5h |
| TM-71.7 | Tests composants | 1h |

---

### TM-72 : Page préférences notifications (Frontend)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P2 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur connecté,
**Je veux** une page pour configurer mes préférences de notifications,
**Afin de** personnaliser les alertes que je reçois.

#### Critères d'Acceptation

- [ ] Page `/settings/notifications` créée
- [ ] Liste des types de notifications avec toggle on/off
- [ ] Description claire de chaque type
- [ ] Sauvegarde automatique ou bouton "Enregistrer"
- [ ] Message de confirmation après modification

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-72.1 | Créer hook useNotificationPreferences | 0.5h |
| TM-72.2 | Créer composant NotificationPreferenceToggle | 1h |
| TM-72.3 | Créer page NotificationPreferencesPage | 1.5h |
| TM-72.4 | Tests composants | 0.5h |

---

## Récapitulatif des Estimations

| Story | Titre | SP |
|-------|-------|:--:|
| TM-65 | Modèle de notification | 2 |
| TM-66 | Service de notification | 2 |
| TM-67 | Liste des notifications | 1 |
| TM-68 | Marquer comme lu | 1 |
| TM-69 | Compteur notifications non lues | 1 |
| TM-70 | Préférences de notifications | 2 |
| TM-71 | Centre de notifications (Frontend) | 3 |
| TM-72 | Page préférences notifications (Frontend) | 2 |
| **Total** | | **14 SP** |

---

## Notes Techniques

### Modèle Notification

```rust
pub struct Notification {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub link: Option<String>,
    pub read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

pub enum NotificationType {
    AbsenceApproved,
    AbsenceRejected,
    AbsenceCancelled,
    CorrectionRequested,
    CorrectionApproved,
    CorrectionRejected,
    TeamAbsenceRequest,
    TeamCorrectionRequest,
    SystemAnnouncement,
}
```

### Templates de Messages

| Type | Titre | Message |
|------|-------|---------|
| AbsenceApproved | Absence approuvée | Votre demande du {date} au {date} a été approuvée par {manager} |
| AbsenceRejected | Absence refusée | Votre demande du {date} au {date} a été refusée : {reason} |
| CorrectionRequested | Correction demandée | {user} demande une correction pour le {date} |
| TeamAbsenceRequest | Nouvelle demande d'absence | {user} a soumis une demande d'absence du {date} au {date} |

### Endpoints Récapitulatif

| Méthode | Endpoint | Permission |
|---------|----------|------------|
| GET | /api/v1/notifications | Owner |
| GET | /api/v1/notifications/unread-count | Owner |
| PUT | /api/v1/notifications/:id/read | Owner |
| PUT | /api/v1/notifications/read-all | Owner |
| GET | /api/v1/users/me/notification-preferences | Owner |
| PUT | /api/v1/users/me/notification-preferences | Owner |

### Événements Déclencheurs

| Événement | Destinataire | Type |
|-----------|--------------|------|
| Absence créée | Manager(s) équipe | TeamAbsenceRequest |
| Absence approuvée | Demandeur | AbsenceApproved |
| Absence refusée | Demandeur | AbsenceRejected |
| Correction demandée | Manager(s) équipe | TeamCorrectionRequest |
| Correction approuvée | Demandeur | CorrectionApproved |
| Correction refusée | Demandeur | CorrectionRejected |

### Optimisations

- Index sur (user_id, read_at) pour les requêtes fréquentes
- Pagination côté serveur pour éviter de charger toutes les notifications
- Polling léger (count uniquement) plutôt que WebSockets pour MVP
- Nettoyage périodique des notifications > 90 jours
