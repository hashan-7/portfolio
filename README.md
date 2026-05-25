---
title: Chamira Hashan Portfolio
emoji: 💜
colorFrom: purple
colorTo: pink
sdk: docker
pinned: false
short_description: Full-stack personal portfolio with Rust, React, admin-managed data, and H7 Assistant.
---

# Hashan Personal Portfolio

A professional personal portfolio website for **Chamira Hashan**, built to showcase practical backend, AI/ML, full-stack, mobile, and software engineering projects.

This portfolio includes a public portfolio interface, an internal admin panel for managing profile data, and an H7 Assistant chatbot that answers only from the provided portfolio information.

---

## Overview

H7 Personal Portfolio is a full-stack portfolio system designed with a clean premium UI, secure admin editing flow, dynamic JSON-based profile data, and a portfolio-specific assistant.

The public website displays profile information, projects, skills, certificates, education, social links, and contact details. The admin panel allows controlled updates to portfolio data without hardcoding public content inside frontend components.

---

## Key Features

- Premium dark purple responsive portfolio UI
- Mobile, tablet, and desktop compatible layout
- Dynamic profile, skills, projects, certificates, and education rendering
- Project carousel with typed description animation
- Project image support with controlled preview sizing
- Certificate flip-book style presentation
- Skills ticker animation
- Profile image support through admin upload flow
- Floating H7 Assistant chatbot popup
- Draggable chatbot window using title area only
- Chatbot typing animation for assistant replies
- Admin panel for updating portfolio JSON data
- Hidden admin route with authentication requirement
- Media upload support for project/profile assets
- Rust Axum backend with Vite React frontend
- Hugging Face Spaces compatible deployment flow

---

## Tech Stack

### Backend

- Rust
- Axum
- Tokio
- Serde
- Reqwest
- Tower HTTP
- Utoipa / Swagger UI

### Frontend

- React
- TypeScript
- Vite
- CSS
- Responsive UI design

### AI / Chatbot

- Hugging Face Router compatible chat completion flow
- Portfolio-context-only assistant behavior
- Local rule-based fallback/safety handling where configured

### Storage

- JSON profile data
- Local `/data` storage flow
- Hugging Face Spaces persistent storage compatible structure

---

## Project Structure

```text
portfolio/
├── backend/
│   └── src/
│       ├── admin.rs
│       ├── auth.rs
│       ├── media.rs
│       ├── profile.rs
│       ├── routes.rs
│       ├── safety.rs
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
└── README.md
```

---

## Public Portfolio Sections

The portfolio currently supports:

- Home / Profile
- Projects
- Skills
- Certificates
- Education
- Contact / Social Links
- H7 Assistant

---

## Admin Panel

The admin panel is intended for portfolio owner usage only.

Admin capabilities include:

- Update basic profile details
- Upload and manage profile image
- Add, update, delete, and reorder projects
- Add project images
- Update project links and chatbot-only notes
- Add, update, delete, and reorder certificates
- Add, update, delete, and reorder education entries
- Update skills and focus areas
- Update contact and social links
- Save changes to JSON profile storage

The admin panel should not be linked visibly from the public portfolio. It should only be accessed through its private route and must remain protected by authentication.

---

## H7 Assistant

H7 Assistant is a portfolio-specific chatbot.

It is designed to:

- Answer only using the provided portfolio information
- Explain projects, skills, certificates, education, and profile details
- Avoid unrelated general-purpose chatbot behavior
- Refuse or redirect questions when information is not available
- Keep hidden/admin/internal data private
- Present answers cleanly without unnecessary markdown symbols in UI

---

## Environment Variables

Create a `.env` file for local development if needed.

```env
PORT=7860
HF_API_TOKEN=your_hugging_face_token
HF_MODEL_ID=your_model_id
PORTFOLIO_PROFILE_JSON=your_profile_json_if_not_using_data_file
ADMIN_USERNAME=your_admin_username
ADMIN_PASSWORD=your_admin_password
JWT_SECRET=your_jwt_secret
```

Do not commit real secrets to GitHub.

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

---

## Deployment Notes

This project is designed to work on Hugging Face Spaces using the backend server to serve both the API and the built frontend.

For deployment:

1. Build the frontend.
2. Run the Rust backend on the expected port.
3. Configure required secrets in the hosting platform.
4. Ensure profile JSON and media storage paths are correctly available.
5. Test the public `.hf.space` link in desktop, mobile, and incognito mode.

---

## Portfolio Link

Add the final deployed portfolio link here:

```text
https://hashan-7-chamira-hashan.hf.space
```

---

## Recommended Git Workflow

```bash
git checkout dev
git add .
git commit -m "Polish portfolio UI and admin data flow"
git push origin dev
```

For final release:

```bash
git checkout main
git merge dev
git push origin main
```

---

## Quality Checklist

Before sharing the portfolio link publicly, confirm:

- Public page loads correctly
- Mobile layout is clean
- Menu active section highlight works
- Project carousel works
- Project images are not stretched badly
- Certificates display correctly
- Education section is clean
- Social links open correctly
- CV link works
- H7 Assistant opens and closes correctly
- Chatbot does not reveal hidden/admin/internal notes
- Admin route is not visible publicly
- Admin login works
- Profile image upload works
- No broken links are shown publicly
- No real secrets are committed

---

## License

This project is created as a personal portfolio system for Chamira Hashan.

<div align="center">

**Developed by 💜 h7**  

</div>
