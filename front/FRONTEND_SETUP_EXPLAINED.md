# 📚 Explication Détaillée : Setup Frontend React (US-010)

## 🎯 Contexte du Projet Time Manager

Le **Time Manager** est une application de gestion du temps pour employés et managers avec :
- **Backend** : API RESTful (Go/Elixir Phoenix dans votre cas)
- **Frontend** : Application web React.js
- **Fonctionnalités** : Clock in/out, gestion d'équipes, dashboards, KPIs
- **Rôles** : Employee (employé) et Manager
- **Authentification** : JWT tokens
- **DevOps** : Docker, CI/CD avec GitHub Actions

---

## 🏗️ Ce que nous avons construit (US-010)

### 1. **Technologies Choisies et Pourquoi**

#### ✅ React.js (choisi parmi React/Vue/Angular)
**Pourquoi React ?**
- **Écosystème mature** : Énorme communauté, nombreuses librairies
- **Performance** : Virtual DOM optimisé pour les interfaces dynamiques
- **Flexibilité** : Pas d'opinions fortes, s'adapte à tous types d'architectures
- **DevTools** : Excellents outils de debugging (React DevTools)
- **Employabilité** : Très demandé sur le marché du travail

#### ✅ Create React App (CRA)
**Pourquoi CRA ?**
- **Setup rapide** : Configuration Webpack/Babel préconfigurée
- **Best practices** : Configuration optimisée pour la production
- **Maintenance** : Mise à jour simplifiée des dépendances
- **Focus développement** : Permet de se concentrer sur le code métier

#### ✅ React Router DOM v6
**Pourquoi React Router ?**
- **Standard de facto** : Solution la plus utilisée pour le routing React
- **Déclaratif** : Routes définies comme des composants React
- **Navigation programmatique** : Redirections depuis le code
- **Protected routes** : Facilite la gestion des routes authentifiées (pour Employee/Manager)
- **Nested routes** : Parfait pour les dashboards avec sous-sections

**Cas d'usage Time Manager :**
```jsx
// Exemple de routes prévues
<Routes>
  <Route path="/login" element={<Login />} />
  <Route path="/dashboard" element={<ProtectedRoute><Dashboard /></ProtectedRoute>}>
    <Route path="clock" element={<ClockInOut />} />
    <Route path="worktime" element={<WorkingTimes />} />
  </Route>
  <Route path="/admin" element={<ManagerRoute><AdminPanel /></ManagerRoute>} />
</Routes>
```

#### ✅ Axios
**Pourquoi Axios et pas fetch ?**
- **Intercepteurs** : Permet d'ajouter automatiquement le JWT token à chaque requête
- **Timeout** : Gestion du timeout (10s configuré)
- **Transformation** : Conversion automatique JSON
- **Annulation** : Possibilité d'annuler les requêtes
- **Compatibilité** : Fonctionne aussi côté Node.js

**Configuration pour Time Manager :**
```javascript
// Intercepteur Request : Ajoute le JWT automatiquement
axiosInstance.interceptors.request.use(config => {
  const token = localStorage.getItem('authToken');
  if (token) config.headers.Authorization = `Bearer ${token}`;
  return config;
});

// Intercepteur Response : Gère les erreurs globalement
axiosInstance.interceptors.response.use(
  response => response,
  error => {
    if (error.response.status === 401) {
      // Token expiré → redirection login
      localStorage.removeItem('authToken');
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);
```

#### ✅ Tailwind CSS
**Pourquoi Tailwind et pas Bootstrap/Material UI ?**
- **Utility-first** : Classes utilitaires, pas de CSS personnalisé
- **Customisation** : Thème entièrement personnalisable
- **Performance** : PurgeCSS supprime le CSS non utilisé en production
- **Responsive** : Breakpoints intégrés (sm:, md:, lg:, xl:, 2xl:)
- **Dark mode** : Support natif du thème sombre
- **Productivité** : Pas besoin de nommer les classes, développement rapide

**Configuration Time Manager :**
```js
// tailwind.config.js
module.exports = {
  content: ['./src/**/*.{js,jsx}'], // Scan tous les fichiers
  theme: {
    extend: {
      colors: {
        // Variables CSS pour thème dynamique
        primary: 'hsl(var(--primary))',
        background: 'hsl(var(--background))',
        // ... autres couleurs
      }
    }
  }
}
```

#### ✅ Magic UI CLI
**Pourquoi Magic UI ?**
- **Composants modernes** : Basé sur Radix UI (accessible) + Tailwind
- **Animations** : Intégration avec Framer Motion
- **Accessibilité** : Composants WCAG compliant (requis par le projet)
- **Customisation** : Source code modifiable
- **Production-ready** : Composants testés et optimisés

**Utilisation prévue :**
- Formulaires (login, ajout employé, création d'équipe)
- Modales (confirmation suppression, édition)
- Tables (liste des employés, working times)
- Charts (dashboards KPI avec graphiques)

#### ✅ React Hook Form + Yup
**Pourquoi React Hook Form ?**
- **Performance** : Minimise les re-renders (important pour les gros formulaires)
- **Validation** : Intégration native avec Yup
- **API simple** : Hook `useForm()` facile à utiliser
- **TypeScript ready** : Support TypeScript excellent

**Pourquoi Yup ?**
- **Schémas déclaratifs** : Validation lisible et maintenable
- **Async validation** : Vérifie email unique côté API
- **Messages personnalisés** : Traduction facile (FR/EN)
- **Chainable** : `.required()`, `.email()`, `.min()`, etc.

**Exemple Time Manager :**
```javascript
const userSchema = yup.object({
  email: yup.string()
    .email('Email invalide')
    .required('Email obligatoire'),
  firstName: yup.string()
    .required('Prénom obligatoire'),
  phoneNumber: yup.string()
    .matches(/^[0-9]{10}$/, 'Format invalide')
});

const { register, handleSubmit, errors } = useForm({
  resolver: yupResolver(userSchema)
});
```

#### ✅ date-fns
**Pourquoi date-fns et pas Moment.js ?**
- **Léger** : 20KB vs 230KB pour Moment.js
- **Immutable** : Pas de mutations (moins de bugs)
- **Tree-shakeable** : Import seulement ce dont on a besoin
- **TypeScript** : Excellente intégration TypeScript
- **i18n** : Support multilingue intégré

**Cas d'usage Time Manager :**
```javascript
import { format, differenceInHours, parseISO } from 'date-fns';
import { fr } from 'date-fns/locale';

// Affichage working times
const displayTime = format(parseISO(workingTime.start),
  'dd/MM/yyyy HH:mm', { locale: fr }
);

// Calcul heures travaillées
const hoursWorked = differenceInHours(
  parseISO(workingTime.end),
  parseISO(workingTime.start)
);
```

---

### 2. **Architecture des Dossiers**

#### 📁 Structure Professionnelle

```
src/
├── api/                    # Services API & Axios
│   └── axiosInstance.js    # Configuration Axios avec intercepteurs
│
├── components/             # Composants React
│   ├── common/            # Boutons, inputs, modales (réutilisables)
│   ├── layout/            # Header, Sidebar, Footer (structure page)
│   └── features/          # Composants métier spécifiques
│       ├── auth/          # Login, Register
│       ├── users/         # UserList, UserForm
│       ├── teams/         # TeamList, TeamForm
│       ├── clock/         # ClockInOut
│       └── dashboard/     # DashboardEmployee, DashboardManager
│
├── contexts/              # React Context (state global)
│   ├── AuthContext.jsx    # User connecté, login/logout
│   └── ThemeContext.jsx   # Dark/Light mode
│
├── hooks/                 # Custom Hooks
│   ├── useAuth.js         # Hook pour authentification
│   ├── useApi.js          # Hook pour appels API
│   └── useForm.js         # Hook pour formulaires
│
├── pages/                 # Pages complètes (= Routes)
│   ├── LoginPage.jsx
│   ├── DashboardPage.jsx
│   ├── UsersPage.jsx
│   ├── TeamsPage.jsx
│   └── ClockPage.jsx
│
├── routes/                # Configuration routing
│   ├── index.jsx          # Routes principales
│   ├── ProtectedRoute.jsx # HOC pour routes authentifiées
│   └── ManagerRoute.jsx   # HOC pour routes manager only
│
├── services/              # Logique métier
│   ├── authService.js     # login(), logout(), refreshToken()
│   ├── userService.js     # CRUD users
│   ├── teamService.js     # CRUD teams
│   └── clockService.js    # clock in/out, get working times
│
├── utils/                 # Fonctions utilitaires
│   ├── formatDate.js      # Helpers date-fns
│   ├── validators.js      # Validations custom
│   └── constants.js       # Constantes (roles, statuts)
│
├── config/                # Configurations
│   ├── api.config.js      # Endpoints API
│   ├── routes.config.js   # Chemins des routes
│   └── app.config.js      # Constantes app (formats date, pagination)
│
└── styles/                # Styles globaux
    └── globals.css        # Reset CSS, variables Tailwind
```

**Pourquoi cette organisation ?**

1. **Séparation des responsabilités** : Chaque dossier a un rôle précis
2. **Scalabilité** : Facile d'ajouter de nouvelles features
3. **Maintenabilité** : Structure claire pour onboarding nouveaux devs
4. **Testabilité** : Logique métier isolée dans services/
5. **Réutilisabilité** : Composants communs mutualisés

---

### 3. **Configuration Tailwind CSS avec Magic UI**

#### 🎨 Thème avec Variables CSS

```css
/* src/index.css */
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  :root {
    /* Couleurs en HSL pour manipulation facile */
    --primary: 221.2 83.2% 53.3%;      /* Bleu principal */
    --background: 0 0% 100%;            /* Blanc */
    --foreground: 222.2 84% 4.9%;      /* Texte sombre */
    --border: 214.3 31.8% 91.4%;       /* Bordures */
    /* ... autres couleurs */
  }

  .dark {
    /* Thème sombre */
    --primary: 217.2 91.2% 59.8%;
    --background: 222.2 84% 4.9%;
    --foreground: 210 40% 98%;
    /* ... */
  }
}
```

**Avantages :**
- **Thème dynamique** : Switch dark/light sans recharger
- **Consistance** : Même palette partout
- **Accessibilité** : Contraste optimisé

#### 📐 Border Radius Personnalisés

```js
// tailwind.config.js
borderRadius: {
  lg: 'var(--radius)',           // 0.5rem (8px)
  md: 'calc(var(--radius) - 2px)', // 6px
  sm: 'calc(var(--radius) - 4px)'  // 4px
}
```

**Utilisation :**
```jsx
<button className="rounded-lg">  {/* 8px */}
<input className="rounded-md">   {/* 6px */}
```

---

### 4. **Configuration Axios & API**

#### 🔌 Instance Axios Configurée

```javascript
// src/api/axiosInstance.js
const axiosInstance = axios.create({
  baseURL: process.env.REACT_APP_API_URL, // http://localhost:8080 en dev
  timeout: 10000, // 10 secondes max par requête
  headers: {
    'Content-Type': 'application/json'
  }
});
```

**Pourquoi cette config ?**
- **baseURL depuis .env** : Différent selon dev/prod
- **Timeout 10s** : Évite les requêtes infinies
- **Headers par défaut** : JSON pour toutes les requêtes

#### 🔐 Intercepteur Request (JWT)

```javascript
axiosInstance.interceptors.request.use(
  (config) => {
    // Récupère le token depuis localStorage
    const token = localStorage.getItem('authToken');

    if (token) {
      // Ajoute le header Authorization automatiquement
      config.headers.Authorization = `Bearer ${token}`;
    }

    // Log en développement
    if (process.env.REACT_APP_ENV === 'development') {
      console.log(`[API Request] ${config.method?.toUpperCase()} ${config.url}`, config);
    }

    return config;
  },
  (error) => Promise.reject(error)
);
```

**Flow authentification :**
1. User se login → API retourne JWT token
2. Frontend stocke token dans `localStorage.setItem('authToken', token)`
3. **Toutes** les requêtes suivantes ont automatiquement `Authorization: Bearer <token>`
4. Plus besoin de gérer le token manuellement !

#### ⚠️ Intercepteur Response (Gestion Erreurs)

```javascript
axiosInstance.interceptors.response.use(
  (response) => response, // Succès : on retourne tel quel
  (error) => {
    // Gestion erreurs globales
    switch (error.response?.status) {
      case 401: // Unauthorized
        // Token expiré ou invalide → logout automatique
        localStorage.removeItem('authToken');
        window.location.href = '/login';
        break;

      case 403: // Forbidden
        // User n'a pas les permissions (ex: employee accède route manager)
        console.error('Access forbidden:', error.response.data);
        break;

      case 404: // Not Found
        console.error('Resource not found:', error.response.data);
        break;

      case 500: // Server Error
        console.error('Server error:', error.response.data);
        break;
    }

    return Promise.reject(error);
  }
);
```

**Avantages :**
- **Gestion centralisée** : Un seul endroit pour gérer les erreurs
- **Logout automatique** : Si token expiré, redirection login
- **Logs structurés** : Facilite le debugging

---

### 5. **Fichiers de Configuration**

#### 📝 api.config.js (Endpoints)

```javascript
export const API_ENDPOINTS = {
  AUTH: {
    LOGIN: '/auth/login',
    REGISTER: '/auth/register',
    LOGOUT: '/auth/logout',
    REFRESH_TOKEN: '/auth/refresh',
    ME: '/auth/me'
  },

  USERS: {
    LIST: '/users',
    DETAIL: (id) => `/users/${id}`,
    CREATE: '/users',
    UPDATE: (id) => `/users/${id}`,
    DELETE: (id) => `/users/${id}`
  },

  // ... TEAMS, WORKING_TIMES, CLOCKS
};
```

**Utilisation dans services :**
```javascript
// src/services/userService.js
import axiosInstance from '@/api/axiosInstance';
import { API_ENDPOINTS } from '@/config/api.config';

export const userService = {
  getAll: () => axiosInstance.get(API_ENDPOINTS.USERS.LIST),
  getById: (id) => axiosInstance.get(API_ENDPOINTS.USERS.DETAIL(id)),
  create: (data) => axiosInstance.post(API_ENDPOINTS.USERS.CREATE, data),
  update: (id, data) => axiosInstance.put(API_ENDPOINTS.USERS.UPDATE(id), data),
  delete: (id) => axiosInstance.delete(API_ENDPOINTS.USERS.DELETE(id))
};
```

**Avantages :**
- **Centralisation** : Tous les endpoints au même endroit
- **Maintenance** : Changement d'endpoint = 1 seul fichier
- **Type-safety** : Fonctions avec paramètres évitent les erreurs

#### 🛣️ routes.config.js (Chemins Routes)

```javascript
export const ROUTES = {
  HOME: '/',
  LOGIN: '/login',
  REGISTER: '/register',
  DASHBOARD: '/dashboard',
  PROFILE: '/profile',
  USERS: '/users',
  USER_DETAIL: '/users/:id',
  WORKING_TIMES: '/working-times',
  CLOCKS: '/clocks',
  NOT_FOUND: '*'
};
```

**Utilisation :**
```javascript
import { ROUTES } from '@/config/routes.config';

// Navigation programmatique
navigate(ROUTES.DASHBOARD);

// Protected route
<Route path={ROUTES.DASHBOARD} element={<ProtectedRoute><Dashboard /></ProtectedRoute>} />
```

#### ⚙️ app.config.js (Constantes App)

```javascript
export const APP_CONFIG = {
  APP_NAME: 'Time Manager',
  APP_VERSION: '1.0.0',

  PAGINATION: {
    DEFAULT_PAGE_SIZE: 10,
    PAGE_SIZE_OPTIONS: [10, 25, 50, 100]
  },

  DATE_FORMATS: {
    DISPLAY: 'dd/MM/yyyy',          // Affichage français
    DISPLAY_TIME: 'dd/MM/yyyy HH:mm',
    API: 'yyyy-MM-dd',              // Format ISO pour API
    API_TIME: "yyyy-MM-dd'T'HH:mm:ss"
  },

  STORAGE_KEYS: {
    AUTH_TOKEN: 'authToken',
    USER_DATA: 'userData',
    THEME: 'theme'
  },

  ROLES: {
    ADMIN: 'admin',
    MANAGER: 'manager',
    EMPLOYEE: 'employee'
  }
};
```

**Utilisation :**
```javascript
import { APP_CONFIG } from '@/config/app.config';

// Pagination
const [pageSize] = useState(APP_CONFIG.PAGINATION.DEFAULT_PAGE_SIZE);

// Format date
const displayDate = format(date, APP_CONFIG.DATE_FORMATS.DISPLAY);

// Rôles
if (user.role === APP_CONFIG.ROLES.MANAGER) {
  // Afficher features manager
}
```

---

### 6. **ESLint & Prettier (Quality Tools)**

#### 🔍 ESLint Configuration

```json
{
  "root": true,
  "parser": "@typescript-eslint/parser",
  "plugins": ["@typescript-eslint", "react", "react-hooks"],
  "extends": [
    "eslint:recommended",
    "plugin:react/recommended",
    "plugin:react-hooks/recommended",
    "prettier"
  ],
  "rules": {
    "react/react-in-jsx-scope": "off", // React 17+ pas besoin d'import React
    "react/jsx-uses-react": "off"
  }
}
```

**Qu'est-ce que ESLint ?**
- **Linter JavaScript** : Détecte les erreurs de code
- **Best practices** : Force l'utilisation de bonnes pratiques
- **Hooks** : Vérifie les règles des React Hooks
- **Consistance** : Code uniforme dans toute l'équipe

**Exemples d'erreurs détectées :**
- `useState` utilisé dans une condition (interdit)
- Variables non utilisées
- Code mort (unreachable)
- Props manquantes

#### 🎨 Prettier Configuration

```json
{
  "singleQuote": true,      // 'string' au lieu de "string"
  "semi": true,             // Point-virgule obligatoire
  "trailingComma": "all",   // Virgule finale partout
  "printWidth": 100         // Max 100 caractères par ligne
}
```

**Qu'est-ce que Prettier ?**
- **Formateur de code** : Formate automatiquement
- **Opinionated** : Pas de débat sur le style
- **Intégration** : Fonctionne avec ESLint

**Scripts package.json :**
```json
{
  "scripts": {
    "lint": "eslint . --ext .js,.jsx --max-warnings=0",
    "lint:fix": "npm run lint -- --fix",
    "format": "prettier --check .",
    "format:fix": "prettier --write ."
  }
}
```

---

### 7. **Environnements (.env)**

#### 🌍 .env.development (Développement Local)

```env
REACT_APP_API_URL=http://localhost:8080
REACT_APP_ENV=development
```

**Utilisation :**
```javascript
// Axios pointe sur backend local
const api = axios.create({
  baseURL: process.env.REACT_APP_API_URL // http://localhost:8080
});

// Logs actifs seulement en dev
if (process.env.REACT_APP_ENV === 'development') {
  console.log('Debug info:', data);
}
```

#### 🚀 .env.production (Production)

```env
REACT_APP_API_URL=/api
REACT_APP_ENV=production
```

**Pourquoi `/api` ?**
- En production, le reverse proxy (KrakenD/Nginx) route `/api` → backend
- Frontend et backend sur le même domaine → pas de CORS

**Architecture production :**
```
User → https://timemanager.com
       ├── /          → Frontend (React build statique)
       └── /api/*     → Backend (API Go/Elixir)
```

---

### 8. **Imports Absolus (jsconfig.json)**

#### ⚡ Configuration

```json
{
  "compilerOptions": {
    "baseUrl": "src",
    "paths": {
      "@/*": ["./*"],
      "@components/*": ["components/*"],
      "@pages/*": ["pages/*"],
      "@hooks/*": ["hooks/*"],
      "@api/*": ["api/*"],
      "@config/*": ["config/*"],
      "@utils/*": ["utils/*"]
    }
  }
}
```

#### ❌ AVANT (imports relatifs)

```javascript
import Button from '../../../components/common/Button';
import { userService } from '../../../../services/userService';
import { ROUTES } from '../../../../../config/routes.config';
```

**Problèmes :**
- Difficile à lire
- Fragile (si on déplace un fichier, tout casse)
- Erreurs fréquentes

#### ✅ APRÈS (imports absolus)

```javascript
import Button from '@components/common/Button';
import { userService } from '@/services/userService';
import { ROUTES } from '@config/routes.config';
```

**Avantages :**
- Lisibilité maximale
- Refactoring facile
- Auto-complétion IDE meilleure

---

### 9. **CI/CD - GitHub Actions**

#### 🔄 Workflow React Lint

```yaml
react-lint:
  name: React Lint
  runs-on: ubuntu-latest
  defaults:
    run:
      working-directory: front
  steps:
    - name: Checkout
      uses: actions/checkout@v4

    - name: Setup Node
      uses: actions/setup-node@v4
      with:
        node-version: '20'
        cache: 'npm'
        cache-dependency-path: front/package-lock.json

    - name: Install deps
      run: npm ci

    - name: ESLint
      run: npm run lint

    - name: Prettier
      run: npm run format
```

**Qu'est-ce que ça fait ?**

1. **À chaque push/PR sur master** :
   - Clone le repo
   - Installe Node.js 20
   - Installe les dépendances (`npm ci`)
   - Lance ESLint (`npm run lint`)
   - Lance Prettier (`npm run format`)

2. **Si échec** :
   - ❌ La CI est rouge
   - Impossible de merger la PR
   - Notification sur GitHub

3. **Si succès** :
   - ✅ La CI est verte
   - Code validé
   - Merge autorisé

**Pourquoi c'est important ?**
- **Qualité garantie** : Code toujours lint/formaté
- **Automatisation** : Pas besoin de penser à lancer manuellement
- **Team work** : Même qualité pour tous les devs

---

## 🚀 Prochaines Étapes (Après US-010)

### US suivantes prévues :

1. **US-011 : Authentification JWT**
   - Login/Register pages
   - AuthContext avec React Context
   - Protected routes (Employee/Manager)
   - Refresh token automatique

2. **US-012 : Dashboard Employee**
   - Clock In/Out component
   - Liste des working times
   - Graphique heures travaillées
   - Profil utilisateur

3. **US-013 : CRUD Users (Manager)**
   - Liste employés avec pagination
   - Formulaire ajout/édition user
   - Suppression avec confirmation
   - Filtres et recherche

4. **US-014 : CRUD Teams (Manager)**
   - Liste équipes
   - Formulaire création team
   - Assignation membres
   - Vue détails team

5. **US-015 : Dashboard Manager**
   - KPIs équipe (heures moyennes, retards)
   - Graphiques comparatifs
   - Export rapports
   - Vue employés par team

6. **US-016 : Tests E2E**
   - Tests Cypress pour flows critiques
   - Tests accessibilité (a11y)
   - Tests performance

---

## 📊 Résumé Technique

### Dépendances Installées

| Package | Version | Usage |
|---------|---------|-------|
| react | 19.2.0 | Framework UI |
| react-router-dom | 6.x | Routing |
| axios | latest | HTTP client |
| tailwindcss | latest | CSS utility-first |
| magicui-cli | 0.1.6 | UI components |
| framer-motion | latest | Animations |
| react-hook-form | latest | Forms |
| yup | latest | Validation |
| date-fns | latest | Date utilities |

### Scripts Disponibles

```bash
# Développement
npm start              # Lance dev server (port 3000)

# Build
npm run build          # Build production

# Quality
npm run lint           # Vérifie ESLint
npm run lint:fix       # Corrige ESLint
npm run format         # Vérifie Prettier
npm run format:fix     # Corrige Prettier

# Tests
npm test               # Lance tests unitaires
```

### Variables d'Environnement

| Variable | Dev | Prod |
|----------|-----|------|
| REACT_APP_API_URL | http://localhost:8080 | /api |
| REACT_APP_ENV | development | production |

---

## 💡 Conseils pour la Suite

### 🎯 Bonnes Pratiques React

1. **Composants fonctionnels uniquement** : Pas de class components
2. **Hooks customs** : Mutualiser la logique réutilisable
3. **PropTypes** : Documenter les props (ou TypeScript)
4. **Lazy loading** : `React.lazy()` pour les routes
5. **Memo** : `React.memo()` pour éviter re-renders inutiles

### 🔒 Sécurité

1. **JWT** : Ne jamais stocker de données sensibles dedans
2. **XSS** : Toujours sanitize les inputs utilisateur
3. **CSRF** : Token CSRF si pas de JWT
4. **HTTPS** : Obligatoire en production
5. **Validation** : Double validation (frontend + backend)

### ⚡ Performance

1. **Code splitting** : Routes lazy loaded
2. **Image optimization** : WebP, lazy loading images
3. **Bundle analyzer** : Vérifier taille du bundle
4. **Caching** : Service Worker pour PWA
5. **Compression** : Gzip/Brotli en production

### ♿ Accessibilité (a11y)

1. **Sémantique HTML** : `<button>` pas `<div onClick>`
2. **ARIA labels** : Pour screen readers
3. **Keyboard navigation** : Tab, Enter, Escape
4. **Contraste** : WCAG AA minimum (4.5:1)
5. **Focus visible** : Outline pour navigation clavier

---

## 📚 Ressources

- [React Docs](https://react.dev)
- [React Router Docs](https://reactrouter.com)
- [Tailwind CSS Docs](https://tailwindcss.com)
- [Axios Docs](https://axios-http.com)
- [React Hook Form](https://react-hook-form.com)
- [Yup Validation](https://github.com/jquense/yup)
- [date-fns Docs](https://date-fns.org)

---

**Auteur** : Claude Code (Assistant IA)
**Date** : 10 Octobre 2025
**Version** : 1.0
