---
title: AI Portfolio
emoji: 🚀
colorFrom: blue
colorTo: indigo
sdk: docker
app_port: 7860
---

# AI Portfolio

AI-powered personal portfolio built with Rust, React, Docker, Hugging Face Router, and Hugging Face Spaces.

## Current Features

- React + TypeScript portfolio frontend
- Rust Axum backend API
- Public filtered profile endpoint
- AI chatbot endpoint powered by Hugging Face Router
- Session-only chat memory
- Portfolio-aware chatbot responses using JSON profile data
- Admin login with JWT authentication
- Protected admin profile read/update API
- Protected media upload endpoint
- Bucket-ready profile JSON storage
- Bucket-ready project image/video media serving
- Swagger UI for public API testing
- Docker multi-stage build
- GitHub Actions deployment to Hugging Face Spaces

## Public API Routes

```text
GET  /health
GET  /api/profile
POST /api/chat
GET  /swagger-ui
```

## Admin API Routes

```text
POST /api/admin/login
GET  /api/admin/verify
GET  /api/admin/profile
PUT  /api/admin/profile
POST /api/admin/media/upload
```

Admin routes require authentication except `/api/admin/login`.

## Media Route

```text
GET /media/*
```

Project images and videos are served through the backend from bucket/local storage.

## Tech Stack

```text
Frontend: React, TypeScript, Vite
Backend: Rust, Axum, Tokio
AI: Hugging Face Router
Docs: Swagger UI / utoipa
Storage: Hugging Face Storage Bucket-ready local file storage
Deployment: Docker, GitHub Actions, Hugging Face Spaces
```

## Environment Variables

```text
PORT
HF_API_TOKEN
HF_MODEL_ID
ADMIN_EMAIL
ADMIN_PASSWORD
ADMIN_SESSION_SECRET
PORTFOLIO_PROFILE_JSON
```

`PORTFOLIO_PROFILE_JSON` is used as a fallback seed when the persistent profile JSON file is not available.

## Storage

The app is designed to read and write profile/media data from persistent storage when mounted at:

```text
/data
```

Expected storage structure:

```text
/data/profile/portfolio_profile.json
/data/assets/projects/images/
/data/assets/projects/videos/
```

For local development, the app falls back to:

```text
./data
```

## Status

The backend, frontend, AI chatbot, admin authentication, media upload, Docker build, and Hugging Face deployment foundation are currently working.

