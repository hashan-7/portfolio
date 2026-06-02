# Hashan Portfolio Frontend

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
- API error handling for authentication and rate-limit responses
- Cloudflare Pages static hosting support

The frontend does not store private secrets. Admin credentials, session secrets, profile storage, media upload handling, and protected API logic are managed by the backend.

---

## Current Production Setup

The current production setup uses separate hosting for the frontend and backend.

```text
Cloudflare Pages Frontend
        ↓
Hugging Face Space Rust Backend
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

---

## Main Files

```text
frontend/
├── public/
│   ├── _headers
│   ├── _redirects
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
│   │   └── media.ts
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

The admin UI depends on backend authentication and protected API routes. Admin tokens are stored locally in the browser and expire according to the backend session rules.

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
- Friendly API error messages
- Friendly rate-limit response handling

---

## API Integration

The frontend API logic is handled in:

```text
src/services/api.ts
```

The API base URL is selected from:

```text
VITE_API_BASE_URL
```

For local development, if `VITE_API_BASE_URL` is not set, the frontend uses:

```text
http://localhost:7860
```

For production, set:

```text
VITE_API_BASE_URL=https://hashan-7-chamira-hashan.hf.space
```

---

## Cloudflare Pages Support

The frontend includes Cloudflare Pages support files inside `public/`.

```text
public/_redirects
```

Used for single-page application routing.

```text
/* /index.html 200
```

```text
public/_headers
```

Used for frontend security headers, including content security policy, frame blocking, referrer policy, and browser permission restrictions.

These files are copied into the Vite production build output during `npm run build`.

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

## Cloudflare Pages Build Settings

Recommended Cloudflare Pages settings for this frontend:

```text
Framework preset: React / Vite
Build command: npm run build
Build output directory: dist
Root directory: frontend
```

Required production environment variable:

```text
VITE_API_BASE_URL=https://hashan-7-chamira-hashan.hf.space
```

---

## Backend API

During local development, the frontend can call the backend through:

```text
http://localhost:7860
```

The production frontend calls the deployed backend API on Hugging Face Space.

Main API routes used by the frontend:

```text
GET  /api/profile
POST /api/chat
POST /api/admin/login
GET  /api/admin/verify
GET  /api/admin/profile
PUT  /api/admin/profile
POST /api/admin/media/upload
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
- Bucket credentials
- Personal access tokens

Secrets must stay in backend environment variables, Hugging Face Space secrets, or the hosting platform secret manager.

Only public frontend configuration values should use `VITE_` variables.

---

## Security Notes

The frontend includes browser-side hardening through Cloudflare Pages headers.

The backend is responsible for:

- Admin authentication
- Session token validation
- Protected admin routes
- Media upload validation
- Request body limits
- API rate limiting
- Storage access
- Secret handling

The frontend handles user-friendly API errors, including expired admin sessions and rate-limit responses.

---

## Build Check

Before committing frontend changes, run:

```bash
npm run build
```

For Cloudflare Pages, `public/_redirects` and `public/_headers` are copied into the production build output.

To test the full app locally through the Rust backend:

```bash
cd ..
cargo run -p backend
```

Open:

```text
http://localhost:7860
```

---

## Production Test Checklist

After deployment, check:

- Home page loads correctly
- Project images and videos load correctly
- `/projects` route works after refresh
- `/h7-admin` route works after refresh
- H7 Assistant opens and replies
- Admin login works
- Admin session expiry works
- Admin profile loading works
- Admin save/update works
- Media upload works
- No frontend console errors
- No private secrets appear in the frontend bundle

---

## Notes

This README is for frontend documentation only. Updating this file does not affect the application build or runtime behavior.