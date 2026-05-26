# H7 Portfolio Frontend

Frontend application for the Chamira Hashan personal portfolio.

This frontend is built with **React**, **TypeScript**, and **Vite**. It renders the public portfolio UI, admin interface, and H7 Assistant chatbot UI. Portfolio data is loaded from the Rust backend API.

---

## Overview

The frontend is responsible for:

- Public portfolio layout
- Mobile-first home section
- Project carousel
- Skills ticker
- Certificate display
- Education section
- Social links and CV button
- H7 Assistant popup UI
- Admin login page
- Admin dashboard forms

The frontend does not store private secrets. Admin credentials, tokens, profile JSON storage, and media upload handling are managed by the backend.

---

## Main Files

```text
frontend/
├── public/
│   └── h7-favicon.svg
├── src/
│   ├── components/
│   │   ├── admin/
│   │   ├── common/
│   │   ├── Chatbot.tsx
│   │   └── Profile.tsx
│   ├── pages/
│   ├── services/
│   │   └── api.ts
│   ├── types/
│   ├── utils/
│   ├── App.css
│   ├── App.tsx
│   └── main.tsx
├── index.html
├── package.json
└── README.md
```

---

## Public UI

The public UI displays:

- Home profile section
- Profile image
- CV button
- Project and tech counters
- Social icons
- Projects
- Skills
- Certificates
- Education
- H7 Assistant

The public UI receives only public-safe portfolio data from the backend.

---

## Admin UI

The admin UI is available through the private admin route configured in the main app.

The admin panel allows the portfolio owner to update:

- Basic profile details
- Contact and social links
- Skills
- Focus areas
- Projects
- Project images
- Certificates
- Education
- Full JSON profile data

The admin UI depends on backend authentication and protected API routes.

---

## H7 Assistant UI

The chatbot popup sends messages to the backend `/api/chat` endpoint.

The current assistant flow is local and portfolio-data-based. It does not require external AI tokens from the frontend.

The UI supports:

- Floating launcher
- Popup chat window
- Assistant and user messages
- Typing indicator
- Clickable link formatting
- Auto-scroll to latest message

---

## Development

Install dependencies:

```bash
npm install
```

Run the Vite development server:

```bash
npm run dev
```

Build for production:

```bash
npm run build
```

Preview production build:

```bash
npm run preview
```

---

## Backend API

During local development, the frontend can call the backend through:

```text
http://localhost:7860
```

The API base URL is configured in:

```text
src/services/api.ts
```

For production, the backend serves the built frontend from:

```text
frontend/dist
```

---

## Environment Notes

The frontend should not contain private secrets.

Do not place these values in frontend source code:

- Admin password
- Admin session secret
- API tokens
- Private profile data
- Hidden admin notes

Secrets must stay in backend environment variables or hosting platform secrets.

---

## Build Check

Before committing frontend changes, run:

```bash
npm run build
```

Then test the full app through the Rust backend:

```bash
cd ..
cargo run -p backend
```

Open:

```text
http://localhost:7860
```

---

## Notes

This README is for frontend documentation only. Updating this file does not affect the application build or runtime behavior.