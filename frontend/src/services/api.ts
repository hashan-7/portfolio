import type { Profile } from '../types';

const API_BASE_URL =
  import.meta.env.VITE_API_BASE_URL ?? (import.meta.env.DEV ? 'http://localhost:7860' : '');

export async function getProfile(): Promise<Profile> {
  const response = await fetch(`${API_BASE_URL}/api/profile`);

  if (!response.ok) {
    throw new Error(`Failed to load profile data. Status: ${response.status}`);
  }

  return response.json();
}