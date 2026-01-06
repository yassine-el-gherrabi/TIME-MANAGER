# TM-E03 : Multi-tenant & Organisations

## Informations Epic

| Champ | Valeur |
|-------|--------|
| **ID** | TM-E03 |
| **Titre** | Multi-tenant & Organisations |
| **Priorité** | P0 - Bloquant |
| **Estimation globale** | 13 SP |
| **Sprint cible** | Sprint 2 |
| **Dépendances** | TM-E01 (Infrastructure), TM-E02 (Authentification) |

---

## Description

### Contexte

Time Manager est une application SaaS multi-tenant : plusieurs entreprises (organisations) utilisent la même instance de l'application, mais leurs données sont strictement isolées. Un utilisateur de l'organisation A ne doit jamais voir les données de l'organisation B.

Cette isolation est fondamentale pour la sécurité et la confidentialité des données RH.

### Objectif Business

Garantir une isolation complète des données entre organisations tout en permettant aux super admins de gérer l'ensemble de la plateforme. Chaque organisation doit pouvoir personnaliser ses paramètres sans impacter les autres.

### Valeur Apportée

- **Pour les clients** : Garantie que leurs données RH restent privées
- **Pour la plateforme** : Architecture scalable permettant d'ajouter des clients facilement
- **Pour les super admins** : Vue globale et gestion de toutes les organisations

---

## Scope

### Inclus

- Middleware d'isolation tenant (filtrage automatique par organization_id)
- Contexte tenant injecté dans toutes les requêtes
- Endpoints de gestion de l'organisation courante
- Guard de permissions par rôle
- Gestion des paramètres organisation (timezone, settings)

### Exclus

- Interface super admin complète (gestion toutes orgs) - simplifié pour MVP
- Facturation par organisation
- Limites de quotas par organisation
- Personnalisation avancée (logo, couleurs)

---

## Critères de Succès de l'Epic

- [ ] Toutes les requêtes sont automatiquement filtrées par organization_id
- [ ] Un utilisateur ne peut jamais accéder aux données d'une autre organisation
- [ ] Les admins peuvent modifier les paramètres de leur organisation
- [ ] Le guard de rôles bloque correctement les accès non autorisés
- [ ] Les tests prouvent l'isolation (tentative d'accès cross-tenant = 404)

---

## User Stories

---

### TM-17 : Middleware d'isolation tenant

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 5 SP |
| **Assigné** | - |

#### Description

**En tant que** responsable sécurité,
**Je veux** que toutes les requêtes soient automatiquement filtrées par organisation,
**Afin de** garantir l'isolation des données entre clients.

#### Contexte Détaillé

Le middleware tenant s'exécute après le middleware d'authentification. Il :
1. Extrait l'organization_id depuis le CurrentUser (JWT)
2. Crée un TenantContext avec cet ID
3. Injecte le contexte dans la requête

Toutes les requêtes vers la base de données DOIVENT inclure ce filtre. C'est une règle architecturale non négociable.

#### Critères d'Acceptation

- [ ] Middleware `tenant_middleware` créé
- [ ] Extraction organization_id depuis CurrentUser
- [ ] Struct `TenantContext` avec organization_id
- [ ] Injection dans request extensions
- [ ] Extractor `TenantContext` pour les handlers
- [ ] Tous les repositories utilisent le TenantContext pour filtrer
- [ ] Pattern : `WHERE organization_id = $tenant_id AND ...`
- [ ] Exception : Super Admin peut bypass (accès cross-tenant)
- [ ] Tests d'isolation : user org A ne voit pas data org B

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-17.1 | Créer struct TenantContext | 0.5h |
| TM-17.2 | Créer le middleware tenant_middleware | 2h |
| TM-17.3 | Créer l'extractor TenantContext | 1h |
| TM-17.4 | Modifier les repositories pour utiliser TenantContext | 3h |
| TM-17.5 | Implémenter bypass Super Admin | 1h |
| TM-17.6 | Tests d'isolation cross-tenant | 2h |

---

### TM-18 : Consultation de l'organisation courante

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** utilisateur connecté,
**Je veux** voir les informations de mon organisation,
**Afin de** connaître les paramètres qui s'appliquent à moi.

#### Contexte Détaillé

Chaque utilisateur appartient à une seule organisation. Il doit pouvoir consulter :
- Le nom de l'organisation
- Le fuseau horaire (important pour le pointage)
- Les paramètres généraux

#### Critères d'Acceptation

- [ ] Endpoint `GET /api/v1/organizations/current` créé
- [ ] Retourne les informations de l'organisation du user connecté :
  - id, name, slug
  - timezone
  - created_at
- [ ] Accessible à tous les rôles (employee, manager, admin, super_admin)
- [ ] Utilise le TenantContext (pas de paramètre dans l'URL)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-18.1 | Créer OrganizationRepository.find_current() | 1h |
| TM-18.2 | Créer OrganizationService.get_current() | 0.5h |
| TM-18.3 | Créer endpoint GET /organizations/current | 1h |
| TM-18.4 | Créer DTO OrganizationResponse | 0.5h |
| TM-18.5 | Tests d'intégration | 1h |

---

### TM-19 : Modification de l'organisation

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** modifier les paramètres de mon organisation,
**Afin de** personnaliser Time Manager pour mon entreprise.

#### Contexte Détaillé

Seuls les admins peuvent modifier l'organisation. Les paramètres modifiables sont :
- Nom de l'organisation
- Fuseau horaire (affecte les calculs de pointage)

Le slug n'est pas modifiable (utilisé dans les URLs, références externes).

#### Critères d'Acceptation

- [ ] Endpoint `PUT /api/v1/organizations/current` créé
- [ ] Champs modifiables :
  - name (min 2 caractères)
  - timezone (format IANA : Europe/Paris)
- [ ] Validation du timezone (liste blanche ou validation IANA)
- [ ] Réservé aux rôles ADMIN et SUPER_ADMIN
- [ ] Retour 403 pour les autres rôles
- [ ] Log d'audit de la modification (pour E11)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-19.1 | Créer DTO UpdateOrganizationRequest | 0.5h |
| TM-19.2 | Implémenter validation timezone | 1h |
| TM-19.3 | Créer OrganizationService.update() | 1h |
| TM-19.4 | Créer endpoint PUT /organizations/current | 1h |
| TM-19.5 | Ajouter guard RequireRole(Admin) | 0.5h |
| TM-19.6 | Tests d'intégration (succès + 403) | 1h |

---

### TM-20 : Guard de permissions par rôle

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur backend,
**Je veux** un système de guards pour vérifier les rôles,
**Afin de** protéger les endpoints selon les permissions.

#### Contexte Détaillé

Les 4 rôles sont hiérarchiques (cascade) :
```
SUPER_ADMIN > ADMIN > MANAGER > EMPLOYEE
```

Un ADMIN a toutes les permissions d'un MANAGER et d'un EMPLOYEE.

Le guard doit permettre de protéger un endpoint en spécifiant le rôle minimum requis.

#### Critères d'Acceptation

- [ ] Enum `Role` avec ordre de hiérarchie
- [ ] Trait/fonction `has_permission(required: Role) -> bool`
- [ ] Extractor `RequireRole<R>` pour Axum
- [ ] Exemple d'utilisation :
  ```rust
  async fn admin_only(
      _: RequireRole<Admin>,
      current_user: CurrentUser
  ) -> Response { ... }
  ```
- [ ] Retour 403 Forbidden si rôle insuffisant
- [ ] Message d'erreur : "Insufficient permissions. Required: admin"
- [ ] Support du rôle MANAGER avec scope équipe (à préparer pour E05)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-20.1 | Créer enum Role avec Ord impl | 1h |
| TM-20.2 | Implémenter has_permission() | 0.5h |
| TM-20.3 | Créer extractor RequireRole<R> | 2h |
| TM-20.4 | Créer réponse 403 standardisée | 0.5h |
| TM-20.5 | Documenter l'utilisation | 0.5h |
| TM-20.6 | Tests unitaires des permissions | 1h |

---

### TM-21 : Page paramètres organisation (Frontend)

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** administrateur,
**Je veux** une page pour gérer les paramètres de mon organisation,
**Afin de** configurer Time Manager selon mes besoins.

#### Contexte Détaillé

Cette page fait partie des settings de l'application. Elle affiche les informations de l'organisation et permet aux admins de les modifier.

#### Critères d'Acceptation

- [ ] Page `/settings/organization` créée
- [ ] Affichage des informations actuelles :
  - Nom de l'organisation
  - Slug (non modifiable, affiché en lecture seule)
  - Timezone actuel
  - Date de création
- [ ] Formulaire de modification (admins uniquement) :
  - Champ nom
  - Sélecteur timezone (dropdown avec fuseaux courants)
- [ ] Bouton "Enregistrer" avec loading state
- [ ] Message de succès après modification
- [ ] Affichage erreurs de validation
- [ ] Masquer le formulaire pour les non-admins (affichage lecture seule)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-21.1 | Créer hook useOrganization (query + mutation) | 1h |
| TM-21.2 | Créer composant OrganizationForm | 2h |
| TM-21.3 | Créer sélecteur timezone avec recherche | 1h |
| TM-21.4 | Créer page OrganizationSettingsPage | 1h |
| TM-21.5 | Gérer les permissions (affichage conditionnel) | 1h |
| TM-21.6 | Tests composants | 1h |

---

## Récapitulatif des Estimations

| Story | Titre | SP |
|-------|-------|:--:|
| TM-17 | Middleware d'isolation tenant | 5 |
| TM-18 | Consultation de l'organisation courante | 2 |
| TM-19 | Modification de l'organisation | 2 |
| TM-20 | Guard de permissions par rôle | 3 |
| TM-21 | Page paramètres organisation (Frontend) | 3 |
| **Total** | | **15 SP** |

---

## Notes Techniques

### Pattern d'Isolation Tenant

```rust
// Dans chaque repository
impl UserRepository {
    pub fn find_all(&self, tenant: &TenantContext) -> Vec<User> {
        users::table
            .filter(users::organization_id.eq(tenant.organization_id))
            .load(&self.conn)
    }
}
```

### Hiérarchie des Rôles

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    Employee = 0,
    Manager = 1,
    Admin = 2,
    SuperAdmin = 3,
}

impl Role {
    pub fn has_permission(&self, required: Role) -> bool {
        *self >= required
    }
}
```

### Fuseaux Horaires Supportés (Exemples)

```
Europe/Paris
Europe/London
America/New_York
America/Los_Angeles
Asia/Tokyo
Asia/Shanghai
UTC
```

### Sécurité Multi-tenant

| Règle | Description |
|-------|-------------|
| Filtrage automatique | Toute requête BDD inclut organization_id |
| Pas d'ID dans URL | On utilise /current, pas /organizations/:id |
| Validation FK | Les FK vers d'autres tables vérifient l'org |
| Super Admin | Seul rôle pouvant bypass l'isolation |
| Tests obligatoires | Chaque feature doit avoir un test cross-tenant |
