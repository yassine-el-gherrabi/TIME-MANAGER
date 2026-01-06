# TM-E04 : Gestion des Utilisateurs

## Informations Epic

| Champ | Valeur |
|-------|--------|
| **ID** | TM-E04 |
| **Titre** | Gestion des Utilisateurs |
| **Priorité** | P0 - Bloquant |
| **Estimation globale** | 34 SP |
| **Sprint cible** | Sprint 3 |
| **Dépendances** | TM-E02 (Auth), TM-E03 (Multi-tenant) |

---

## Description

### Contexte

La gestion des utilisateurs est centrale dans Time Manager. Les administrateurs doivent pouvoir créer, modifier et désactiver les comptes des employés de leur organisation. Chaque utilisateur a un rôle qui détermine ses permissions dans l'application.

### Objectif Business

Permettre aux administrateurs de gérer le cycle de vie complet des utilisateurs : de l'invitation à la désactivation, en passant par l'attribution des rôles et l'assignation aux équipes.

### Valeur Apportée

- **Pour les admins** : Gestion autonome des utilisateurs sans intervention technique
- **Pour les employés** : Accès à leur profil et possibilité de le maintenir à jour
- **Pour l'entreprise** : Contrôle des accès et traçabilité des utilisateurs
- **Pour la conformité RGPD** : Droits d'accès, rectification, effacement et portabilité

---

## Scope

### Inclus

- CRUD complet des utilisateurs (création, lecture, modification, suppression)
- Soft delete (désactivation) avec possibilité de restauration
- Gestion des rôles (employee, manager, admin)
- Profil utilisateur personnel
- Changement de mot de passe
- Filtres et recherche dans la liste des utilisateurs
- Invitation par email (envoi credentials initiaux)
- **RGPD** : Export des données personnelles (Art. 15)
- **RGPD** : Droit à l'effacement avec anonymisation (Art. 17)
- **RGPD** : Portabilité des données (Art. 20)

### Exclus

- Import/export CSV d'utilisateurs en masse (post-MVP)
- Photo de profil / avatar
- Préférences utilisateur avancées (langue, thème)

---

## Critères de Succès de l'Epic

- [ ] Un admin peut créer un utilisateur qui reçoit un email d'invitation
- [ ] Un admin peut modifier les informations d'un utilisateur
- [ ] Un admin peut désactiver un utilisateur (soft delete)
- [ ] Un admin peut restaurer un utilisateur désactivé
- [ ] Un admin peut changer le rôle d'un utilisateur
- [ ] Chaque utilisateur peut voir et modifier son profil
- [ ] Chaque utilisateur peut changer son mot de passe
- [ ] Un manager ne voit que les utilisateurs de son équipe
- [ ] Un utilisateur peut exporter toutes ses données personnelles (RGPD)
- [ ] Un utilisateur peut demander l'effacement de ses données (RGPD)

---

## User Stories

---

### TM-22 : Liste des utilisateurs

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** voir la liste de tous les utilisateurs de mon organisation,
**Afin de** avoir une vue d'ensemble des comptes.

#### Contexte Détaillé

La liste doit être paginée et permettre de filtrer/rechercher. Les managers ont une vue restreinte à leur équipe uniquement.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/users` créé
- [ ] Pagination : `page`, `per_page` (défaut 20, max 100)
- [ ] Filtres query params :
  - `team_id` : filtrer par équipe
  - `role` : filtrer par rôle
  - `search` : recherche sur email, first_name, last_name
  - `include_deleted` : inclure les désactivés (admin only)
- [ ] Tri : `sort_by` (name, email, created_at), `sort_order` (asc, desc)
- [ ] Réponse avec meta pagination (total, page, per_page)
- [ ] Manager : filtre automatique sur son équipe
- [ ] Admin : voit toute l'organisation
- [ ] Champs retournés : id, email, first_name, last_name, role, team, created_at, deleted_at

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-22.1 | Créer DTO ListUsersQuery avec validations | 1h |
| TM-22.2 | Implémenter UserRepository.find_all() avec filtres | 2h |
| TM-22.3 | Créer UserService.list() avec logique rôle | 1h |
| TM-22.4 | Créer endpoint GET /users | 1h |
| TM-22.5 | Créer DTO UserListResponse avec pagination | 0.5h |
| TM-22.6 | Tests d'intégration (filtres, pagination, rôles) | 2h |

---

### TM-23 : Création d'un utilisateur

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** créer un nouvel utilisateur dans mon organisation,
**Afin de** lui donner accès à Time Manager.

#### Contexte Détaillé

L'admin crée le compte avec les informations de base. Un email d'invitation est envoyé avec un lien pour définir le mot de passe (utilise le flow reset password).

#### Critères d'Acceptation

- [ ] Endpoint `POST /api/v1/users` créé
- [ ] Champs requis : email, first_name, last_name, role
- [ ] Champs optionnels : phone, team_id
- [ ] Validation :
  - Email unique dans l'organisation
  - Role valide (employee, manager, admin - pas super_admin)
  - Team appartient à l'organisation
- [ ] Génération d'un mot de passe temporaire aléatoire
- [ ] Envoi email d'invitation avec lien de reset password
- [ ] Réservé aux ADMIN
- [ ] Retour : utilisateur créé avec id

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-23.1 | Créer DTO CreateUserRequest | 0.5h |
| TM-23.2 | Implémenter UserService.create() | 2h |
| TM-23.3 | Créer template email d'invitation | 1h |
| TM-23.4 | Créer endpoint POST /users | 1h |
| TM-23.5 | Tests d'intégration | 1h |

---

### TM-24 : Consultation d'un utilisateur

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 1 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur ou manager,
**Je veux** voir le détail d'un utilisateur,
**Afin de** consulter ses informations complètes.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/users/:id` créé
- [ ] Retourne toutes les infos de l'utilisateur
- [ ] Admin : peut voir tout utilisateur de l'organisation
- [ ] Manager : peut voir uniquement les membres de son équipe
- [ ] Employee : peut voir uniquement son propre profil
- [ ] Retour 404 si user d'une autre organisation ou non autorisé

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-24.1 | Créer UserRepository.find_by_id() | 0.5h |
| TM-24.2 | Implémenter logique d'autorisation | 1h |
| TM-24.3 | Créer endpoint GET /users/:id | 0.5h |
| TM-24.4 | Tests d'intégration (permissions) | 1h |

---

### TM-25 : Modification d'un utilisateur

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** modifier les informations d'un utilisateur,
**Afin de** maintenir les données à jour.

#### Critères d'Acceptation

- [ ] Endpoint `PUT /api/v1/users/:id` créé
- [ ] Champs modifiables : first_name, last_name, email, phone, team_id
- [ ] Validation email unique (si changé)
- [ ] Réservé aux ADMIN
- [ ] Ne peut pas modifier un SUPER_ADMIN
- [ ] Ne peut pas se modifier soi-même via cet endpoint (utiliser /auth/me)
- [ ] Retour 403 si permissions insuffisantes

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-25.1 | Créer DTO UpdateUserRequest | 0.5h |
| TM-25.2 | Implémenter UserService.update() | 1.5h |
| TM-25.3 | Créer endpoint PUT /users/:id | 0.5h |
| TM-25.4 | Tests d'intégration | 1h |

---

### TM-26 : Désactivation d'un utilisateur (Soft Delete)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** désactiver un utilisateur,
**Afin qu'** il ne puisse plus se connecter tout en conservant son historique.

#### Contexte Détaillé

Le soft delete met à jour `deleted_at` plutôt que de supprimer la ligne. Cela permet de :
- Conserver l'historique (pointages, absences)
- Restaurer le compte si besoin (pendant période de grâce)
- Respecter les obligations légales de conservation (6 ans pour données RH)

> ⚠️ **RGPD** : Cette action déclenche un délai de grâce de 30 jours. Après ce délai, les données personnelles sont anonymisées automatiquement (voir TM-108).

#### Critères d'Acceptation

- [ ] Endpoint `DELETE /api/v1/users/:id` créé
- [ ] Met à jour deleted_at avec timestamp actuel
- [ ] Révoque tous les refresh tokens de l'utilisateur
- [ ] L'utilisateur ne peut plus se connecter
- [ ] Réservé aux ADMIN
- [ ] Ne peut pas désactiver un SUPER_ADMIN
- [ ] Ne peut pas se désactiver soi-même
- [ ] Retour 200 avec message de confirmation
- [ ] Email de notification envoyé à l'utilisateur (RGPD)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-26.1 | Implémenter UserService.soft_delete() | 1h |
| TM-26.2 | Ajouter révocation des tokens | 0.5h |
| TM-26.3 | Créer endpoint DELETE /users/:id | 0.5h |
| TM-26.4 | Modifier login pour vérifier deleted_at | 0.5h |
| TM-26.5 | Envoyer email notification suppression | 0.5h |
| TM-26.6 | Tests d'intégration | 1h |

---

### TM-27 : Restauration d'un utilisateur

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 1 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** restaurer un utilisateur désactivé,
**Afin de** lui redonner accès à l'application.

#### Critères d'Acceptation

- [ ] Endpoint `PUT /api/v1/users/:id/restore` créé
- [ ] Remet deleted_at à NULL
- [ ] L'utilisateur peut à nouveau se connecter
- [ ] Réservé aux ADMIN
- [ ] Retour 404 si utilisateur non trouvé ou non désactivé
- [ ] Retour 200 avec utilisateur restauré

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-27.1 | Implémenter UserService.restore() | 0.5h |
| TM-27.2 | Créer endpoint PUT /users/:id/restore | 0.5h |
| TM-27.3 | Tests d'intégration | 0.5h |

---

### TM-28 : Changement de rôle

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** modifier le rôle d'un utilisateur,
**Afin de** lui attribuer les bonnes permissions.

#### Contexte Détaillé

Le changement de rôle est une action sensible qui doit être tracée. Les règles :
- ADMIN peut assigner : employee, manager, admin
- SUPER_ADMIN peut assigner tous les rôles
- On ne peut pas changer son propre rôle

#### Critères d'Acceptation

- [ ] Endpoint `PUT /api/v1/users/:id/role` créé
- [ ] Body : `{ "role": "manager" }`
- [ ] Validation du rôle (enum valide)
- [ ] ADMIN : peut assigner employee, manager, admin
- [ ] SUPER_ADMIN : peut assigner tous les rôles
- [ ] Ne peut pas modifier son propre rôle
- [ ] Ne peut pas downgrade un SUPER_ADMIN
- [ ] Log d'audit de la modification
- [ ] Retour utilisateur mis à jour

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-28.1 | Créer DTO ChangeRoleRequest | 0.5h |
| TM-28.2 | Implémenter UserService.change_role() | 1h |
| TM-28.3 | Créer endpoint PUT /users/:id/role | 0.5h |
| TM-28.4 | Ajouter log d'audit | 0.5h |
| TM-28.5 | Tests d'intégration (tous les cas) | 1h |

---

### TM-29 : Profil utilisateur personnel

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur connecté,
**Je veux** voir et modifier mon profil,
**Afin de** maintenir mes informations à jour.

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/auth/me` créé (déjà dans E02, enrichir)
- [ ] Retourne : id, email, first_name, last_name, phone, role, organization, team
- [ ] Endpoint `PUT /api/v1/auth/me` créé
- [ ] Champs modifiables : first_name, last_name, phone
- [ ] Email non modifiable (sécurité)
- [ ] Validation des données
- [ ] Accessible à tous les rôles (son propre profil)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-29.1 | Enrichir GET /auth/me avec toutes les infos | 1h |
| TM-29.2 | Créer DTO UpdateProfileRequest | 0.5h |
| TM-29.3 | Implémenter UserService.update_profile() | 1h |
| TM-29.4 | Créer endpoint PUT /auth/me | 0.5h |
| TM-29.5 | Tests d'intégration | 1h |

---

### TM-30 : Changement de mot de passe

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur connecté,
**Je veux** changer mon mot de passe,
**Afin de** sécuriser mon compte.

#### Critères d'Acceptation

- [ ] Endpoint `PUT /api/v1/auth/me/password` créé
- [ ] Body : current_password, new_password, confirm_password
- [ ] Vérification du mot de passe actuel
- [ ] Validation nouveau mot de passe (mêmes règles que inscription)
- [ ] Vérification new_password == confirm_password
- [ ] Hash avec Argon2
- [ ] Révocation de tous les autres refresh tokens (sécurité)
- [ ] Session courante conservée

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-30.1 | Créer DTO ChangePasswordRequest | 0.5h |
| TM-30.2 | Implémenter AuthService.change_password() | 1h |
| TM-30.3 | Créer endpoint PUT /auth/me/password | 0.5h |
| TM-30.4 | Implémenter révocation autres tokens | 0.5h |
| TM-30.5 | Tests d'intégration | 1h |

---

### TM-31 : Page liste utilisateurs (Frontend)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 5 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** une interface pour voir et gérer tous les utilisateurs,
**Afin de** administrer facilement les comptes.

#### Critères d'Acceptation

- [ ] Page `/users` créée
- [ ] Tableau avec colonnes : Nom, Email, Rôle, Équipe, Statut, Actions
- [ ] Pagination (20 par page par défaut)
- [ ] Filtres :
  - Recherche textuelle (nom, email)
  - Filtre par rôle (dropdown)
  - Filtre par équipe (dropdown)
  - Toggle "Afficher désactivés"
- [ ] Tri sur colonnes cliquables
- [ ] Actions par ligne :
  - Voir détail
  - Modifier
  - Changer rôle
  - Désactiver / Restaurer
- [ ] Bouton "Nouvel utilisateur" ouvrant modal
- [ ] Badge visuel pour utilisateurs désactivés
- [ ] Responsive (cards sur mobile)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-31.1 | Créer hook useUsers avec filtres et pagination | 2h |
| TM-31.2 | Créer composant UsersTable | 3h |
| TM-31.3 | Créer composants filtres (SearchInput, RoleFilter, etc.) | 2h |
| TM-31.4 | Créer modal CreateUserModal | 2h |
| TM-31.5 | Créer page UsersPage assemblant le tout | 1h |
| TM-31.6 | Implémenter actions (edit, delete, restore) | 2h |
| TM-31.7 | Styling responsive | 1h |
| TM-31.8 | Tests composants | 1h |

---

### TM-32 : Page profil personnel (Frontend)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur,
**Je veux** une page pour voir et modifier mon profil,
**Afin de** gérer mes informations personnelles.

#### Critères d'Acceptation

- [ ] Page `/profile` créée
- [ ] Section informations personnelles :
  - Affichage : email (lecture seule), nom, prénom, téléphone
  - Formulaire de modification
- [ ] Section sécurité :
  - Formulaire changement mot de passe
  - Champs : mot de passe actuel, nouveau, confirmation
  - Indicateur de force du mot de passe
- [ ] Section informations (lecture seule) :
  - Rôle
  - Équipe
  - Organisation
  - Date de création du compte
- [ ] Messages de succès/erreur
- [ ] Accessible à tous les utilisateurs connectés

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-32.1 | Créer composant ProfileInfoForm | 2h |
| TM-32.2 | Créer composant ChangePasswordForm | 2h |
| TM-32.3 | Créer page ProfilePage | 1h |
| TM-32.4 | Intégrer mutations (update profile, change password) | 1h |
| TM-32.5 | Styling et responsive | 1h |
| TM-32.6 | Tests composants | 1h |

---

### TM-108 : Export des données personnelles (RGPD Art. 15)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur,
**Je veux** pouvoir exporter toutes mes données personnelles,
**Afin d'** exercer mon droit d'accès RGPD.

#### Contexte Détaillé

L'article 15 du RGPD accorde à toute personne le droit d'obtenir une copie de ses données personnelles. L'export doit être complet et fourni dans un délai raisonnable (30 jours max légalement).

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/users/me/data-export` créé
- [ ] Confirmation d'identité requise (mot de passe dans header ou body)
- [ ] Retourne JSON structuré avec :
  - Données profil (email, nom, prénom, téléphone)
  - Historique complet des pointages
  - Historique complet des absences
  - Soldes de congés actuels
  - Actions dans audit_logs concernant l'utilisateur
- [ ] Rate limiting : 1 export par jour par utilisateur
- [ ] Log dans audit_logs
- [ ] Option format : JSON (défaut) ou CSV (voir TM-110)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-108.1 | Créer service DataExportService | 2h |
| TM-108.2 | Agréger toutes les données utilisateur | 2h |
| TM-108.3 | Créer endpoint GET /users/me/data-export | 1h |
| TM-108.4 | Implémenter rate limiting | 0.5h |
| TM-108.5 | Tests d'intégration | 1h |

---

### TM-109 : Demande d'effacement (RGPD Art. 17)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur,
**Je veux** pouvoir demander l'effacement de mes données,
**Afin d'** exercer mon droit à l'oubli RGPD.

#### Contexte Détaillé

L'article 17 du RGPD permet la suppression des données personnelles, sauf obligation légale de conservation (6 ans pour données RH). Le processus :
1. L'utilisateur demande l'effacement
2. Délai de grâce 30 jours (possibilité d'annulation)
3. Après 30 jours, anonymisation des données personnelles
4. Conservation des données non-personnelles (IDs, dates) pour intégrité

#### Critères d'Acceptation

- [ ] Endpoint `POST /api/v1/users/me/deletion-request` créé
- [ ] Confirmation d'identité requise (mot de passe)
- [ ] Email de confirmation envoyé avec lien d'annulation
- [ ] Délai de grâce : 30 jours avant anonymisation
- [ ] Endpoint `POST /api/v1/users/me/cancel-deletion` pour annuler
- [ ] Admin peut annuler pendant le délai de grâce
- [ ] Après 30 jours, anonymisation automatique :
  - email → null ou hash unique
  - first_name, last_name → "Utilisateur anonyme"
  - phone → null
  - Conservation : ID (intégrité), dates pointages (obligation légale 6 ans)
- [ ] Log dans audit_logs

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-109.1 | Ajouter colonne deletion_requested_at sur users | 0.5h |
| TM-109.2 | Créer endpoint POST /users/me/deletion-request | 1h |
| TM-109.3 | Créer endpoint POST /users/me/cancel-deletion | 0.5h |
| TM-109.4 | Implémenter service d'anonymisation | 2h |
| TM-109.5 | Créer job cron d'anonymisation (> 30 jours) | 1.5h |
| TM-109.6 | Emails de confirmation et rappel | 1h |
| TM-109.7 | Tests d'intégration | 1h |

---

### TM-110 : Portabilité des données (RGPD Art. 20)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur,
**Je veux** exporter mes données dans un format portable,
**Afin de** les transférer vers un autre service.

#### Contexte Détaillé

L'article 20 du RGPD impose un format structuré, couramment utilisé et lisible par machine. Les formats CSV et JSON sont appropriés.

#### Critères d'Acceptation

- [ ] Extension de TM-108 avec paramètre `format`
- [ ] Formats supportés : JSON (défaut), CSV
- [ ] Structure CSV documentée et standardisée
- [ ] Fichiers séparés par type de données (pointages.csv, absences.csv, profil.csv)
- [ ] Archive ZIP si multiple fichiers
- [ ] Documentation du schéma de données fournie

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-110.1 | Implémenter export CSV | 2h |
| TM-110.2 | Implémenter génération ZIP | 1h |
| TM-110.3 | Documenter schéma de données | 1h |
| TM-110.4 | Tests d'intégration | 1h |

---

## Récapitulatif des Estimations

| Story | Titre | SP |
|-------|-------|:--:|
| TM-22 | Liste des utilisateurs | 3 |
| TM-23 | Création d'un utilisateur | 3 |
| TM-24 | Consultation d'un utilisateur | 1 |
| TM-25 | Modification d'un utilisateur | 2 |
| TM-26 | Désactivation d'un utilisateur | 2 |
| TM-27 | Restauration d'un utilisateur | 1 |
| TM-28 | Changement de rôle | 2 |
| TM-29 | Profil utilisateur personnel | 2 |
| TM-30 | Changement de mot de passe | 2 |
| TM-31 | Page liste utilisateurs (Frontend) | 5 |
| TM-32 | Page profil personnel (Frontend) | 3 |
| TM-108 | Export données personnelles (RGPD Art. 15) | 3 |
| TM-109 | Demande d'effacement (RGPD Art. 17) | 3 |
| TM-110 | Portabilité des données (RGPD Art. 20) | 2 |
| **Total** | | **34 SP** |

---

## Notes Techniques

### Endpoints Récapitulatif

| Méthode | Endpoint | Permission |
|---------|----------|------------|
| GET | /api/v1/users | Admin, Manager* |
| POST | /api/v1/users | Admin |
| GET | /api/v1/users/:id | Admin, Manager*, Self |
| PUT | /api/v1/users/:id | Admin |
| DELETE | /api/v1/users/:id | Admin |
| PUT | /api/v1/users/:id/restore | Admin |
| PUT | /api/v1/users/:id/role | Admin |
| GET | /api/v1/auth/me | All |
| PUT | /api/v1/auth/me | All |
| PUT | /api/v1/auth/me/password | All |
| GET | /api/v1/users/me/data-export | All |
| POST | /api/v1/users/me/deletion-request | All |
| POST | /api/v1/users/me/cancel-deletion | All |

*Manager = uniquement son équipe

### Endpoints RGPD

| Méthode | Endpoint | Description | Rate Limit |
|---------|----------|-------------|------------|
| GET | /api/v1/users/me/data-export | Export données personnelles | 1/jour |
| GET | /api/v1/users/me/data-export?format=csv | Export portable CSV | 1/jour |
| POST | /api/v1/users/me/deletion-request | Demande effacement | 1/mois |
| POST | /api/v1/users/me/cancel-deletion | Annulation effacement | - |

### Modèle User

```rust
pub struct User {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub team_id: Option<Uuid>,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
```

### Règles de Validation Email

- Format valide (regex)
- Unique dans l'organisation (pas globalement, sauf pour login)
- Longueur max 255 caractères
- Lowercase automatique
