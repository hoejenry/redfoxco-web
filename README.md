# redfoxcohomes.com

Production website for Red Fox Co Homes — a veteran-owned home improvement company in Northern Virginia.

**Live site:** https://www.redfoxcohomes.com  
**Preview (WASM):** https://preview.redfoxcohomes.com

---

## Overview

The site runs in two configurations:

| | Current (Production) | In Development |
|---|---|---|
| Stack | WordPress + MySQL on Kubernetes | Rust + WebAssembly (Leptos) |
| Hosting | LKE (Linode Kubernetes Engine) | Same LKE cluster |
| Repo | `hoejenry/redfoxco` (k8s manifests) | `hoejenry/redfoxco-web` (this repo) |
| URL | www.redfoxcohomes.com | preview.redfoxcohomes.com |

---

## Repository layout

```
redfoxco-web/           ← this repo (WASM site source + deployment)
├── src/
│   └── main.rs         # Leptos app — all components in one file
├── index.html          # Trunk entry point + Tailwind CDN config
├── style.css           # Custom CSS (only what Tailwind can't handle)
├── Cargo.toml          # Rust deps: leptos 0.7 (CSR)
├── Dockerfile          # Multi-stage: rust:latest → nginx:alpine
├── nginx.conf          # SPA routing + WASM MIME + asset caching
├── .dockerignore
├── k8s/
│   ├── deployment.yaml       # 2-replica nginx deployment
│   ├── service.yaml          # ClusterIP service
│   ├── ingress-preview.yaml  # preview.redfoxcohomes.com (testing)
│   └── ingress-production.yaml # www.redfoxcohomes.com (cutover)
└── .github/
    └── workflows/
        └── deploy.yml  # CI: build image → push to ghcr.io → deploy to LKE

redfoxco/               ← separate repo (WordPress/MySQL k8s manifests)
├── kustomization.yaml
├── ingress.yaml
├── issuer.yaml
├── external-dns-values.yaml
├── patch-wordpress-resources.yaml
└── patch-mysql-resources.yaml
```

---

## Tech stack (WASM site)

| Layer | Technology |
|---|---|
| Language | Rust |
| Framework | [Leptos](https://leptos.dev) 0.7 (CSR mode) |
| Build tool | [Trunk](https://trunkrs.dev) 0.21.14 |
| CSS | Tailwind CSS (CDN) with custom `brand` color tokens |
| Assets | Inline static slices — images via `const` URLs |
| Deployment | Docker (nginx:alpine) on Kubernetes |

### Key Leptos patterns used

- `signal()` / `RwSignal` for reactive state (gallery tab, lightbox, contact form)
- `window_event_listener` for keyboard events (Escape closes lightbox)
- `spawn_local` (ready for async Formspree fetch)
- `move ||` closures capture signals by value

---

## Local development

**Prerequisites:** Rust stable, `wasm32-unknown-unknown` target, Trunk

```bash
# Install Rust target (once)
rustup target add wasm32-unknown-unknown

# Install Trunk (use pre-built binary — source build has known issues on macOS)
curl -LO https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-aarch64-apple-darwin.tar.gz
tar xzf trunk-aarch64-apple-darwin.tar.gz
mv trunk ~/.cargo/bin/

# Run dev server (hot reload)
trunk serve
```

Opens at **http://localhost:8080**

---

## Production build

```bash
trunk build --release
```

Output in `dist/` — static files (HTML, JS, WASM, assets). The WASM binary is optimized with `opt-level = "z"` + LTO.

---

## Deployment pipeline (CI/CD)

Every push to `master` triggers `.github/workflows/deploy.yml`:

```
push to master
    │
    ▼
Build Docker image
(rust:latest → trunk build --release → nginx:alpine)
    │
    ▼
Push to ghcr.io/hoejenry/redfoxco-web:latest + :<sha>
    │
    ▼
kubectl apply -f k8s/ (deployment, service, ingress-preview)
    │
    ▼
kubectl set image (rolls out the new SHA-tagged image)
    │
    ▼
kubectl rollout status (waits for healthy pods)
```

**Required GitHub secret:** `KUBECONFIG` — the LKE kubeconfig (Settings → Secrets → Actions)

Build time: ~10–15 min on first run (no Rust cache), ~3–5 min with cache.

---

## Kubernetes infrastructure

### Cluster

- **Provider:** Linode Kubernetes Engine (LKE)
- **Nodes:** 3
- **Region:** us-east (Newark)

### Components

| Component | Purpose |
|---|---|
| NGINX Ingress Controller | Routes external traffic to services |
| cert-manager | Auto-issues Let's Encrypt TLS certs |
| ExternalDNS | Syncs k8s Ingress hostnames → Linode DNS |
| WordPress + MySQL | Current production site |
| redfoxco-web | WASM site (preview) |

### Resource limits

| Workload | Memory request/limit | CPU request/limit |
|---|---|---|
| WordPress | 128Mi / 512Mi | 100m / 500m |
| MySQL | 256Mi / 512Mi | 250m / 500m |
| redfoxco-web (WASM) | 32Mi / 128Mi | 50m / 200m |

### DNS

- **Provider:** Linode DNS
- **Load balancer IP:** 172.239.156.150
- **ExternalDNS** manages A records automatically based on Ingress hostnames
- TTL: 30 seconds (fast propagation for cutover)

---

## Cutover procedure (WordPress → WASM)

When the WASM site is ready for production:

1. **Verify** the preview site at https://preview.redfoxcohomes.com looks correct
2. **Delete** the WordPress ingress:
   ```bash
   kubectl delete ingress wordpress-ingress
   ```
3. **Apply** the production WASM ingress:
   ```bash
   kubectl apply -f k8s/ingress-production.yaml
   ```
4. cert-manager issues a new TLS cert for `redfoxcohomes.com` and `www.redfoxcohomes.com`
5. ExternalDNS updates DNS records within ~30 seconds
6. Verify https://www.redfoxcohomes.com serves the WASM site

**Rollback:** Delete `redfoxco-web-ingress` and re-apply `redfoxco/ingress.yaml` from the WordPress repo.

**Scale down WordPress** (after confirming cutover is stable):
```bash
kubectl scale deployment wordpress --replicas=0
kubectl scale deployment wordpress-mysql --replicas=0
```

---

## WordPress / LKE operations

The WordPress site is managed via the `hoejenry/redfoxco` repo with kustomize.

### Deploy

```bash
cd redfoxco/
kubectl apply -k .
```

### Rotate MySQL password

The MySQL password is stored as a Kubernetes secret (`mysql-pass`) and set via GitHub Secret `MYSQL_PASSWORD` in CI.

**Important:** When rotating, update all three MySQL users:
```sql
ALTER USER 'root'@'localhost' IDENTIFIED BY 'NEW_PASS';
ALTER USER 'root'@'%'         IDENTIFIED BY 'NEW_PASS';
ALTER USER 'wordpress'@'%'    IDENTIFIED BY 'NEW_PASS';
```

Update the secret on the cluster:
```bash
kubectl create secret generic mysql-pass \
  --from-literal=password="$NEW_PASS" \
  --dry-run=client -o yaml | kubectl apply -f -
kubectl rollout restart deployment/wordpress
```

### Known gotchas

- **Shell special characters in passwords** — never pass passwords with `&`, `!`, `$` directly to shell commands; use `--from-literal` with double quotes or base64 encode first
- **MySQL multi-host users** — `root@localhost` and `root@%` are separate users; both must be updated when rotating passwords
- **Secret name** — the secret is `mysql-pass` (plain, not kustomize-hashed); secretGenerator is intentionally removed from `kustomization.yaml`

---

## Site content (WASM)

The site is a single-page app with these sections:

| Section | Notes |
|---|---|
| Nav | Hamburger menu on mobile, hover dropdown on desktop |
| Hero | Full-viewport photo background, dark overlay, logo + CTA |
| Trust bar | Veteran-owned, licensed/insured, 5-star callouts |
| Services | 6 service cards (Kitchen, Bath, Flooring, Painting, Outdoor, General) |
| Gallery | 5 tabs (All, Kitchen, Bath, Flooring, Exterior) with lightbox |
| Contact | Form → Formspree (pending) |
| Footer | Contact info + social links |

### Color tokens

| Token | Hex | Usage |
|---|---|---|
| `brand-700` | `#c2410c` | Primary buttons, active states |
| `brand-800` | `#9a3412` | Button hover |

---

## Pending work

- [ ] Wire up Formspree for the contact form (form ID needed from formspree.io)
- [ ] Confirm CI build succeeds end-to-end
- [ ] Test preview.redfoxcohomes.com after first successful deploy
- [ ] Schedule cutover from WordPress to WASM
