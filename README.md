---
title: Chamira Hashan Portfolio
emoji: 💜
colorFrom: purple
colorTo: pink
sdk: docker
pinned: false
short_description: Rust React portfolio with local H7 Assistant
---

# Chamira Hashan Personal Portfolio

A professional full-stack personal portfolio website for **Chamira Hashan**, built to showcase backend, AI/ML, full-stack, mobile, and software engineering projects.

The project includes a public portfolio website, a protected admin panel for managing portfolio data, media upload support, and a local H7 Assistant chatbot that answers from portfolio JSON data without using external AI inference credits.

---

## Live Portfolio

Primary public portfolio:

```text
https://chamirahashan.tech
```

Backend / Hugging Face Space:

```text
https://hashan-7-chamira-hashan.hf.space
```

Health check:

```text
https://hashan-7-chamira-hashan.hf.space/health
```

---

## Overview

This portfolio is designed as a dynamic portfolio system instead of a static hardcoded website.

The public website renders profile details, projects, skills, certificates, education, social links, and CV access from backend-managed JSON data. The admin panel allows the portfolio owner to update profile content, project details, certificates, education entries, links, and media paths without editing frontend source code.

The H7 Assistant is a local portfolio assistant. It answers common portfolio questions using the stored profile data, such as project details, skills, certificates, education, contact links, and overall profile summary. It does not call Hugging Face Router or external AI inference providers.

---

## Current Production Architecture

The primary production path is a single Hugging Face Docker Space. The image builds the React application and Rust backend, then the backend serves the compiled SPA, API, and public media from one origin.

```text
Custom domain / Hugging Face Space URL
                    ↓
Hugging Face Docker Space
  ├── React SPA served by Axum
  ├── Rust API and H7 Assistant
  └── persistent filesystem at /data when Space storage is enabled
```

Primary public URLs:

```text
https://chamirahashan.tech
https://hashan-7-chamira-hashan.hf.space
```

Because the browser and API share an origin, the production frontend can use an empty API base URL:

```text
VITE_API_BASE_URL=
```

Cloudflare Pages remains an optional deployment mode. When used, set `VITE_API_BASE_URL` to the Space URL and add the exact frontend origin to `ALLOWED_ORIGINS`.

---

## Key Features

- Premium dark purple responsive portfolio UI
- Mobile-first home section layout
- Dynamic profile, project, skills, certificates, and education rendering
- Project carousel with typed description animation
- Project image and video preview support
- Certificate display
- Skills ticker animation
- Social links and CV access
- Floating H7 Assistant chatbot popup
- Local JSON-based portfolio assistant replies
- Protected admin panel with authentication
- Configurable 5-60 minute admin session expiry
- Admin-managed JSON profile data
- Media upload support for project/profile assets
- Rust Axum backend
- React TypeScript Vite frontend
- Same-origin Docker deployment on Hugging Face Spaces
- Optional Cloudflare Pages frontend deployment
- Persistent Hugging Face storage support
- Bounded per-client and global API rate limiting
- CORS origin restriction
- Request IDs, timeouts, concurrency limits, and security headers

---

## Tech Stack

### Backend

- Rust
- Axum
- Tokio
- Serde
- Serde JSON
- Tower HTTP
- Utoipa-generated OpenAPI schema
- Validated JSON Web Token authentication
- Argon2id admin-password verification
- Bounded in-memory API rate limiting
- Hugging Face Space Docker runtime

### Frontend

- React
- TypeScript
- Vite
- CSS
- Responsive UI design
- Axum static serving with optional Cloudflare Pages hosting

### Chatbot

- Local JSON-based portfolio assistant
- No external AI model call required
- No Hugging Face Inference Provider credit usage
- Portfolio-data-only answers
- Simple guardrails for out-of-scope questions

### Storage

- JSON profile data
- Local `./data` fallback for development
- Hugging Face Space persistent storage at `/data`, when provisioned
- Media folders for uploaded images and videos

---

## Project Structure

```text
portfolio/
├── backend/
│   ├── Cargo.toml
│   └── src/
│       ├── admin.rs
│       ├── auth.rs
│       ├── error.rs
│       ├── media.rs
│       ├── portfolio_bot.rs
│       ├── profile.rs
│       ├── rate_limit.rs
│       ├── routes.rs
│       ├── state.rs
│       └── storage.rs
├── frontend/
│   ├── public/
│   │   ├── _headers
│   │   ├── _redirects
│   │   └── h7-favicon.svg
│   ├── src/
│   │   ├── components/
│   │   │   ├── admin/
│   │   │   ├── common/
│   │   │   ├── Chatbot.tsx
│   │   │   └── Profile.tsx
│   │   ├── pages/
│   │   ├── services/
│   │   │   └── api.ts
│   │   ├── types/
│   │   ├── utils/
│   │   │   └── media.ts
│   │   ├── App.css
│   │   ├── App.tsx
│   │   └── main.tsx
│   ├── index.html
│   ├── package.json
│   └── README.md
├── data/
│   ├── profile/
│   └── assets/
├── Dockerfile
├── Cargo.toml
└── README.md
```

---

## Public Portfolio Sections

The public portfolio currently includes:

- Home / Profile
- Projects
- Skills
- Certificates
- Education
- Social Links
- CV link
- H7 Assistant

---

## Admin Panel

The admin panel is intended only for the portfolio owner.

Admin capabilities include:

- Update basic profile details
- Manage profile image path
- Add, update, delete, and reorder projects
- Add project images and media paths
- Update project links and chatbot-only project details
- Add, update, delete, and reorder certificates
- Add, update, delete, and reorder education entries
- Update skills and focus areas
- Update contact and social links
- Save changes to JSON profile storage
- Upload project images and videos

The admin panel is not linked from the public portfolio UI. It is available through a private route and protected with authentication.

Admin sessions expire after the configured 5-60 minute lifetime; the default is one hour.

---

## H7 Assistant

H7 Assistant is a local portfolio-specific chatbot.

It can answer about:

- Profile summary
- Projects
- Project details
- Skills
- Certificates
- Education
- Focus areas
- Contact and social links

It is designed to answer only from the portfolio data. It does not act as a general-purpose chatbot and does not use external AI inference APIs.

This avoids monthly inference credit issues and keeps chatbot responses fast, stable, and fully based on the portfolio JSON data.

---

## API Routes

Main public API routes:

```text
GET  /health
GET  /health/ready
GET  /api/profile
POST /api/chat
```

Main protected admin API routes:

```text
POST /api/admin/login
GET  /api/admin/verify
GET  /api/admin/profile
PUT  /api/admin/profile
POST /api/admin/media/upload
```

`GET /api/profile` returns an `ETag` and supports `If-None-Match`. Admin profile reads also return an `ETag`; API clients should send it in `If-Match` when updating to prevent a stale editor from overwriting a newer change.

Interactive Swagger UI is intentionally not bundled. The OpenAPI JSON document is available only when `ENABLE_API_DOCS=true`:

```text
GET /api-docs/openapi.json
```

---

## Security Notes

This project includes security hardening for a public portfolio deployment.

Backend protections include:

- Strict startup validation for authentication and storage configuration; the documented password and session-secret placeholders are deliberately rejected
- Restricted CORS origins with exact origin values
- Route-specific body limits, request timeouts, and concurrency limits, including a four-request login concurrency cap and a 1 MiB total serialized-profile limit
- Protected admin routes with non-cacheable responses
- HS256 bearer tokens bound to issuer, audience, subject, type, version, issue time, not-before time, expiry, and unique token ID
- Five-minute to one-hour configurable admin sessions; increasing `ADMIN_SESSION_VERSION` revokes existing tokens
- Preferred bounded Argon2id v19 password hashes, with constant-time plaintext verification retained only for compatibility
- Bounded per-client and global limits for chat, login, and protected admin APIs, including `Retry-After`
- Proxy IP headers ignored by default and only one explicitly configured header trusted when enabled
- Exactly one upload per request; filenames are randomized, and file extension, declared MIME type, and byte signature must all match
- 25 MiB per-file upload limit, bounded total media quota, staging outside the public media tree, interrupted-staging cleanup at startup, and atomic publish
- Validated profile payloads, cached reads, serialized writes, ETag conflict protection, atomic replacement, and last-known-good recovery
- Stable JSON API errors that do not expose internal error details
- Request IDs, structured HTTP tracing, graceful shutdown, and explicit JSON `404` responses for unknown API routes
- `nosniff`, referrer, permissions, and restrictive public-media response headers

Frontend protections include:

- Static-host security headers when the optional Cloudflare Pages deployment is used
- Content Security Policy
- Frame policy matched to the host: the integrated Space build permits only self/Hugging Face ancestors so the Space can render normally, while the optional standalone Cloudflare build blocks framing
- Referrer policy
- Browser permission restrictions
- SPA redirect support
- Friendly API error handling
- Friendly rate-limit response handling

Secrets must never be committed to GitHub or exposed in frontend code. The `/media` tree is intentionally public: upload only portfolio assets, never private documents, credentials, or media containing sensitive metadata. Remove EXIF or other embedded metadata before upload when confidentiality matters.

---

## Environment Variables

Create a `.env` file for local development if needed.

```env
PORT=7860
RUST_LOG=backend=info,tower_http=info

ADMIN_EMAIL=your_admin_email
# Preferred: a valid Argon2id PHC string. Leave ADMIN_PASSWORD unset when this is used.
ADMIN_PASSWORD_HASH=$argon2id$...
# Compatibility fallback only; minimum 12 bytes and rejected if weak or email-derived.
# ADMIN_PASSWORD=replace_with_a_strong_password
ADMIN_SESSION_SECRET=minimum_32_characters_long_secret_value
ADMIN_SESSION_ISSUER=portfolio-backend
ADMIN_SESSION_AUDIENCE=portfolio-admin
ADMIN_SESSION_VERSION=1
ADMIN_SESSION_SECONDS=3600

ALLOWED_ORIGINS=http://localhost:5173,http://localhost:7860
TRUST_PROXY_HEADERS=false
TRUSTED_PROXY_HEADER=x-forwarded-for

PORTFOLIO_DATA_DIR=./data
PORTFOLIO_PROFILE_JSON=
MAX_MEDIA_STORAGE_BYTES=536870912
ENABLE_API_DOCS=false
```

Do not commit real secrets to GitHub.

Notes:

- The placeholder values shown in `.env.example` are documentation only and intentionally make startup fail until replaced.
- `ADMIN_PASSWORD_HASH` must be an Argon2id v19 PHC string. Accepted bounds are 19,456-65,536 KiB memory, 2-4 iterations, parallelism 1-4, a 16-64 byte output, and at most 256 MiB of memory-times-iterations work. Generate it with a trusted Argon2id tool and keep the plaintext password out of shell history and source control.
- `ADMIN_PASSWORD` is a temporary compatibility fallback only. Do not configure both password forms.
- `ADMIN_SESSION_SECRET` must be a unique value between 32 and 4096 bytes. `ADMIN_SESSION_SECONDS` must be between `300` and `3600`.
- Change `ADMIN_SESSION_VERSION` to invalidate all previously issued admin tokens.
- `ALLOWED_ORIGINS` is a comma-separated exact allowlist. Do not use `*` for the admin API.
- Keep `TRUST_PROXY_HEADERS=false` unless requests can reach the application only through a trusted reverse proxy. When enabled, `TRUSTED_PROXY_HEADER` must be exactly `x-forwarded-for`, `cf-connecting-ip`, or `x-real-ip`; choose the header set by that proxy.
- `PORTFOLIO_DATA_DIR` explicitly selects the persistent storage root. Without it, the backend uses `/data` when present and `./data` otherwise.
- `MAX_MEDIA_STORAGE_BYTES` must be between 25 MiB and 10 GiB; the default is 512 MiB.
- `PORTFOLIO_PROFILE_JSON` is an optional first-start seed, not a live database.
- Keep `ENABLE_API_DOCS=false` in production unless the schema is intentionally exposed.
- `HF_API_TOKEN` and `HF_MODEL_ID` are not required for the local H7 Assistant.
- Secret values must stay in `.env`, Hugging Face Space secrets, GitHub secrets, or hosting platform secret managers.

---

## Storage Flow

The backend loads and validates the profile once at startup, keeps an in-memory read snapshot, and serializes updates through a single write path. Each successful update is synced to a unique temporary file and atomically promoted. The previous valid profile is retained as `portfolio_profile.backup.json`; a corrupt primary file is restored from that last-known-good backup at startup.

Set the storage root explicitly with `PORTFOLIO_DATA_DIR`. A profile is limited to 1 MiB after serialization. The automatic filesystem fallback is:

In production on Hugging Face Spaces:

```text
/data
```

In local development:

```text
./data
```

For durable writes on Hugging Face Spaces, provision persistent Space storage and use:

```text
PORTFOLIO_DATA_DIR=/data
```

The backend reads and writes a normal filesystem; it does not connect to Hugging Face Storage Buckets through the S3 API. A Storage Bucket is suitable only if the deployment separately mounts or synchronizes it into the configured directory. Free/ephemeral Space files can disappear after a restart or rebuild, so `PORTFOLIO_PROFILE_JSON` can restore an initial profile but cannot preserve later admin edits or uploads. Use persistent Space storage or an externally managed filesystem mount for those writes.

If profile data is not found in storage, the backend can optionally load initial profile data from:

```text
PORTFOLIO_PROFILE_JSON
```

Uploaded assets are served publicly under `/media`. Visibility flags protect profile metadata and chatbot answers; they do not make an already uploaded media URL private. Upload processing does not strip EXIF or other embedded metadata.

---

## Frontend Deployment

The default Docker build compiles `frontend/` and copies `frontend/dist` into the final image. Axum serves the SPA and API together, so no separate frontend host or cross-origin API URL is required.

Cloudflare Pages is still supported as an optional split deployment:

Recommended Cloudflare Pages settings:

```text
Root directory: frontend
Framework preset: React / Vite
Build command: npm run build
Build output directory: dist
```

Required variable only for the optional split deployment:

```text
VITE_API_BASE_URL=https://hashan-7-chamira-hashan.hf.space
```

Cloudflare Pages support files:

```text
frontend/public/_redirects
frontend/public/_headers
```

`_redirects` supports React single-page application routing.

```text
/* /index.html 200
```

`_headers` adds browser security headers for the deployed frontend.

---

## Backend Deployment

The full application is deployed on Hugging Face Spaces using Docker. The multi-stage image uses pinned Node and Rust toolchains, cached dependency layers, a small Debian runtime, and a non-root runtime user.

The backend server handles:

- API routes
- Admin routes
- Compiled React SPA and client-side route fallback
- Media files
- Optional OpenAPI JSON
- JSON profile storage access
- Persistent storage access through `/data`

The backend listens on the configured `PORT`.

```text
PORT=7860
```

The Hugging Face Space should keep these secrets configured:

```text
ADMIN_EMAIL
ADMIN_PASSWORD_HASH
ADMIN_SESSION_SECRET
```

Keep `ADMIN_PASSWORD` only when temporarily using the compatibility path. Configure session claims, allowed origins, storage, and proxy trust from `.env.example`. Enable forwarded-header trust only after confirming which header the deployment edge sets; never enable it when clients can bypass that proxy.

When persistent Space storage is provisioned, configure the backend filesystem root as:

```text
/data
```

`.github/workflows/deploy.yml` gates deployment on Rust formatting/tests/Clippy/audit, a high-severity production npm audit, the frontend build, and the production container build. Frontend lint remains visible but nonblocking because this release intentionally leaves the existing frontend source untouched. Pushes to `main` deploy a clean source snapshot without repository history. Store `HF_TOKEN` as a GitHub Actions secret; it is a deployment credential, not a frontend or Space runtime variable.

---

## Local Development

### 1. Install frontend dependencies

```bash
cd frontend
npm ci
```

### 2. Build frontend

```bash
npm run build
```

### 3. Run backend

From the project root:

```bash
cargo run -p backend
```

### 4. Open site

```text
http://localhost:7860
```

Health check:

```text
http://localhost:7860/health
http://localhost:7860/health/ready
```

Optional OpenAPI JSON after setting `ENABLE_API_DOCS=true`:

```text
http://localhost:7860/api-docs/openapi.json
```

Admin panel:

```text
http://localhost:7860/h7-admin
```

---

## Build Checks

Before committing major changes, run:

```bash
cargo fmt --all -- --check
cargo test --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

Then verify the frontend and production image:

```bash
cd frontend
npm ci --no-audit --no-fund
npm audit --omit=dev --audit-level=high
npm run lint || true
npm run build
cd ..
docker build --pull --tag portfolio-backend:local .
```

Run the Rust dependency audit when `cargo-audit` is installed:

```bash
cargo audit
```

---

## Recommended Git Workflow

Work on `dev` first:

```bash
git checkout dev
git pull origin dev
git merge origin/main
git add .
git commit -m "Update portfolio project"
git push origin dev
```

For final release:

```bash
git checkout main
git pull origin main
git merge dev
git push origin main
```

Sync `dev` back with `main`:

```bash
git checkout dev
git pull origin dev
git merge origin/main
git push origin dev
```

---

## Production Test Checklist

Before sharing the portfolio link publicly, confirm:

- Custom domain loads correctly
- Public page loads correctly
- Mobile home layout is clean
- Menu active section highlight works
- Project carousel works
- Project images are displayed correctly
- Project videos are displayed correctly
- Skills ticker works
- Certificates display correctly
- Education section is clean
- Social icons open the correct links
- CV link works
- H7 Assistant opens and closes correctly
- H7 Assistant answers from local portfolio data
- H7 Assistant does not reveal hidden/admin/internal fields
- Admin route is not visible publicly
- Admin login works
- Admin session expires at the configured lifetime
- Admin save/update works
- Media upload works
- Public profile revalidation returns `304` for a matching `ETag`
- A stale admin update with `If-Match` is rejected instead of overwriting newer data
- `/projects` route works after refresh
- `/h7-admin` route works after refresh
- Hugging Face Docker build and deployment succeed
- Both `/health` and `/health/ready` succeed
- Optional Cloudflare Pages deployment succeeds, when used
- No frontend console errors are shown
- No broken public links are shown
- No real secrets are committed
- No private values are exposed in frontend code

---

## Security Checklist

Before final deployment, confirm:

- `.env` is not committed
- Admin Argon2id password hash is stored only in secrets
- Admin session secret is stored only in secrets
- `ADMIN_SESSION_SECRET` is unique and has at least 32 bytes
- Session issuer, audience, lifetime, and version are intentional
- CORS allowed origins are exact and restricted
- `VITE_API_BASE_URL` is empty for same-origin deployment, or points to the backend only in split deployment
- Persistent Space storage or another durable filesystem is mounted at `/data`
- `PORTFOLIO_DATA_DIR=/data` is configured for production
- Admin route is protected
- API rate limiting is enabled
- Proxy headers remain untrusted unless the application is isolated behind the configured proxy
- Frontend handles `429 Too Many Requests`
- Production response security headers are present
- TLS is enabled on the public domain
- Public portfolio data is safe to show
- Public `/media` files contain no confidential content or unwanted embedded metadata

---

## License

This project is created as a personal portfolio system for Chamira Hashan.

<div align="center">

**Developed by 💜 h7**

</div>
