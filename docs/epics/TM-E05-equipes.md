# TM-E05 : Gestion des Équipes

## Informations Epic

| Champ | Valeur |
|-------|--------|
| **ID** | TM-E05 |
| **Titre** | Gestion des Équipes |
| **Priorité** | P1 - Critique |
| **Estimation globale** | 13 SP |
| **Sprint cible** | Sprint 3 |
| **Dépendances** | TM-E04 (Utilisateurs) |

---

## Description

### Contexte

Les équipes permettent d'organiser les utilisateurs en groupes logiques (départements, services, projets). Chaque équipe a un manager responsable de valider les pointages et absences de ses membres. Cette structure hiérarchique est fondamentale pour le workflow de validation.

### Objectif Business

Permettre aux organisations de structurer leurs employés en équipes avec des managers responsables, facilitant ainsi la délégation des validations et la visibilité par service.

### Valeur Apportée

- **Pour les admins** : Organisation claire des employés par service/département
- **Pour les managers** : Visibilité et responsabilité sur leur équipe uniquement
- **Pour les employés** : Identification claire de leur manager et collègues

---

## Scope

### Inclus

- CRUD complet des équipes
- Assignation d'un manager par équipe
- Assignation d'utilisateurs à une équipe
- Liste des membres d'une équipe
- Filtre des données par équipe pour les managers

### Exclus

- Équipes imbriquées (hiérarchie multi-niveaux)
- Utilisateur dans plusieurs équipes
- Historique des changements d'équipe
- Équipes inter-organisations

---

## Critères de Succès de l'Epic

- [ ] Un admin peut créer, modifier et supprimer des équipes
- [ ] Un admin peut assigner un manager à une équipe
- [ ] Un admin peut assigner des utilisateurs à une équipe
- [ ] Un manager voit automatiquement les données de son équipe
- [ ] La suppression d'une équipe désassigne ses membres (team_id = null)
- [ ] Un utilisateur ne peut appartenir qu'à une seule équipe

---

## User Stories

---

### TM-33 : Liste des équipes

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur connecté,
**Je veux** voir la liste des équipes de mon organisation,
**Afin de** connaître la structure organisationnelle.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/teams` créé
- [ ] Retourne : id, name, description, manager (id, name), members_count
- [ ] Accessible à tous les rôles (lecture)
- [ ] Tri par nom (alphabétique)
- [ ] Pagination optionnelle (si > 50 équipes)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-33.1 | Créer migration table teams | 1h |
| TM-33.2 | Créer modèle Team et repository | 1h |
| TM-33.3 | Créer TeamService.list() | 0.5h |
| TM-33.4 | Créer endpoint GET /teams | 0.5h |
| TM-33.5 | Tests d'intégration | 1h |

---

### TM-34 : Création d'une équipe

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** créer une nouvelle équipe,
**Afin de** structurer mon organisation.

#### Critères d'Acceptation

- [ ] Endpoint `POST /api/v1/teams` créé
- [ ] Champs requis : name
- [ ] Champs optionnels : description, manager_id
- [ ] Validation : nom unique dans l'organisation
- [ ] Si manager_id fourni : vérifie que le user existe et appartient à l'org
- [ ] Réservé aux ADMIN
- [ ] Retour : équipe créée avec id

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-34.1 | Créer DTO CreateTeamRequest | 0.5h |
| TM-34.2 | Implémenter TeamService.create() | 1h |
| TM-34.3 | Créer endpoint POST /teams | 0.5h |
| TM-34.4 | Tests d'intégration | 1h |

---

### TM-35 : Consultation d'une équipe

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 1 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur connecté,
**Je veux** voir le détail d'une équipe,
**Afin de** connaître ses membres et son manager.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/teams/:id` créé
- [ ] Retourne : id, name, description, manager (détail), created_at
- [ ] Accessible à tous les rôles
- [ ] Retour 404 si équipe d'une autre organisation

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-35.1 | Créer TeamRepository.find_by_id() | 0.5h |
| TM-35.2 | Créer endpoint GET /teams/:id | 0.5h |
| TM-35.3 | Tests d'intégration | 0.5h |

---

### TM-36 : Modification d'une équipe

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 1 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** modifier une équipe,
**Afin de** mettre à jour ses informations.

#### Critères d'Acceptation

- [ ] Endpoint `PUT /api/v1/teams/:id` créé
- [ ] Champs modifiables : name, description
- [ ] Validation nom unique (si changé)
- [ ] Réservé aux ADMIN
- [ ] Retour équipe mise à jour

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-36.1 | Créer DTO UpdateTeamRequest | 0.5h |
| TM-36.2 | Implémenter TeamService.update() | 0.5h |
| TM-36.3 | Créer endpoint PUT /teams/:id | 0.5h |
| TM-36.4 | Tests d'intégration | 0.5h |

---

### TM-37 : Suppression d'une équipe

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 1 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** supprimer une équipe,
**Afin de** réorganiser ma structure.

#### Contexte Détaillé

La suppression d'une équipe désassigne automatiquement tous ses membres (team_id = null). C'est un hard delete car l'équipe n'a pas d'historique critique à conserver.

#### Critères d'Acceptation

- [ ] Endpoint `DELETE /api/v1/teams/:id` créé
- [ ] Désassigne tous les membres (team_id = null)
- [ ] Hard delete de l'équipe
- [ ] Réservé aux ADMIN
- [ ] Retour 200 avec confirmation

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-37.1 | Implémenter désassignation membres | 0.5h |
| TM-37.2 | Implémenter TeamService.delete() | 0.5h |
| TM-37.3 | Créer endpoint DELETE /teams/:id | 0.5h |
| TM-37.4 | Tests d'intégration | 0.5h |

---

### TM-38 : Assignation du manager

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** assigner un manager à une équipe,
**Afin de** définir qui valide les pointages et absences.

#### Contexte Détaillé

Quand un utilisateur devient manager d'une équipe :
- Son rôle passe automatiquement à MANAGER (si était EMPLOYEE)
- Il peut voir les données de son équipe
- Il peut valider les demandes de son équipe

Un manager peut gérer plusieurs équipes.

#### Critères d'Acceptation

- [ ] Endpoint `PUT /api/v1/teams/:id/manager` créé
- [ ] Body : `{ "user_id": "uuid" }` ou `{ "user_id": null }` pour retirer
- [ ] Vérifie que l'utilisateur appartient à l'organisation
- [ ] Upgrade automatique du rôle en MANAGER si EMPLOYEE
- [ ] Notification envoyée au nouveau manager
- [ ] Réservé aux ADMIN
- [ ] Retour équipe mise à jour

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-38.1 | Créer DTO AssignManagerRequest | 0.5h |
| TM-38.2 | Implémenter TeamService.assign_manager() | 1h |
| TM-38.3 | Implémenter upgrade automatique du rôle | 0.5h |
| TM-38.4 | Créer endpoint PUT /teams/:id/manager | 0.5h |
| TM-38.5 | Tests d'intégration | 1h |

---

### TM-39 : Liste des membres d'une équipe

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 1 SP |
| **Assigné** | - |

#### Description

**En tant que** manager ou administrateur,
**Je veux** voir les membres d'une équipe,
**Afin de** connaître sa composition.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/teams/:id/members` créé
- [ ] Retourne liste des utilisateurs avec : id, email, first_name, last_name, role
- [ ] Admin : peut voir toutes les équipes
- [ ] Manager : peut voir uniquement son/ses équipe(s)
- [ ] Exclut les utilisateurs désactivés (sauf param include_deleted)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-39.1 | Créer UserRepository.find_by_team() | 0.5h |
| TM-39.2 | Implémenter autorisation manager/admin | 0.5h |
| TM-39.3 | Créer endpoint GET /teams/:id/members | 0.5h |
| TM-39.4 | Tests d'intégration | 0.5h |

---

### TM-40 : Page gestion des équipes (Frontend)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 5 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** une interface pour gérer les équipes,
**Afin de** structurer mon organisation visuellement.

#### Critères d'Acceptation

- [ ] Page `/teams` créée
- [ ] Liste des équipes en cards ou tableau :
  - Nom de l'équipe
  - Description (tronquée)
  - Manager (nom + avatar placeholder)
  - Nombre de membres
  - Actions (voir, modifier, supprimer)
- [ ] Bouton "Nouvelle équipe" → Modal création
- [ ] Clic sur équipe → Vue détail avec liste membres
- [ ] Modal modification équipe
- [ ] Modal assignation manager (dropdown users)
- [ ] Confirmation avant suppression
- [ ] Responsive (cards sur mobile)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-40.1 | Créer hook useTeams (query list, mutations) | 1h |
| TM-40.2 | Créer composant TeamCard | 1h |
| TM-40.3 | Créer modal CreateTeamModal | 1h |
| TM-40.4 | Créer modal EditTeamModal | 1h |
| TM-40.5 | Créer modal AssignManagerModal | 1h |
| TM-40.6 | Créer vue TeamDetail avec membres | 2h |
| TM-40.7 | Créer page TeamsPage | 1h |
| TM-40.8 | Styling responsive | 1h |
| TM-40.9 | Tests composants | 1h |

---

## Récapitulatif des Estimations

| Story | Titre | SP |
|-------|-------|:--:|
| TM-33 | Liste des équipes | 2 |
| TM-34 | Création d'une équipe | 2 |
| TM-35 | Consultation d'une équipe | 1 |
| TM-36 | Modification d'une équipe | 1 |
| TM-37 | Suppression d'une équipe | 1 |
| TM-38 | Assignation du manager | 2 |
| TM-39 | Liste des membres d'une équipe | 1 |
| TM-40 | Page gestion des équipes (Frontend) | 5 |
| **Total** | | **15 SP** |

---

## Notes Techniques

### Modèle Team

```rust
pub struct Team {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub manager_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Endpoints Récapitulatif

| Méthode | Endpoint | Permission |
|---------|----------|------------|
| GET | /api/v1/teams | All |
| POST | /api/v1/teams | Admin |
| GET | /api/v1/teams/:id | All |
| PUT | /api/v1/teams/:id | Admin |
| DELETE | /api/v1/teams/:id | Admin |
| PUT | /api/v1/teams/:id/manager | Admin |
| GET | /api/v1/teams/:id/members | Admin, Manager* |

*Manager = uniquement ses équipes

### Règles Métier

| Règle | Description |
|-------|-------------|
| Un user = une équipe | Un utilisateur ne peut appartenir qu'à une seule équipe |
| Manager multi-équipes | Un manager peut gérer plusieurs équipes |
| Upgrade auto | Assigner comme manager upgrade le rôle en MANAGER |
| Suppression cascade | Supprimer une équipe désassigne les membres |
| Nom unique | Le nom d'équipe est unique dans l'organisation |
