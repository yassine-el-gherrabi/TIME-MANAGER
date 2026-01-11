# Roadmap Docker & CI

> Feuille de route issue de l'analyse d'infrastructure du 11/01/2026

---

## Haute Priorité

- [x] **Fix hardcoded passwords** - Remplacer `admin` par des variables d'environnement dans docker-compose.yml
- [x] **Fix frontend Dockerfile.prod** - Simplifier le multi-stage inutile (stage dependencies contre-productif)
- [x] **Add Trivy scan** - Ajouter scan de vulnérabilités images Docker en CI

---

## Priorité Moyenne

- [x] **Add Docker Compose profiles** - Séparer dev/monitoring pour ne pas démarrer 12 conteneurs
- [x] **Add E2E tests to CI** - Intégrer Playwright dans le workflow CI
- [x] **Add CSP header** - Ajouter Content-Security-Policy dans nginx.conf
- [x] **Fix Tempo root user** - Supprimer `user: "0:0"` et configurer les permissions

---

## Basse Priorité

- [x] **Add OCI labels** - Ajouter métadonnées aux images Docker (org.opencontainers.image.*)
- [x] **Unify backend Dockerfiles** - Supprimer la redondance Dockerfile/Dockerfile.prod (optionnel)

---

## Historique des Commits

| Date | Tâche | Commit |
|------|-------|--------|
| 11/01/2026 | Fix hardcoded passwords | `3d1de2d` |
| 11/01/2026 | Fix frontend Dockerfile.prod | `1c0ebef` |
| 11/01/2026 | Add Trivy scan | `ce15791` |
| 11/01/2026 | Add Docker Compose profiles | `7164ce1` |
| 11/01/2026 | Add E2E tests to CI | `56c6d5e` |
| 11/01/2026 | Add CSP header | `9b892cd` |
| 11/01/2026 | Fix Tempo root user | `1b44e34` |
| 11/01/2026 | Add OCI labels | `4ec6ae1` |
| 11/01/2026 | Unify backend Dockerfiles | `1c0e797` |

