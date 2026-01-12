# Glossaire

> Termes techniques utilisés dans la documentation

---

## A

**Access Token**
: JWT de courte durée (15 min) utilisé pour authentifier les requêtes API.

**Argon2id**
: Algorithme de hashing de mots de passe résistant au GPU et side-channel attacks.

**Audit Log**
: Journal des actions sensibles (login, changements de données, etc.).

---

## B

**Bearer Token**
: Schéma d'authentification HTTP où le token est passé dans le header Authorization.

**Brute Force Protection**
: Mécanisme de verrouillage de compte après plusieurs tentatives échouées.

---

## C

**CORS (Cross-Origin Resource Sharing)**
: Mécanisme de sécurité contrôlant l'accès cross-domain aux ressources.

**CSRF (Cross-Site Request Forgery)**
: Type d'attaque où un site malveillant exécute des actions au nom de l'utilisateur.

**Clock Entry**
: Enregistrement d'un pointage (arrivée ou départ).

---

## D

**Diesel**
: ORM Rust utilisé pour les interactions avec PostgreSQL.

**Docker Compose**
: Outil d'orchestration de containers Docker multi-services.

---

## G

**GHCR (GitHub Container Registry)**
: Registry Docker de GitHub pour stocker les images.

**Grafana**
: Plateforme de visualisation de métriques et logs.

---

## H

**HIBP (Have I Been Pwned)**
: Service permettant de vérifier si un mot de passe a été compromis.

**HttpOnly Cookie**
: Cookie inaccessible depuis JavaScript, protégeant contre le vol via XSS.

---

## J

**JWT (JSON Web Token)**
: Standard de token signé pour l'authentification stateless.

---

## K

**KPI (Key Performance Indicator)**
: Indicateur de performance (heures travaillées, taux de présence, etc.).

---

## L

**Loki**
: Système d'agrégation de logs de Grafana Labs.

---

## M

**Multi-tenant**
: Architecture permettant à plusieurs organisations de partager la même instance.

**Middleware**
: Fonction interceptant les requêtes avant/après les handlers.

---

## O

**OpenAPI**
: Spécification standard pour documenter les APIs REST.

**Organization**
: Entité de niveau supérieur regroupant utilisateurs, équipes et données.

**OTLP (OpenTelemetry Protocol)**
: Protocole standard pour l'export de traces et métriques.

---

## P

**Prometheus**
: Système de monitoring et alerting basé sur des time series.

**Promtail**
: Agent de collecte de logs pour Loki.

---

## R

**RBAC (Role-Based Access Control)**
: Système de contrôle d'accès basé sur les rôles utilisateur.

**Refresh Token**
: Token de longue durée (7 jours) permettant d'obtenir de nouveaux access tokens.

**RS256**
: Algorithme de signature JWT utilisant RSA avec SHA-256.

---

## S

**SameSite Cookie**
: Attribut de cookie limitant son envoi aux requêtes same-origin.

**Session**
: Instance de connexion d'un utilisateur, associée à un refresh token.

---

## T

**Tempo**
: Backend de tracing distribué de Grafana Labs.

**Traefik**
: Reverse proxy et load balancer moderne.

---

## V

**Vite**
: Build tool et dev server pour applications frontend modernes.

---

## Z

**Zustand**
: Bibliothèque de state management pour React.
