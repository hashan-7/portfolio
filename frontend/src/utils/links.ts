export function hasLink(value?: string): boolean {
  return Boolean(value && value.trim().length > 0);
}

export function formatExternalLink(value?: string): string | undefined {
  if (!value) return undefined;
  const trimmed = value.trim();
  if (!trimmed) return undefined;

  if (
    trimmed.startsWith('http://') ||
    trimmed.startsWith('https://') ||
    trimmed.startsWith('mailto:') ||
    trimmed.startsWith('tel:') ||
    trimmed.startsWith('/')
  ) {
    return trimmed;
  }

  return `https://${trimmed}`;
}

export function emailLink(email?: string): string | undefined {
  if (!email?.trim()) return undefined;
  return `mailto:${email.trim()}`;
}

export function phoneLink(phone?: string): string | undefined {
  if (!phone?.trim()) return undefined;
  return `tel:${phone.trim()}`;
}
