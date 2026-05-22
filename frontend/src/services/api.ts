import type { ChatMessage, Profile } from '../types';

const API_BASE_URL =
  import.meta.env.VITE_API_BASE_URL ?? (import.meta.env.DEV ? 'http://localhost:7860' : '');

export async function getProfile(): Promise<Profile> {
  const response = await fetch(`${API_BASE_URL}/api/profile`);

  if (!response.ok) {
    throw new Error(`Failed to load profile data. Status: ${response.status}`);
  }

  return response.json();
}

export async function sendChatMessage(history: ChatMessage[]): Promise<string> {
  const response = await fetch(`${API_BASE_URL}/api/chat`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ history }),
  });

  if (!response.ok) {
    throw new Error(`Failed to send chat message. Status: ${response.status}`);
  }

  const data = (await response.json()) as { reply?: string };

  return data.reply ?? 'No reply received from the AI assistant.';
}