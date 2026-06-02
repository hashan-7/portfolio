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

The current production setup uses separate hosting for the frontend and backend.

```text
Cloudflare Pages Frontend
        ↓
Hugging Face Space Rust Backend
        ↓
Hugging Face Storage Bucket mounted at /data
```

The frontend is hosted on Cloudflare Pages and connected to the custom domain:

```text
https://chamirahashan.tech
```

The backend is hosted on Hugging Face Spaces:

```text
https://hashan-7-chamira-hashan.hf.space
```

The frontend calls the backend through:

```text
VITE_API_BASE_URL=https://hashan-7-chamira-hashan.hf.space
```

The Hugging Face Space keeps the backend public so the Cloudflare-hosted frontend can call the API from the browser.

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
- One-hour admin session expiry
- Admin-managed JSON profile data
- Media upload support for project/profile assets
- Rust Axum backend
- React TypeScript Vite frontend
- Cloudflare Pages frontend deployment
- Docker-based Hugging Face Spaces backend deployment
- Hugging Face Storage Bucket support
- API rate limiting
- CORS origin restriction
- Frontend security headers

---

## Tech Stack

### Backend

- Rust
- Axum
- Tokio
- Serde
- Serde JSON
- Tower HTTP
- Utoipa / Swagger UI
- JSON Web Token authentication
- Custom in-memory API rate limiting
- Hugging Face Space Docker runtime

### Frontend

- React
- TypeScript
- Vite
- CSS
- Responsive UI design
- Cloudflare Pages static hosting

### Chatbot

- Local JSON-based portfolio assistant
- No external AI model call required
- No Hugging Face Inference Provider credit usage
- Portfolio-data-only answers
- Simple guardrails for out-of-scope questions

### Storage

- JSON profile data
- Local `./data` fallback for development
- Hugging Face Storage Bucket mounted at `/data` in production
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
│       ├── media.rs
│       ├── portfolio_bot.rs
│       ├── profile.rs
│       ├── rate_limit.rs
│       ├── routes.rs
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

Admin sessions expire after one hour.

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

Swagger UI:

```text
/swagger-ui
```

---

## Security Notes

This project includes security hardening for a public portfolio deployment.

Backend protections include:

- Restricted CORS allowed origins
- Request body size limit
- Protected admin routes
- Bearer token authentication
- One-hour admin session expiry
- Admin session secret validation
- Media upload type validation
- Media upload size limit
- API rate limiting for chat, admin login, and protected admin routes
- Backend-only secret handling

Frontend protections include:

- Cloudflare Pages security headers
- Content Security Policy
- Frame blocking
- Referrer policy
- Browser permission restrictions
- SPA redirect support
- Friendly API error handling
- Friendly rate-limit response handling

Secrets must never be committed to GitHub or exposed in frontend code.

---

## Environment Variables

Create a `.env` file for local development if needed.

```env
PORT=7860

ADMIN_EMAIL=your_admin_email
ADMIN_PASSWORD=your_admin_password
ADMIN_SESSION_SECRET=minimum_32_characters_long_secret_value

PORTFOLIO_PROFILE_JSON=optional_profile_json_seed
ALLOWED_ORIGINS=http://localhost:5173,http://localhost:7860
```

Do not commit real secrets to GitHub.

Notes:

- `ADMIN_SESSION_SECRET` must be at least 32 characters.
- `PORTFOLIO_PROFILE_JSON` is optional.
- `ALLOWED_ORIGINS` is optional.
- `HF_API_TOKEN` and `HF_MODEL_ID` are not required for the local H7 Assistant.
- Secret values must stay in `.env`, Hugging Face Space secrets, GitHub secrets, or hosting platform secret managers.

---

## Storage Flow

The backend uses a storage root helper.

In production on Hugging Face Spaces:

```text
/data
```

In local development:

```text
./data
```

The production Hugging Face Space uses a mounted Storage Bucket:

```text
Bucket: hashan-7/ai-portfolio-assets
Mount path: /data
Access: Read & Write
```

This allows profile JSON and uploaded media files to persist beyond normal Space restarts.

If profile data is not found in storage, the backend can optionally load initial profile data from:

```text
PORTFOLIO_PROFILE_JSON
```

---

## Frontend Deployment

The current public frontend is deployed separately on Cloudflare Pages.

Recommended Cloudflare Pages settings:

```text
Root directory: frontend
Framework preset: React / Vite
Build command: npm run build
Build output directory: dist
```

Required production environment variable:

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

The backend is deployed on Hugging Face Spaces using Docker.

The backend server handles:

- API routes
- Admin routes
- Media files
- Swagger UI
- JSON profile storage access
- Hugging Face Storage Bucket access through `/data`

The backend listens on the configured `PORT`.

```text
PORT=7860
```

The Hugging Face Space should keep these secrets configured:

```text
ADMIN_EMAIL
ADMIN_PASSWORD
ADMIN_SESSION_SECRET
```

The Hugging Face Space should keep the Storage Bucket mounted at:

```text
/data
```

---

## Local Development

### 1. Install frontend dependencies

```bash
cd frontend
npm install
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
```

Swagger UI:

```text
http://localhost:7860/swagger-ui
```

Admin panel:

```text
http://localhost:7860/h7-admin
```

---

## Build Checks

Before committing major changes, run:

```bash
cargo fmt
cargo check -p backend
cd frontend
npm run build
```

Then return to the root directory and run:

```bash
cd ..
cargo run -p backend
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
- Admin session expires after one hour
- Admin save/update works
- Media upload works
- `/projects` route works after refresh
- `/h7-admin` route works after refresh
- Cloudflare Pages deployment succeeds
- Hugging Face Space backend health check works
- No frontend console errors are shown
- No broken public links are shown
- No real secrets are committed
- No private values are exposed in frontend code

---

## Security Checklist

Before final deployment, confirm:

- `.env` is not committed
- Admin password is stored only in secrets
- Admin session secret is stored only in secrets
- `ADMIN_SESSION_SECRET` has at least 32 characters
- CORS allowed origins are restricted
- Frontend `VITE_API_BASE_URL` points to the backend API
- Hugging Face Storage Bucket is mounted at `/data`
- Storage Bucket is not unmounted
- Admin route is protected
- API rate limiting is enabled
- Frontend handles `429 Too Many Requests`
- Frontend security headers are included
- Cloudflare SSL is enabled
- Public portfolio data is safe to show

---

## License

This project is created as a personal portfolio system for Chamira Hashan.

<div align="center">

**Developed by 💜 h7**

</div>