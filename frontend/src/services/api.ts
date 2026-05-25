import type { ChatMessage, FullProfile, PublicProfile } from '../types';

const API_BASE_URL =
  import.meta.env.VITE_API_BASE_URL ?? (import.meta.env.DEV ? 'http://localhost:7860' : '');

const ADMIN_TOKEN_KEY = 'admin_token';
const ADMIN_TOKEN_EXPIRES_AT_KEY = 'admin_token_expires_at';

interface AdminLoginResponse {
  token?: string;
  expires_at?: number;
  expires_in_seconds?: number;
}

function getStoredAdminExpiryMs(): number | null {
  const value = localStorage.getItem(ADMIN_TOKEN_EXPIRES_AT_KEY);

  if (!value) {
    return null;
  }

  const parsedValue = Number(value);

  if (!Number.isFinite(parsedValue) || parsedValue <= 0) {
    return null;
  }

  return parsedValue;
}

function clearAdminToken(): void {
  localStorage.removeItem(ADMIN_TOKEN_KEY);
  localStorage.removeItem(ADMIN_TOKEN_EXPIRES_AT_KEY);
}

function getAdminToken(): string | null {
  const token = localStorage.getItem(ADMIN_TOKEN_KEY);

  if (!token) {
    return null;
  }

  const expiresAtMs = getStoredAdminExpiryMs();

  if (expiresAtMs !== null && Date.now() >= expiresAtMs) {
    clearAdminToken();
    return null;
  }

  return token;
}

function getAuthHeaders() {
  const token = getAdminToken();

  return {
    'Content-Type': 'application/json',
    ...(token ? { Authorization: `Bearer ${token}` } : {}),
  };
}

export function logoutAdmin(): void {
  clearAdminToken();
}

export function getAdminSessionRemainingMs(): number | null {
  const token = localStorage.getItem(ADMIN_TOKEN_KEY);
  const expiresAtMs = getStoredAdminExpiryMs();

  if (!token || expiresAtMs === null) {
    return null;
  }

  return expiresAtMs - Date.now();
}

export async function getProfile(): Promise<PublicProfile> {
  const response = await fetch(`${API_BASE_URL}/api/profile`);

  if (!response.ok) {
    throw new Error(`Failed to load profile data. Status: ${response.status}`);
  }

  return response.json();
}

export async function sendChatMessage(history: ChatMessage[], scope = 'all'): Promise<string> {
  const response = await fetch(`${API_BASE_URL}/api/chat`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ history, scope }),
  });

  if (!response.ok) {
    throw new Error(`Failed to send chat message. Status: ${response.status}`);
  }

  const data = (await response.json()) as { reply?: string };
  return data.reply ?? 'No reply received from H7 Assistant.';
}

export async function loginAdmin(email: string, password: string): Promise<string> {
  const response = await fetch(`${API_BASE_URL}/api/admin/login`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ email, password }),
  });

  if (!response.ok) {
    clearAdminToken();
    throw new Error('Invalid admin email or password.');
  }

  const data = (await response.json()) as AdminLoginResponse;

  if (!data.token) {
    clearAdminToken();
    throw new Error('Admin token was not received.');
  }

  const fallbackExpiresAtMs = Date.now() + 60 * 60 * 1000;
  const expiresAtMs = data.expires_at ? data.expires_at * 1000 : fallbackExpiresAtMs;

  localStorage.setItem(ADMIN_TOKEN_KEY, data.token);
  localStorage.setItem(ADMIN_TOKEN_EXPIRES_AT_KEY, String(expiresAtMs));

  return data.token;
}

export async function verifyAdminSession(): Promise<boolean> {
  const token = getAdminToken();

  if (!token) {
    clearAdminToken();
    return false;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/api/admin/verify`, {
      headers: getAuthHeaders(),
    });

    if (!response.ok) {
      clearAdminToken();
      return false;
    }

    return true;
  } catch {
    clearAdminToken();
    return false;
  }
}

export async function getAdminProfile(): Promise<FullProfile> {
  const response = await fetch(`${API_BASE_URL}/api/admin/profile`, {
    headers: getAuthHeaders(),
  });

  if (response.status === 401) {
    clearAdminToken();
    throw new Error('Admin session expired. Please sign in again.');
  }

  if (!response.ok) {
    throw new Error(`Failed to load admin profile. Status: ${response.status}`);
  }

  return response.json();
}

export async function updateAdminProfile(profile: FullProfile): Promise<void> {
  const response = await fetch(`${API_BASE_URL}/api/admin/profile`, {
    method: 'PUT',
    headers: getAuthHeaders(),
    body: JSON.stringify(profile),
  });

  if (response.status === 401) {
    clearAdminToken();
    throw new Error('Admin session expired. Please sign in again.');
  }

  if (!response.ok) {
    throw new Error(`Failed to update profile. Status: ${response.status}`);
  }
}

export async function uploadMedia(file: File): Promise<string[]> {
  const token = getAdminToken();
  const formData = new FormData();

  formData.append('file', file);

  const response = await fetch(`${API_BASE_URL}/api/admin/media/upload`, {
    method: 'POST',
    headers: {
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    body: formData,
  });

  if (response.status === 401) {
    clearAdminToken();
    throw new Error('Admin session expired. Please sign in again.');
  }

  if (!response.ok) {
    throw new Error(`Failed to upload media. Status: ${response.status}`);
  }

  const data = (await response.json()) as { files?: string[] };
  return data.files ?? [];
}