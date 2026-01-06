# TM-E06 : Horaires Configurables

## Informations Epic

| Champ | Valeur |
|-------|--------|
| **ID** | TM-E06 |
| **Titre** | Horaires Configurables |
| **Priorité** | P1 - Critique |
| **Estimation globale** | 18 SP |
| **Sprint cible** | Sprint 4 |
| **Dépendances** | TM-E05 (Équipes) |

---

## Description

### Contexte

Les horaires de travail varient selon les organisations, les équipes et même les individus. Time Manager doit permettre de définir des modèles d'horaires (35h, 39h, temps partiel, etc.) et de les assigner à différents niveaux : organisation (défaut), équipe, ou utilisateur individuel.

### Objectif Business

Permettre une gestion flexible des horaires de travail avec des modèles réutilisables, tout en supportant les exceptions individuelles. Ces horaires servent de base pour le calcul des KPIs (heures théoriques vs réelles).

### Valeur Apportée

- **Pour les admins** : Configuration centralisée des horaires types
- **Pour les RH** : Calcul automatique des heures théoriques
- **Pour les employés** : Visibilité sur leurs horaires attendus
- **Pour le système** : Base de calcul pour les KPIs de ponctualité et écart horaire

---

## Scope

### Inclus

- CRUD des modèles d'horaires (work_schedules)
- Configuration par jour de la semaine (work_schedule_days)
- Assignation à une équipe ou un utilisateur
- Résolution de l'horaire effectif (user > team > org default)
- Consultation de son horaire personnel
- Modèles prédéfinis (35h, 39h, temps partiel)
- Tolérance de ponctualité configurable

### Exclus

- Horaires variables par semaine/mois
- Planification d'horaires futurs avec dates
- Cycles de rotation (3x8, etc.)
- Heures supplémentaires majorées (calcul)

---

## Critères de Succès de l'Epic

- [ ] Un admin peut créer des modèles d'horaires réutilisables
- [ ] Un admin peut configurer les horaires jour par jour
- [ ] Un admin peut assigner un horaire à une équipe
- [ ] Un admin peut assigner un horaire spécifique à un utilisateur
- [ ] Chaque utilisateur voit ses horaires effectifs
- [ ] Le système résout correctement : user > team > org default
- [ ] Les modèles prédéfinis sont créés au setup de l'organisation

---

## User Stories

---

### TM-41 : Liste des modèles d'horaires

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur ou manager,
**Je veux** voir la liste des modèles d'horaires disponibles,
**Afin de** connaître les options pour mon organisation.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/schedules` créé
- [ ] Retourne : id, name, weekly_hours, tolerance_minutes, is_default
- [ ] Accessible aux ADMIN et MANAGER
- [ ] Tri par nom
- [ ] Indication du modèle par défaut de l'organisation

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-41.1 | Créer migrations work_schedules et work_schedule_days | 1h |
| TM-41.2 | Créer modèles et repository | 1h |
| TM-41.3 | Créer ScheduleService.list() | 0.5h |
| TM-41.4 | Créer endpoint GET /schedules | 0.5h |
| TM-41.5 | Tests d'intégration | 1h |

---

### TM-42 : Création d'un modèle d'horaire

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** créer un modèle d'horaire personnalisé,
**Afin de** l'adapter aux besoins de mon organisation.

#### Contexte Détaillé

Un modèle d'horaire comprend :
- Un nom (ex: "35 heures standard")
- Les heures théoriques par semaine (calculé automatiquement)
- Une tolérance de ponctualité (ex: 10 minutes)
- La configuration de chaque jour de la semaine

#### Critères d'Acceptation

- [ ] Endpoint `POST /api/v1/schedules` créé
- [ ] Champs requis :
  - name (unique dans l'org)
  - tolerance_minutes (défaut: 10)
  - days : tableau de 7 jours avec start_time, end_time, break_minutes
- [ ] Calcul automatique de weekly_hours
- [ ] Jours non travaillés : start_time et end_time null
- [ ] Validation : end_time > start_time
- [ ] Validation : break_minutes < durée journée
- [ ] Réservé aux ADMIN

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-42.1 | Créer DTO CreateScheduleRequest avec validation | 1h |
| TM-42.2 | Implémenter calcul weekly_hours | 0.5h |
| TM-42.3 | Implémenter ScheduleService.create() | 1.5h |
| TM-42.4 | Créer endpoint POST /schedules | 0.5h |
| TM-42.5 | Tests d'intégration | 1h |

---

### TM-43 : Consultation détaillée d'un horaire

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 1 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur,
**Je veux** voir le détail d'un modèle d'horaire,
**Afin de** connaître les heures de chaque jour.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/schedules/:id` créé
- [ ] Retourne : infos du modèle + tableau des 7 jours
- [ ] Pour chaque jour : day_of_week, start_time, end_time, break_minutes, worked_hours
- [ ] Accessible à tous les rôles (lecture)
- [ ] Retour 404 si horaire d'une autre organisation

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-43.1 | Créer ScheduleRepository.find_with_days() | 0.5h |
| TM-43.2 | Créer endpoint GET /schedules/:id | 0.5h |
| TM-43.3 | Créer DTO ScheduleDetailResponse | 0.5h |
| TM-43.4 | Tests d'intégration | 0.5h |

---

### TM-44 : Modification d'un modèle d'horaire

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** modifier un modèle d'horaire existant,
**Afin de** l'adapter aux évolutions.

#### Critères d'Acceptation

- [ ] Endpoint `PUT /api/v1/schedules/:id` créé
- [ ] Champs modifiables : name, tolerance_minutes, days
- [ ] Recalcul de weekly_hours si days modifié
- [ ] Mêmes validations que la création
- [ ] Réservé aux ADMIN
- [ ] Ne peut pas modifier les modèles prédéfinis système (optionnel)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-44.1 | Créer DTO UpdateScheduleRequest | 0.5h |
| TM-44.2 | Implémenter ScheduleService.update() | 1h |
| TM-44.3 | Créer endpoint PUT /schedules/:id | 0.5h |
| TM-44.4 | Tests d'intégration | 1h |

---

### TM-45 : Suppression d'un modèle d'horaire

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 1 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** supprimer un modèle d'horaire inutilisé,
**Afin de** garder une liste propre.

#### Critères d'Acceptation

- [ ] Endpoint `DELETE /api/v1/schedules/:id` créé
- [ ] Impossible de supprimer le modèle par défaut de l'org
- [ ] Impossible de supprimer si assigné à des équipes/users
- [ ] Ou : désassigne automatiquement et supprime
- [ ] Réservé aux ADMIN
- [ ] Retour 409 Conflict si en cours d'utilisation

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-45.1 | Vérifier les assignations existantes | 0.5h |
| TM-45.2 | Implémenter ScheduleService.delete() | 0.5h |
| TM-45.3 | Créer endpoint DELETE /schedules/:id | 0.5h |
| TM-45.4 | Tests d'intégration | 0.5h |

---

### TM-46 : Assignation d'horaire à une équipe

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** assigner un modèle d'horaire à une équipe,
**Afin que** tous ses membres aient ces horaires par défaut.

#### Contexte Détaillé

L'assignation à une équipe affecte tous les membres qui n'ont pas d'horaire individuel défini. C'est le niveau intermédiaire de la hiérarchie de résolution.

#### Critères d'Acceptation

- [ ] Endpoint `POST /api/v1/schedules/:id/assign-team` créé
- [ ] Body : `{ "team_id": "uuid" }`
- [ ] Vérifie que l'équipe appartient à l'organisation
- [ ] Met à jour team.work_schedule_id
- [ ] Réservé aux ADMIN
- [ ] Retour : équipe mise à jour

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-46.1 | Ajouter work_schedule_id à la table teams | 0.5h |
| TM-46.2 | Créer DTO AssignScheduleRequest | 0.5h |
| TM-46.3 | Implémenter ScheduleService.assign_to_team() | 1h |
| TM-46.4 | Créer endpoint POST /schedules/:id/assign-team | 0.5h |
| TM-46.5 | Tests d'intégration | 1h |

---

### TM-47 : Assignation d'horaire à un utilisateur

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur ou manager,
**Je veux** assigner un horaire spécifique à un utilisateur,
**Afin de** gérer les cas particuliers (temps partiel, horaires décalés).

#### Contexte Détaillé

L'horaire individuel a la plus haute priorité. Il écrase l'horaire de l'équipe et celui de l'organisation.

#### Critères d'Acceptation

- [ ] Endpoint `POST /api/v1/schedules/:id/assign-user` créé
- [ ] Body : `{ "user_id": "uuid" }`
- [ ] Vérifie que l'utilisateur appartient à l'organisation
- [ ] Met à jour user.work_schedule_id
- [ ] ADMIN : peut assigner à tout utilisateur de l'org
- [ ] MANAGER : peut assigner uniquement aux membres de son équipe
- [ ] Retour : utilisateur mis à jour

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-47.1 | Vérifier que work_schedule_id existe sur users | 0.5h |
| TM-47.2 | Implémenter ScheduleService.assign_to_user() | 1h |
| TM-47.3 | Implémenter autorisation manager | 0.5h |
| TM-47.4 | Créer endpoint POST /schedules/:id/assign-user | 0.5h |
| TM-47.5 | Tests d'intégration | 1h |

---

### TM-48 : Résolution de l'horaire effectif

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** utilisateur,
**Je veux** voir mes horaires effectifs,
**Afin de** connaître mes heures de travail attendues.

#### Contexte Détaillé

La résolution suit la hiérarchie :
1. **User** : Si user.work_schedule_id défini → utiliser cet horaire
2. **Team** : Sinon, si team.work_schedule_id défini → utiliser cet horaire
3. **Org Default** : Sinon, utiliser l'horaire par défaut de l'organisation

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/users/:id/schedule` créé
- [ ] Résout l'horaire selon la hiérarchie user > team > org
- [ ] Retourne le modèle d'horaire complet avec les jours
- [ ] Indique la source : "user", "team", ou "organization"
- [ ] Self : peut voir son propre horaire
- [ ] Manager : peut voir les horaires de son équipe
- [ ] Admin : peut voir tous les horaires
- [ ] Endpoint `/api/v1/auth/me/schedule` pour son propre horaire

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-48.1 | Implémenter ScheduleService.resolve_for_user() | 1.5h |
| TM-48.2 | Créer endpoint GET /users/:id/schedule | 0.5h |
| TM-48.3 | Créer endpoint GET /auth/me/schedule | 0.5h |
| TM-48.4 | Créer DTO ResolvedScheduleResponse | 0.5h |
| TM-48.5 | Tests d'intégration (tous les cas) | 1h |

---

### TM-49 : Modèles d'horaires prédéfinis

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 1 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur d'une nouvelle organisation,
**Je veux** avoir des modèles d'horaires prédéfinis,
**Afin de** démarrer rapidement sans tout configurer.

#### Contexte Détaillé

Au setup d'une organisation, créer automatiquement :
- **35h Standard** : 7h/jour, lun-ven, 9h-12h + 14h-18h, 1h pause
- **39h Standard** : 7h48/jour, lun-ven, 9h-12h + 13h-17h48, 1h pause
- **Temps Partiel 50%** : 4h/jour, lun-ven, 9h-13h, 0 pause

Le modèle 35h est défini comme horaire par défaut de l'organisation.

#### Critères d'Acceptation

- [ ] Seed/migration crée les 3 modèles au register d'une organisation
- [ ] Le modèle 35h est marqué is_default = true
- [ ] Les modèles sont liés à l'organization_id
- [ ] Les modèles peuvent être modifiés par l'admin
- [ ] Organisation.default_schedule_id pointe vers le 35h

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-49.1 | Créer fonction seed_default_schedules() | 1h |
| TM-49.2 | Appeler dans AuthService.register() | 0.5h |
| TM-49.3 | Ajouter default_schedule_id à organizations | 0.5h |
| TM-49.4 | Tests d'intégration | 0.5h |

---

### TM-50 : Page gestion des horaires (Frontend)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 5 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** une interface pour gérer les modèles d'horaires,
**Afin de** configurer les temps de travail visuellement.

#### Critères d'Acceptation

- [ ] Page `/settings/schedules` créée
- [ ] Liste des modèles avec :
  - Nom
  - Heures hebdomadaires
  - Tolérance
  - Badge "Défaut" si is_default
  - Actions (voir, modifier, supprimer)
- [ ] Modal création avec éditeur de semaine :
  - 7 jours en colonnes ou lignes
  - Pour chaque jour : checkbox "travaillé", heure début, heure fin, pause
  - Calcul temps réel des heures totales
- [ ] Modal modification similaire
- [ ] Section "Assignations" :
  - Liste des équipes avec leur horaire assigné
  - Dropdown pour changer l'horaire d'une équipe
- [ ] Responsive

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-50.1 | Créer hook useSchedules | 1h |
| TM-50.2 | Créer composant WeekScheduleEditor | 3h |
| TM-50.3 | Créer composant DayScheduleInput | 1h |
| TM-50.4 | Créer modal CreateScheduleModal | 1h |
| TM-50.5 | Créer modal EditScheduleModal | 1h |
| TM-50.6 | Créer composant TeamScheduleAssignment | 1h |
| TM-50.7 | Créer page SchedulesSettingsPage | 1h |
| TM-50.8 | Styling responsive | 1h |
| TM-50.9 | Tests composants | 1h |

---

## Récapitulatif des Estimations

| Story | Titre | SP |
|-------|-------|:--:|
| TM-41 | Liste des modèles d'horaires | 2 |
| TM-42 | Création d'un modèle d'horaire | 3 |
| TM-43 | Consultation détaillée d'un horaire | 1 |
| TM-44 | Modification d'un modèle d'horaire | 2 |
| TM-45 | Suppression d'un modèle d'horaire | 1 |
| TM-46 | Assignation d'horaire à une équipe | 2 |
| TM-47 | Assignation d'horaire à un utilisateur | 2 |
| TM-48 | Résolution de l'horaire effectif | 2 |
| TM-49 | Modèles d'horaires prédéfinis | 1 |
| TM-50 | Page gestion des horaires (Frontend) | 5 |
| **Total** | | **21 SP** |

---

## Notes Techniques

### Modèles

```rust
pub struct WorkSchedule {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub weekly_hours: f32,        // Calculé automatiquement
    pub tolerance_minutes: i32,   // Tolérance ponctualité
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct WorkScheduleDay {
    pub id: Uuid,
    pub schedule_id: Uuid,
    pub day_of_week: i32,         // 0 = Lundi, 6 = Dimanche
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub break_minutes: i32,
}
```

### Calcul des Heures

```rust
fn calculate_daily_hours(day: &WorkScheduleDay) -> f32 {
    match (day.start_time, day.end_time) {
        (Some(start), Some(end)) => {
            let minutes = (end - start).num_minutes() as f32;
            (minutes - day.break_minutes as f32) / 60.0
        }
        _ => 0.0
    }
}

fn calculate_weekly_hours(days: &[WorkScheduleDay]) -> f32 {
    days.iter().map(calculate_daily_hours).sum()
}
```

### Hiérarchie de Résolution

```
User.work_schedule_id      (priorité haute)
        ↓ si null
Team.work_schedule_id      (priorité moyenne)
        ↓ si null
Organization.default_schedule_id  (priorité basse)
```

### Endpoints Récapitulatif

| Méthode | Endpoint | Permission |
|---------|----------|------------|
| GET | /api/v1/schedules | Admin, Manager |
| POST | /api/v1/schedules | Admin |
| GET | /api/v1/schedules/:id | All |
| PUT | /api/v1/schedules/:id | Admin |
| DELETE | /api/v1/schedules/:id | Admin |
| POST | /api/v1/schedules/:id/assign-team | Admin |
| POST | /api/v1/schedules/:id/assign-user | Admin, Manager* |
| GET | /api/v1/users/:id/schedule | Self, Admin, Manager* |
| GET | /api/v1/auth/me/schedule | All |

*Manager = uniquement son équipe
