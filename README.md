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

The project includes a public portfolio website, a protected admin panel for managing portfolio data, media upload support, and a local H7 Assistant chatbot that answers from the portfolio JSON data without using external AI inference credits.

---

## Live Portfolio

```text
https://hashan-7-chamira-hashan.hf.space
```

---

## Overview

This portfolio is designed as a dynamic portfolio system instead of a static hardcoded website.

The public website renders profile details, projects, skills, certificates, education, social links, and CV access from backend-managed JSON data. The admin panel allows the portfolio owner to update profile content, project details, certificates, education entries, links, and media paths without editing frontend source code.

The H7 Assistant is a local portfolio assistant. It answers common portfolio questions using the stored profile data, such as project details, skills, certificates, education, contact links, and overall profile summary. It does not call Hugging Face Router or external AI inference providers.

---

## Key Features

- Premium dark purple responsive portfolio UI
- Mobile-first home section layout
- Dynamic profile, project, skills, certificates, and education rendering
- Project carousel with typed description animation
- Project image and video preview support
- Certificate flip-book style presentation
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
- Docker-based Hugging Face Spaces deployment

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

### Frontend

- React
- TypeScript
- Vite
- CSS
- Responsive UI design

### Chatbot

- Local JSON-based portfolio assistant
- No external AI model call required
- No Hugging Face Inference Provider credit usage
- Portfolio-data-only answers
- Simple guardrails for out-of-scope questions

### Storage

- JSON profile data
- Local `/data` storage flow
- Hugging Face Spaces persistent storage compatible structure
- Media folders for uploaded images and videos

---

## Project Structure

```text
portfolio/
├── backend/
│   └── src/
│       ├── admin.rs
│       ├── auth.rs
│       ├── media.rs
│       ├── portfolio_bot.rs
│       ├── profile.rs
│       ├── routes.rs
│       └── storage.rs
├── frontend/
│   └── src/
│       ├── components/
│       │   ├── admin/
│       │   ├── common/
│       │   ├── Chatbot.tsx
│       │   └── Profile.tsx
│       ├── pages/
│       ├── services/
│       ├── types/
│       ├── utils/
│       ├── App.css
│       └── App.tsx
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

## Environment Variables

Create a `.env` file for local development if needed.

```env
PORT=7860

ADMIN_EMAIL=your_admin_email
ADMIN_PASSWORD=your_admin_password
ADMIN_SESSION_SECRET=minimum_32_characters_long_secret_value

PORTFOLIO_PROFILE_JSON=optional_profile_json_seed
```

Do not commit real secrets to GitHub.

Notes:

- `ADMIN_SESSION_SECRET` must be at least 32 characters.
- `PORTFOLIO_PROFILE_JSON` is optional.
- `HF_API_TOKEN` and `HF_MODEL_ID` are not required for the local H7 Assistant.

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
cargo run -p backend
```

---

## Deployment Notes

This project is designed to run on Hugging Face Spaces using Docker.

The backend server serves:

- API routes
- Admin routes
- Media files
- Built frontend files from `frontend/dist`

For deployment:

1. Build the frontend.
2. Build the Rust backend.
3. Run the backend on the expected `PORT`.
4. Configure admin secrets in the hosting platform.
5. Ensure profile JSON and media storage paths are available.
6. Test the public `.hf.space` link on desktop, mobile, and incognito mode.

---

## Recommended Git Workflow

```bash
git checkout dev
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

---

## Quality Checklist

Before sharing the portfolio link publicly, confirm:

- Public page loads correctly
- Mobile home layout is clean
- Menu active section highlight works
- Project carousel works
- Project images are displayed correctly
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
- No broken public links are shown
- No real secrets are committed

---

## License

This project is created as a personal portfolio system for Chamira Hashan.

<div align="center">

**Developed by 💜 h7**

</div>
