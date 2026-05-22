---
title: AI Portfolio
emoji: 🚀
colorFrom: blue
colorTo: indigo
sdk: docker
app_port: 7860
---

# AI Portfolio

AI-powered personal portfolio built with Rust, React, Docker, and Hugging Face.

## Current Features

- React + TypeScript portfolio frontend
- Rust Axum backend API
- Public profile data endpoint
- AI chatbot endpoint powered by Hugging Face Router
- Session-only chat memory
- Portfolio-aware chatbot responses using JSON profile data
- Swagger UI for API testing
- Docker multi-stage build
- GitHub Actions deployment to Hugging Face Spaces

## API Routes

```text
GET  /health
GET  /api/profile
POST /api/chat
GET  /swagger-ui

```

## Tech Stack

```text
Frontend: React, TypeScript, Vite
Backend: Rust, Axum, Tokio
AI: Hugging Face Router
Docs: Swagger UI / utoipa
Deployment: Docker, GitHub Actions, Hugging Face Spaces
```

## Environment Variables

```text
PORT
HF_API_TOKEN
HF_MODEL_ID
PORTFOLIO_PROFILE_JSON
```

## Status

The backend, frontend, AI chatbot, Docker build, and Hugging Face deployment are currently working.