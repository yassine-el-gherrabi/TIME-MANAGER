# Roadmap Docker & CI

> Feuille de route issue de l'analyse d'infrastructure du 11/01/2026

---

## Haute Priorité

- [ ] **Fix hardcoded passwords** - Remplacer `admin` par des variables d'environnement dans docker-compose.yml
- [ ] **Fix frontend Dockerfile.prod** - Simplifier le multi-stage inutile (stage dependencies contre-productif)
- [ ] **Add Trivy scan** - Ajouter scan de vulnérabilités images Docker en CI

---

## Priorité Moyenne

- [ ] **Add Docker Compose profiles** - Séparer dev/monitoring pour ne pas démarrer 12 conteneurs
- [ ] **Add E2E tests to CI** - Intégrer Playwright dans le workflow CI
- [ ] **Add CSP header** - Ajouter Content-Security-Policy dans nginx.conf
- [ ] **Fix Tempo root user** - Supprimer `user: "0:0"` et configurer les permissions

---

## Basse Priorité

- [ ] **Add OCI labels** - Ajouter métadonnées aux images Docker (org.opencontainers.image.*)
- [ ] **Unify backend Dockerfiles** - Supprimer la redondance Dockerfile/Dockerfile.prod (optionnel)

---

## Historique des Commits

| Date | Tâche | Commit |
|------|-------|--------|

