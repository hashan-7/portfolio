import type { ChatMessage, FullProfile, PublicProfile } from '../types';

const API_BASE_URL =
  import.meta.env.VITE_API_BASE_URL ?? (import.meta.env.DEV ? 'http://localhost:7860' : '');

function getAdminToken(): string | null {
  return localStorage.getItem('admin_token');
}

function getAuthHeaders() {
  const token = getAdminToken();

  return {
    'Content-Type': 'application/json',
    ...(token ? { Authorization: `Bearer ${token}` } : {}),
  };
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
    throw new Error('Invalid admin email or password.');
  }

  const data = (await response.json()) as { token?: string };

  if (!data.token) {
    throw new Error('Admin token was not received.');
  }

  localStorage.setItem('admin_token', data.token);
  return data.token;
}

export async function verifyAdminSession(): Promise<boolean> {
  const token = getAdminToken();

  if (!token) {
    return false;
  }

  const response = await fetch(`${API_BASE_URL}/api/admin/verify`, {
    headers: getAuthHeaders(),
  });

  return response.ok;
}

export async function getAdminProfile(): Promise<FullProfile> {
  const response = await fetch(`${API_BASE_URL}/api/admin/profile`, {
    headers: getAuthHeaders(),
  });

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

  if (!response.ok) {
    throw new Error(`Failed to upload media. Status: ${response.status}`);
  }

  const data = (await response.json()) as { files?: string[] };
  return data.files ?? [];
}
