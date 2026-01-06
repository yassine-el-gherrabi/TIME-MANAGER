# TM-E12 : Frontend Core

## Informations Epic

| Champ | Valeur |
|-------|--------|
| **ID** | TM-E12 |
| **Titre** | Frontend Core |
| **Priorité** | P1 - Haute |
| **Estimation globale** | 21 SP |
| **Sprint cible** | Sprint 3-6 (parallèle aux autres Epics) |
| **Dépendances** | TM-E01 (Infrastructure) |

---

## Description

### Contexte

Cette Epic regroupe les composants frontend transversaux qui ne sont pas spécifiques à une fonctionnalité métier : layout principal, navigation, thème, composants réutilisables, gestion d'erreurs globale, etc. Ces éléments forment le socle de l'expérience utilisateur.

### Objectif Business

Fournir une expérience utilisateur cohérente, moderne et accessible sur l'ensemble de l'application, avec une interface responsive fonctionnant sur desktop et mobile.

### Valeur Apportée

- **Pour les utilisateurs** : Interface intuitive, cohérente et accessible
- **Pour les développeurs** : Composants réutilisables accélérant le développement
- **Pour l'accessibilité** : Conformité WCAG 2.1 AA
- **Pour la marque** : Image professionnelle et moderne

---

## Scope

### Inclus

- Layout principal avec sidebar et header
- Système de navigation par rôle
- Thème et design system (Shadcn/UI)
- Gestion globale des erreurs
- Loading states et skeletons
- Responsive design (mobile-first)
- Composants de base réutilisables

### Exclus

- Dark mode (v2)
- Internationalisation (i18n)
- PWA (Progressive Web App)
- Mode offline
- Personnalisation du thème par organisation

---

## Critères de Succès de l'Epic

- [ ] Le layout s'adapte correctement du mobile au desktop
- [ ] La navigation affiche uniquement les pages accessibles selon le rôle
- [ ] Les erreurs API sont gérées globalement avec messages clairs
- [ ] Les états de chargement sont présents sur toutes les pages
- [ ] L'interface respecte les standards d'accessibilité (a11y)
- [ ] Les composants suivent le design system Shadcn/UI

---

## User Stories

---

### TM-77 : Layout principal

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur connecté,
**Je veux** une interface avec un layout cohérent,
**Afin de** naviguer facilement dans l'application.

#### Contexte Détaillé

Le layout comprend :
- Header fixe avec logo, titre de page, notifications, menu utilisateur
- Sidebar rétractable avec navigation principale
- Zone de contenu principale
- Footer optionnel avec informations légales

Sur mobile, la sidebar devient un drawer qui s'ouvre au tap sur le burger menu.

#### Critères d'Acceptation

- [ ] Composant `MainLayout` créé
- [ ] Header fixe avec hauteur constante
- [ ] Sidebar de 240px sur desktop, drawer sur mobile
- [ ] Zone de contenu avec scroll indépendant
- [ ] Sidebar rétractable (icônes seules) sur tablette
- [ ] Breakpoints : mobile (< 768px), tablet (768-1024px), desktop (> 1024px)
- [ ] Transitions fluides

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-77.1 | Créer composant Header | 1.5h |
| TM-77.2 | Créer composant Sidebar | 2h |
| TM-77.3 | Créer composant MobileDrawer | 1.5h |
| TM-77.4 | Créer composant MainLayout | 1h |
| TM-77.5 | Implémenter responsive breakpoints | 1h |
| TM-77.6 | Tests composants | 1h |

---

### TM-78 : Navigation par rôle

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur connecté,
**Je veux** voir uniquement les menus auxquels j'ai accès,
**Afin de** ne pas être confus par des options inaccessibles.

#### Contexte Détaillé

La navigation est construite dynamiquement selon le rôle :
- **Employee** : Dashboard, Pointage, Mes absences, Profil
- **Manager** : + Équipe, Validations, Rapports équipe
- **Admin** : + Utilisateurs, Équipes, Paramètres org, Audit logs
- **Super Admin** : + Organisations (future)

#### Critères d'Acceptation

- [ ] Configuration centralisée des routes avec permissions
- [ ] Composant `NavMenu` affichant uniquement les items autorisés
- [ ] Icônes pour chaque item de menu
- [ ] Indication visuelle de la page active
- [ ] Sous-menus collapsibles si nécessaire
- [ ] Badge sur certains items (ex: validations en attente)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-78.1 | Définir configuration routes avec rôles | 1h |
| TM-78.2 | Créer composant NavItem | 0.5h |
| TM-78.3 | Créer composant NavMenu avec filtrage | 1.5h |
| TM-78.4 | Ajouter badges dynamiques | 1h |
| TM-78.5 | Tests composants | 1h |

---

### TM-79 : Menu utilisateur

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 1 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur connecté,
**Je veux** un menu avec mes informations et actions de compte,
**Afin d'** accéder rapidement à mon profil et me déconnecter.

#### Critères d'Acceptation

- [ ] Avatar (initiales ou image) + nom dans le header
- [ ] Dropdown au clic avec :
  - Mon profil
  - Paramètres
  - Se déconnecter
- [ ] Affichage du rôle
- [ ] Nom de l'organisation visible
- [ ] Confirmation avant déconnexion optionnelle

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-79.1 | Créer composant UserAvatar | 0.5h |
| TM-79.2 | Créer composant UserMenu dropdown | 1h |
| TM-79.3 | Intégrer dans Header | 0.5h |
| TM-79.4 | Tests composants | 0.5h |

---

### TM-80 : Gestion globale des erreurs

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur,
**Je veux** voir des messages d'erreur clairs quand quelque chose échoue,
**Afin de** comprendre le problème et savoir quoi faire.

#### Contexte Détaillé

Types d'erreurs à gérer :
- Erreurs réseau (offline, timeout)
- Erreurs API (400, 401, 403, 404, 500)
- Erreurs de validation formulaires
- Erreurs JavaScript non capturées

#### Critères d'Acceptation

- [ ] ErrorBoundary global capturant les erreurs React
- [ ] Page d'erreur 404 custom
- [ ] Page d'erreur 500 custom
- [ ] Intercepteur Axios pour erreurs API
- [ ] Toast notifications pour erreurs non bloquantes
- [ ] Messages d'erreur traduits et compréhensibles
- [ ] Bouton "Réessayer" où pertinent
- [ ] Logging des erreurs (console en dev)

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-80.1 | Créer composant ErrorBoundary | 1h |
| TM-80.2 | Créer pages 404 et 500 | 1h |
| TM-80.3 | Configurer intercepteur Axios | 1h |
| TM-80.4 | Créer système de toast avec messages | 1h |
| TM-80.5 | Définir mapping erreurs → messages | 0.5h |
| TM-80.6 | Tests composants | 1h |

---

### TM-81 : Loading states et skeletons

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant qu'** utilisateur,
**Je veux** voir des indicateurs de chargement pendant les requêtes,
**Afin de** savoir que l'application travaille.

#### Critères d'Acceptation

- [ ] Composant `Skeleton` générique (lignes, cercles, rectangles)
- [ ] Skeleton spécifiques : TableSkeleton, CardSkeleton, FormSkeleton
- [ ] Spinner pour actions courtes
- [ ] Loading overlay pour actions bloquantes
- [ ] Suspense boundaries pour lazy loading
- [ ] TanStack Query gère les états loading automatiquement

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-81.1 | Créer composants Skeleton de base | 1h |
| TM-81.2 | Créer TableSkeleton et CardSkeleton | 1h |
| TM-81.3 | Créer composant LoadingOverlay | 0.5h |
| TM-81.4 | Intégrer dans les pages existantes | 1h |
| TM-81.5 | Tests composants | 0.5h |

---

### TM-82 : Composants de formulaire

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur frontend,
**Je veux** des composants de formulaire réutilisables,
**Afin de** créer des formulaires cohérents rapidement.

#### Contexte Détaillé

Basé sur Shadcn/UI + React Hook Form, avec :
- Validation intégrée (Zod)
- Messages d'erreur inline
- États disabled et loading
- Accessibilité (labels, aria)

#### Critères d'Acceptation

- [ ] Composants wrappés : Input, Select, Checkbox, DatePicker, Textarea
- [ ] Intégration React Hook Form avec Controller
- [ ] Affichage des erreurs de validation sous les champs
- [ ] Composant FormField générique
- [ ] États : default, focus, error, disabled, loading
- [ ] Labels et hints accessibles

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-82.1 | Configurer React Hook Form + Zod | 1h |
| TM-82.2 | Créer composant FormField wrapper | 1h |
| TM-82.3 | Wrapper Input avec validation | 0.5h |
| TM-82.4 | Wrapper Select avec validation | 0.5h |
| TM-82.5 | Wrapper DatePicker avec validation | 1h |
| TM-82.6 | Créer composant FormActions (boutons) | 0.5h |
| TM-82.7 | Documentation des composants | 0.5h |
| TM-82.8 | Tests composants | 1h |

---

### TM-83 : Tableaux de données

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur frontend,
**Je veux** un composant de tableau de données réutilisable,
**Afin d'** afficher des listes de manière cohérente.

#### Contexte Détaillé

Basé sur TanStack Table avec :
- Tri côté client ou serveur
- Pagination
- Filtres
- Sélection de lignes
- Actions par ligne
- Responsive (horizontal scroll ou cards sur mobile)

#### Critères d'Acceptation

- [ ] Composant `DataTable` générique
- [ ] Configuration des colonnes déclarative
- [ ] Tri cliquable sur les en-têtes
- [ ] Pagination avec choix du nombre par page
- [ ] Slot pour filtres au-dessus
- [ ] Actions par ligne (menu ou boutons)
- [ ] Mode carte sur mobile
- [ ] Empty state personnalisable avec :
  - Illustration ou icône contextuelle
  - Message explicatif clair
  - Call-to-action (CTA) pertinent
  - Support du mode skeleton pendant le chargement

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-83.1 | Configurer TanStack Table | 1h |
| TM-83.2 | Créer composant DataTable | 2h |
| TM-83.3 | Créer composant TablePagination | 1h |
| TM-83.4 | Créer composant TableFilters | 1h |
| TM-83.5 | Implémenter mode cards mobile | 1.5h |
| TM-83.6 | Tests composants | 1h |

---

### TM-84 : Modales et dialogs

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** développeur frontend,
**Je veux** des composants de modales standardisés,
**Afin de** créer des dialogs cohérents.

#### Critères d'Acceptation

- [ ] Composant `Modal` basé sur Shadcn Dialog
- [ ] Variants : small, medium, large, fullscreen (mobile)
- [ ] Header avec titre et bouton fermer
- [ ] Footer avec actions
- [ ] Fermeture sur Escape et clic extérieur
- [ ] Animation d'ouverture/fermeture
- [ ] Composant `ConfirmDialog` pour confirmations
- [ ] Composant `AlertDialog` pour alertes

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-84.1 | Créer composant Modal wrapper | 1h |
| TM-84.2 | Créer composant ConfirmDialog | 1h |
| TM-84.3 | Créer composant AlertDialog | 0.5h |
| TM-84.4 | Créer hook useModal | 0.5h |
| TM-84.5 | Tests composants | 1h |

---

### TM-85 : Dashboard Employee

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P0 |
| **Estimation** | 3 SP |
| **Assigné** | - |

#### Description

**En tant qu'** employé,
**Je veux** un tableau de bord personnalisé,
**Afin d'** avoir une vue d'ensemble de ma situation.

#### Critères d'Acceptation

- [ ] Page `/dashboard` créée
- [ ] Widget de pointage (statut actuel, bouton clock in/out)
- [ ] Résumé heures de la semaine
- [ ] KPIs personnels (ponctualité, heures)
- [ ] Demandes d'absence en cours
- [ ] Notifications récentes
- [ ] Accès rapide aux actions fréquentes
- [ ] Responsive avec réorganisation des widgets
- [ ] Empty states pour les widgets sans données :
  - "Aucun pointage aujourd'hui" avec bouton Clock In
  - "Aucune demande d'absence en cours" avec bouton Nouvelle demande
  - "Aucune notification" avec icône adaptée

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-85.1 | Créer composant DashboardWidget | 1h |
| TM-85.2 | Créer layout dashboard responsive | 1h |
| TM-85.3 | Intégrer ClockWidget | 0.5h |
| TM-85.4 | Intégrer KPIs | 0.5h |
| TM-85.5 | Créer composant QuickActions | 1h |
| TM-85.6 | Créer page EmployeeDashboard | 1.5h |
| TM-85.7 | Tests composants | 1h |

---

### TM-86 : Dashboard Manager

| Champ | Valeur |
|-------|--------|
| **Type** | Story |
| **Priorité** | P1 |
| **Estimation** | 2 SP |
| **Assigné** | - |

#### Description

**En tant que** manager,
**Je veux** un tableau de bord avec vue équipe,
**Afin de** suivre mon équipe et traiter les validations.

#### Critères d'Acceptation

- [ ] Extension du dashboard employee
- [ ] Widget présence équipe (qui est là aujourd'hui)
- [ ] Widget validations en attente avec count
- [ ] KPIs équipe agrégés
- [ ] Raccourci vers les validations
- [ ] Alertes : retards récurrents, soldes bas

#### Sub-tasks

| ID | Tâche | Estimation |
|----|-------|------------|
| TM-86.1 | Créer composant TeamPresenceWidget | 1h |
| TM-86.2 | Créer composant PendingValidationsWidget | 1h |
| TM-86.3 | Créer page ManagerDashboard | 1.5h |
| TM-86.4 | Tests composants | 0.5h |

---

## Récapitulatif des Estimations

| Story | Titre | SP |
|-------|-------|:--:|
| TM-77 | Layout principal | 3 |
| TM-78 | Navigation par rôle | 2 |
| TM-79 | Menu utilisateur | 1 |
| TM-80 | Gestion globale des erreurs | 2 |
| TM-81 | Loading states et skeletons | 2 |
| TM-82 | Composants de formulaire | 3 |
| TM-83 | Tableaux de données | 3 |
| TM-84 | Modales et dialogs | 2 |
| TM-85 | Dashboard Employee | 3 |
| TM-86 | Dashboard Manager | 2 |
| **Total** | | **23 SP** |

---

## Notes Techniques

### Structure des Composants

```
src/
├── components/
│   ├── ui/                 # Shadcn/UI components
│   │   ├── button.tsx
│   │   ├── input.tsx
│   │   └── ...
│   ├── layout/
│   │   ├── MainLayout.tsx
│   │   ├── Header.tsx
│   │   ├── Sidebar.tsx
│   │   └── MobileDrawer.tsx
│   ├── forms/
│   │   ├── FormField.tsx
│   │   ├── FormInput.tsx
│   │   └── ...
│   ├── data/
│   │   ├── DataTable.tsx
│   │   └── TablePagination.tsx
│   ├── feedback/
│   │   ├── ErrorBoundary.tsx
│   │   ├── Skeleton.tsx
│   │   └── Toast.tsx
│   └── shared/
│       ├── Modal.tsx
│       ├── ConfirmDialog.tsx
│       └── ...
├── hooks/
│   ├── useAuth.ts
│   ├── useModal.ts
│   └── useToast.ts
└── pages/
    ├── dashboard/
    │   ├── EmployeeDashboard.tsx
    │   └── ManagerDashboard.tsx
    └── errors/
        ├── NotFound.tsx
        └── ServerError.tsx
```

### Configuration Navigation

```typescript
interface NavItem {
  path: string;
  label: string;
  icon: React.ComponentType;
  roles: Role[];
  badge?: () => number;  // Fonction pour badge dynamique
  children?: NavItem[];
}

const navConfig: NavItem[] = [
  { path: '/dashboard', label: 'Dashboard', icon: Home, roles: ['employee', 'manager', 'admin'] },
  { path: '/clock', label: 'Pointage', icon: Clock, roles: ['employee', 'manager', 'admin'] },
  { path: '/absences', label: 'Absences', icon: Calendar, roles: ['employee', 'manager', 'admin'] },
  { path: '/team', label: 'Mon équipe', icon: Users, roles: ['manager', 'admin'] },
  { path: '/validations', label: 'Validations', icon: CheckCircle, roles: ['manager', 'admin'], badge: getPendingCount },
  { path: '/reports', label: 'Rapports', icon: BarChart, roles: ['manager', 'admin'] },
  { path: '/users', label: 'Utilisateurs', icon: UserCog, roles: ['admin'] },
  { path: '/teams', label: 'Équipes', icon: Building, roles: ['admin'] },
  { path: '/settings', label: 'Paramètres', icon: Settings, roles: ['admin'] },
  { path: '/audit', label: 'Audit logs', icon: FileText, roles: ['admin'] },
];
```

### Breakpoints Tailwind

```javascript
// tailwind.config.js
module.exports = {
  theme: {
    screens: {
      'sm': '640px',   // Mobile landscape
      'md': '768px',   // Tablet
      'lg': '1024px',  // Desktop
      'xl': '1280px',  // Large desktop
      '2xl': '1536px', // Extra large
    },
  },
}
```

### Gestion des Erreurs API

```typescript
// Intercepteur Axios
api.interceptors.response.use(
  (response) => response,
  (error) => {
    const status = error.response?.status;
    const message = error.response?.data?.message;

    switch (status) {
      case 401:
        // Redirect to login
        authStore.logout();
        break;
      case 403:
        toast.error('Accès non autorisé');
        break;
      case 404:
        toast.error('Ressource non trouvée');
        break;
      case 422:
        // Validation errors handled by form
        break;
      case 500:
        toast.error('Erreur serveur. Veuillez réessayer.');
        break;
      default:
        toast.error(message || 'Une erreur est survenue');
    }

    return Promise.reject(error);
  }
);
```

### Critères d'Accessibilité

| Critère | Implémentation |
|---------|----------------|
| Contraste | Ratio minimum 4.5:1 (WCAG AA) |
| Navigation clavier | Tab order logique, focus visible |
| Screen readers | Labels ARIA, rôles sémantiques |
| Responsive | Zoom 200% sans perte de fonctionnalité |
| Animations | Respect prefers-reduced-motion |

### Design des Empty States

Les empty states sont essentiels pour guider l'utilisateur quand il n'y a pas de données. Chaque contexte a son propre message et CTA.

| Contexte | Message | Icône | CTA |
|----------|---------|-------|-----|
| Tableau utilisateurs vide | "Aucun utilisateur trouvé" | 👥 | "Créer un utilisateur" |
| Tableau absences vide | "Aucune absence à afficher" | 📅 | "Nouvelle demande" |
| Pointages du jour vides | "Pas encore de pointage" | ⏰ | "Pointer mon arrivée" |
| Équipe vide | "Aucun membre dans l'équipe" | 🏢 | "Ajouter des membres" |
| Validations vides | "Aucune validation en attente" | ✅ | - |
| Notifications vides | "Aucune notification" | 🔔 | - |

```tsx
// Composant EmptyState réutilisable
interface EmptyStateProps {
  icon: React.ReactNode;
  title: string;
  description?: string;
  action?: {
    label: string;
    onClick: () => void;
  };
}

<EmptyState
  icon={<UsersIcon className="h-12 w-12 text-muted-foreground" />}
  title="Aucun utilisateur trouvé"
  description="Commencez par créer votre premier utilisateur"
  action={{
    label: "Créer un utilisateur",
    onClick: () => navigate('/users/new')
  }}
/>
```
